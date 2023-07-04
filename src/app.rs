use fractal_renderer_shared as shared;
use shared::fractal::FractalKind;
use glam::dvec2;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode, MouseScrollDelta, MouseButton};

const SHADER_CODE: &[u8] = include_bytes!(env!("fractal_renderer_shader.spv"));

pub struct App
{
	target: crate::Target,
	render: crate::render::Render,
	compute: crate::compute::Compute,
	fractal_params: shared::ComputeParams,
	prev_mouse_pos: Option<PhysicalPosition<f64>>,
	mouse_left_down: bool,
	mouse_right_down: bool,
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
		
		let workgroup_size = glam::uvec2(16, 16);
    	let size = target.window.inner_size();
    	let compute = crate::compute::Compute::new(&target, &shader_module, workgroup_size, size);
		
		let render = crate::render::Render::new(&target, &shader_module, size);

		Self
		{
			target,
			render,
			compute,
			fractal_params: Default::default(),
			prev_mouse_pos: None,
			mouse_left_down: false,
			mouse_right_down: false,
		}
	}

    fn resize(&mut self, new_size: PhysicalSize<u32>)
	{
		if self.target.resize(new_size)
		{
			self.compute.resize(&self.target, new_size);
			self.render.resize(&self.target, new_size);
		}
    }

	fn apply_zoom(&mut self, zoom_value: f64)
	{
		let old_zoom = self.fractal_params.zoom;
		self.fractal_params.zoom *= (-zoom_value * 0.5).exp();

		if let Some(mouse_pos) = self.prev_mouse_pos
		{
			self.fractal_params.pos += dvec2(mouse_pos.x - self.target.config.width as f64 * 0.5, mouse_pos.y - self.target.config.height as f64 * 0.5) * self.pixel_world_size() * (old_zoom - self.fractal_params.zoom);
		}
		
		self.target.window.request_redraw();
	}

	fn set_fractal_kind(&mut self, fractal_kind: FractalKind)
	{
		self.fractal_params.fractal_kind = fractal_kind;
		
		self.target.window.request_redraw();
	}

	fn pixel_world_size(&self) -> f64
	{
		4.0 / (self.target.config.width.min(self.target.config.height) as f64 - 1.0)
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

		for line in result.chunks_exact(self.compute.size().width as usize)
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

		self.compute.copy_buffer_to_texture(&mut commands, self.render.fractal_texture());
		
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
		self.compute.set_params(&self.target.queue, &self.fractal_params);
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
							self.resize(*physical_size);
						},
						WindowEvent::ScaleFactorChanged { new_inner_size, .. } =>
						{
							// new_inner_size is &&mut so we have to dereference it twice
							self.resize(**new_inner_size);
						},
						WindowEvent::KeyboardInput
						{
							input: KeyboardInput
								{
									state: ElementState::Pressed,
									virtual_keycode: Some(keycode),
									..
								},
							..
						} =>
						{
							match keycode
							{
								VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
								VirtualKeyCode::M => self.set_fractal_kind(FractalKind::MandelbrotSet),
								VirtualKeyCode::J => self.set_fractal_kind(FractalKind::JuliaSet),
								VirtualKeyCode::Key3 | VirtualKeyCode::Comma => self.set_fractal_kind(FractalKind::Multibrot3),
								VirtualKeyCode::T => self.set_fractal_kind(FractalKind::Tricorn),
								VirtualKeyCode::S => self.set_fractal_kind(FractalKind::BurningShip),
								VirtualKeyCode::C => self.set_fractal_kind(FractalKind::CosLeaf),
								VirtualKeyCode::L => self.set_fractal_kind(FractalKind::Lyapunov),
								_ => {},
							}
						},
						WindowEvent::MouseWheel { delta, .. } =>
						{
							match delta
							{
								MouseScrollDelta::LineDelta(_dx, dy) =>
								{
									self.apply_zoom(*dy as f64);
								},
								MouseScrollDelta::PixelDelta(delta) =>
								{
									self.apply_zoom(delta.y * 10.0);
								},
							}
						},
						WindowEvent::MouseInput { button, state, ..} =>
						{
							match button
							{
								MouseButton::Left => self.mouse_left_down = *state == ElementState::Pressed,
								MouseButton::Right => self.mouse_right_down = *state == ElementState::Pressed,
								_ => {},
							}
						},
						WindowEvent::CursorMoved { position, .. } =>
						{
							if let Some(prev_pos) = self.prev_mouse_pos
							{
								if self.mouse_left_down
								{
									self.fractal_params.pos -= dvec2(position.x - prev_pos.x, position.y - prev_pos.y) * self.pixel_world_size() * self.fractal_params.zoom;
									self.target.window.request_redraw();
								}
								else if self.mouse_right_down
								{
									self.fractal_params.secondary_pos -= dvec2(position.x - prev_pos.x, position.y - prev_pos.y) * self.pixel_world_size() * self.fractal_params.zoom;
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