use glam::{DVec2, dvec2};
use winit::dpi::PhysicalPosition;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode, MouseScrollDelta, MouseButton};

const SHADER_CODE: &[u8] = include_bytes!(env!("fractal_renderer_shader.spv"));

pub struct App
{
	target: crate::Target,
	render: crate::render::Render,
	compute: crate::compute::Compute,
	zoom: f64,
	pos: DVec2,
	prev_mouse_pos: Option<PhysicalPosition<f64>>,
	mouse_down: bool
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
		
    	let size = 512;
    	let compute = crate::compute::Compute::new(&shader_module, &target, size);
		
		let render = crate::render::Render::new(&shader_module, &target, size);

		Self
		{
			target,
			render,
			compute,
			zoom: 1.0,
			pos: DVec2::ZERO,
			prev_mouse_pos: None,
			mouse_down: false
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

	pub fn redraw(&self) -> Result<(), wgpu::SurfaceError>
	{
		self.compute.set_params(&self.target.queue,
			&fractal_renderer_shared::ComputeParams
			{
				zoom: self.zoom as f32,
				pos: self.pos.as_vec2(),
				padding: 0,
			});
		self.do_compute();
		self.do_render()
	}

	pub fn run(mut self, event_loop: EventLoop<()>) -> !
	{
		event_loop.run(move
			|event, _, control_flow|
			match event
			{
				Event::RedrawRequested(window_id) if window_id == self.target.window.id() =>
				{
					match self.redraw()
					{
						Ok(_) => {},
						// Reconfigure the surface if lost
						Err(wgpu::SurfaceError::Lost) => self.target.configure_surface(),
						// The system is out of memory, we should probably quit
						Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
						// All other errors (Outdated, Timeout) should be resolved by the next frame
						Err(e @ wgpu::SurfaceError::Outdated | e @ wgpu::SurfaceError::Timeout) => eprintln!("{:?}", e),
					}
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
						WindowEvent::MouseWheel { delta, .. } =>
						{
							match delta
							{
								MouseScrollDelta::LineDelta(_dx, dy) =>
								{
									self.zoom *= (*dy as f64).exp();
									self.target.window.request_redraw();
								},
								MouseScrollDelta::PixelDelta(delta) =>
								{
									self.zoom *= delta.y.exp();
									self.target.window.request_redraw();
								},
							}
						},
						WindowEvent::MouseInput { button: MouseButton::Left, state, ..} =>
						{
							self.mouse_down = *state == ElementState::Pressed;
						},
						WindowEvent::CursorMoved { position, .. } =>
						{
							if self.mouse_down
							{
								if let Some(prev_pos) = self.prev_mouse_pos
								{
									self.pos -= dvec2(position.x - prev_pos.x, position.y - prev_pos.y) * 0.01 / self.zoom;
									self.target.window.request_redraw();
								}
							}
							self.prev_mouse_pos = Some(*position);
						}
						_ => {}
					}
				},
				_ => {}
			})
	}
}