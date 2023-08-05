use std::collections::BTreeMap;
use std::ops::RangeInclusive;

use fractal_renderer_shared as shared;
use shared::complex::Complex;
use shared::fractal::FractalKind;
use glam::{dvec2, DVec2, i64vec2};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode, MouseScrollDelta, MouseButton};

use crate::quad_cell::QuadPos;

const SHADER_CODE: &[u8] = include_bytes!(env!("fractal_renderer_shader.spv"));

pub struct App
{
	target: crate::Target,
	render: crate::render::Render,
	compute: crate::compute::Compute,
	cell_size: u32,
	cells: BTreeMap<QuadPos, crate::render::Instance>,
    pos: DVec2,
    zoom: f64,
	fractal_params: shared::fractal::FractalParams,
	prev_mouse_pos: Option<PhysicalPosition<f64>>,
	mouse_left_down: bool,
	mouse_right_down: bool,
	require_redraw: bool,
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
		let cell_size = PhysicalSize::new(256, 256);
    	let compute = crate::compute::Compute::new(&target, &shader_module, workgroup_size, cell_size);
		
		let render = crate::render::Render::new(&target, &shader_module, cell_size);

		Self
		{
			target,
			render,
			compute,
			cell_size: cell_size.width.min(cell_size.height),
			cells: BTreeMap::new(),
			pos: DVec2::ZERO,
			zoom: 1.0,
			fractal_params: Default::default(),
			prev_mouse_pos: None,
			mouse_left_down: false,
			mouse_right_down: false,
			require_redraw: false,
		}
	}

    fn resize(&mut self, new_size: PhysicalSize<u32>)
	{
		self.target.resize(new_size);
    }

	fn apply_zoom(&mut self, zoom_value: f64)
	{
		let old_zoom = self.zoom;
		self.zoom *= (-zoom_value * 0.5).exp();

		if let Some(mouse_pos) = self.prev_mouse_pos
		{
			self.pos += dvec2(mouse_pos.x - self.target.config.width as f64 * 0.5, self.target.config.height as f64 * 0.5 - mouse_pos.y) * self.base_pixel_world_size() * (old_zoom - self.zoom);
		}
		
		self.target.window.request_redraw();
	}

	fn set_fractal_kind(&mut self, fractal_kind: FractalKind)
	{
		if self.fractal_params.fractal_kind == fractal_kind
		{
			return;
		}

		self.cells.clear();

		self.fractal_params.fractal_kind = fractal_kind;
		
		self.target.window.request_redraw();
	}

	fn base_pixel_world_size(&self) -> f64
	{
		4.0 / self.target.config.width.min(self.target.config.height) as f64
	}

	fn pixel_world_size(&self) -> f64
	{
		self.base_pixel_world_size() * self.zoom
	}

	fn viewport_world_size(&self) -> DVec2
	{
		let window_size = dvec2(self.target.config.width as f64, self.target.config.height as f64);
		window_size * self.pixel_world_size()
	}

	fn compute_cell(&self, commands: &mut wgpu::CommandEncoder, pos: QuadPos) -> crate::render::Instance
	{
		let cell_size = pos.cell_size();
		let cell_pos = pos.cell_bottom_left();

		let cell = self.render.make_instance(&self.target);

		cell.set_data(&self.target.queue, &shared::render::Instance
		{
			pos: cell_pos,
			size: DVec2::splat(cell_size),
		});

		// Compute
		self.compute.set_params(&self.target.queue, &shared::ComputeParams
		{
			min_pos: cell_pos + dvec2(0.0, cell_size),
			max_pos: cell_pos + dvec2(cell_size, 0.0),
			fractal: self.fractal_params,
		});
		self.compute.make_compute_pass(commands);
		self.compute.copy_buffer_to_texture(commands, cell.fractal_texture());

		cell
	}

	fn cleanup_cells(&mut self, viewport_size: DVec2, exponent_range: (i32, i32))
	{
		let valid_exponents = (exponent_range.0 - 4) ..= (exponent_range.1 + 4);
		let valid_pos_min = self.pos - viewport_size * 2.0;
		let valid_pos_max = self.pos + viewport_size * 2.0;

		self.cells.retain(|pos, _cell| valid_exponents.contains(&pos.exponent) && pos.cell_bottom_left().cmplt(valid_pos_max).all() && pos.cell_top_right().cmpgt(valid_pos_min).all());
	}

	fn find_cell_to_load(&mut self, viewport_size: DVec2, exponent_range: (i32, i32)) -> Option<QuadPos>
	{
		let viewport_min = self.pos - viewport_size / 2.0;
		let viewport_max = self.pos + viewport_size / 2.0;

		for exponent in (exponent_range.0 ..= exponent_range.1).rev()
		{
			let cell_size = 2.0_f64.powi(exponent);

			let quad_min = (viewport_min / cell_size).floor().as_i64vec2();
			let quad_max = (viewport_max / cell_size).ceil().as_i64vec2();

			let cells_iter = (quad_min.x .. quad_max.x).flat_map(|x| (quad_min.y .. quad_max.y).map(move |y| i64vec2(x, y)));
			let mut cells: Vec<_> = cells_iter.map(|pos| (pos, (self.pos - (pos.as_dvec2() + 0.5) * cell_size).length_squared())).collect();
			cells.sort_by(|(_pos1, dist1), (_pos2, dist2)| dist1.partial_cmp(dist2).unwrap());

			for (pos, _dist) in cells
			{
				let cell = QuadPos { unscaled_pos: pos, exponent };
				if !self.cells.contains_key(&cell)
				{
					return Some(cell);
				}
			}
		}
		None
	}

	fn do_render(&mut self, commands: &mut wgpu::CommandEncoder, output: &wgpu::SurfaceTexture)
	{
		let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

		let scale = 2.0 / self.viewport_world_size();
		self.render.set_uniforms(&self.target.queue, &shared::render::Uniforms
			{
				camera_pos: self.pos,
				world_to_view_scale: scale,
			});

		self.render.make_render_pass(self.cells.iter().map(|(_pos, instance)| instance), &view, commands);
	}

	pub fn redraw(&mut self) -> Result<(), wgpu::SurfaceError>
	{
		self.require_redraw = false;

		let viewport_size = self.viewport_world_size();
		let min_exponent = (self.pixel_world_size() * self.cell_size as f64).log2().floor() as i32;
		let max_exponent = (self.zoom.log2().ceil() as i32 + 1).max(min_exponent);
		let exponent_range = (min_exponent, max_exponent);

		// Free cells that are far away
		self.cleanup_cells(viewport_size, exponent_range);

		// Find new cell to load
		let quad_pos = self.find_cell_to_load(viewport_size, exponent_range);
		
		let mut commands = self.target.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

		if let Some(pos) = quad_pos
		{
			self.cells.insert(pos, self.compute_cell(&mut commands, pos));
			self.require_redraw = true;
		}
		
		let output = self.target.surface.get_current_texture()?;

		self.do_render(&mut commands, &output);

		// Submit
		self.target.queue.submit(std::iter::once(commands.finish()));
		output.present();

		Ok(())
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
				Event::MainEventsCleared =>
				{
					if self.require_redraw
					{
						self.require_redraw = false;
						self.target.window.request_redraw();
					}
				}
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
								VirtualKeyCode::Comma | VirtualKeyCode::Key3 | VirtualKeyCode::Numpad3 => self.set_fractal_kind(FractalKind::Multibrot3),
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
									self.pos -= dvec2(position.x - prev_pos.x, prev_pos.y - position.y) * self.pixel_world_size();
									self.target.window.request_redraw();
								}
								else if self.mouse_right_down
								{
									self.cells.clear();
									self.fractal_params.secondary_pos -= Complex::new(position.x - prev_pos.x, position.y - prev_pos.y) * self.pixel_world_size();
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