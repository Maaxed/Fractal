use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode};

const SHADER_CODE: &[u8] = include_bytes!(env!("fractal_renderer_shader.spv"));

pub struct App
{
	target: crate::Target,
	render: crate::render::Render,
	compute: crate::compute::Compute,
}

impl App
{
	pub fn new(target: crate::Target) -> Self
	{
        let shader_module = target.device.create_shader_module(
            wgpu::ShaderModuleDescriptor
            {
                label: None,
                source: wgpu::util::make_spirv(SHADER_CODE),
            });
		
    	let size = 64;
    	let compute = crate::compute::Compute::new(&shader_module, &target, size);
		
		let render = crate::render::Render::new(&shader_module, &target, size);

		Self
		{
			target,
			render,
			compute,
		}
	}

	pub fn do_print_compute(&self)
	{
		let readback_buffer = self.target.device.create_buffer(
        &wgpu::BufferDescriptor
        {
            label: None,
            size: self.compute.buffer().size(),
            // Can be read to the CPU, and can be copied from the shader's storage buffer
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
    
		let mut commands = self.target.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

		self.compute.make_compute_pass(&mut commands);

		self.compute.copy_buffer(&mut commands, &readback_buffer, 0);
		
		self.target.queue.submit(std::iter::once(commands.finish()));

		let buffer_slice = readback_buffer.slice(..);
		buffer_slice.map_async(wgpu::MapMode::Read, |r| r.unwrap());
		self.target.device.poll(wgpu::Maintain::Wait);

		let result = self.compute.read_buffer(&buffer_slice.get_mapped_range());

		for line in result.chunks_exact(self.compute.size as usize)
		{
			for c in line
			{
				print!("{c:>3}");
			}
			println!();
		}
		
		readback_buffer.unmap();
	}

	pub fn do_compute(&self)
	{
		let mut commands = self.target.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

		self.compute.make_compute_pass(&mut commands);

		self.compute.copy_buffer_to_texture(&mut commands, &self.render.fractal_texture);
		
		self.target.queue.submit(std::iter::once(commands.finish()));
	}

	pub fn do_render(&self) -> Result<(), wgpu::SurfaceError>
	{
		let output = self.target.surface.get_current_texture()?;
		let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

		let mut commands = self.target.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

		self.render.make_render_pass(&view, &mut commands);

		self.target.queue.submit(std::iter::once(commands.finish()));
		output.present();

		Ok(())
	}

	pub fn run(mut self, event_loop: EventLoop<()>) -> !
	{
		self.do_compute();
		
		event_loop.run(move
			|event, _, control_flow|
			match event
			{
				Event::RedrawRequested(window_id) if window_id == self.target.window.id() =>
				{
					match self.do_render()
					{
						Ok(_) => {}
						// Reconfigure the surface if lost
						Err(wgpu::SurfaceError::Lost) => self.target.configure_surface(),
						// The system is out of memory, we should probably quit
						Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
						// All other errors (Outdated, Timeout) should be resolved by the next frame
						Err(e @ wgpu::SurfaceError::Outdated | e @ wgpu::SurfaceError::Timeout) => eprintln!("{:?}", e),
					}
				}
				Event::MainEventsCleared =>
				{
					// RedrawRequested will only trigger once, unless we manually request it.
					self.target.window.request_redraw();
				},
				Event::WindowEvent { window_id, ref event} if window_id == self.target.window.id() =>
				{
					match event
					{
						WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
						WindowEvent::Resized(physical_size) =>
						{
							self.target.resize(*physical_size);
						},
						WindowEvent::ScaleFactorChanged { new_inner_size, .. } =>
						{
							// new_inner_size is &&mut so we have to dereference it twice
							self.target.resize(**new_inner_size);
						},
						WindowEvent::KeyboardInput
						{
							input: KeyboardInput
								{
									state: ElementState::Pressed,
									virtual_keycode: Some(VirtualKeyCode::Escape),
									..
								},
							..
						} => *control_flow = ControlFlow::Exit,
						_ => {}
					}
				},
				_ => {}
			})
	}
}