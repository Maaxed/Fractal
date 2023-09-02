use std::collections::BTreeMap;

use fractal_renderer_shared as shared;
use shared::math::*;
use shared::fractal::{FractalKind, FractalVariation, RenderTechnique};
use glam::{dvec2, DVec2, i64vec2};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode, MouseScrollDelta, MouseButton};

use crate::{Target, render};
use crate::quad_cell::QuadPos;
use crate::compute::Compute;
use crate::render::Render;

const VERTEX32_SHADER_CODE: &[u8] = include_bytes!(env!("fractal_renderer_shader_vertex32.spv"));
const VERTEX64_SHADER_CODE: &[u8] = include_bytes!(env!("fractal_renderer_shader_vertex64.spv"));
const FRAGMENT_SHADER_CODE: &[u8] = include_bytes!(env!("fractal_renderer_shader_fragment.spv"));
const COMPUTE32_SHADER_CODE: &[u8] = include_bytes!(env!("fractal_renderer_shader_compute32.spv"));
const COMPUTE64_SHADER_CODE: &[u8] = include_bytes!(env!("fractal_renderer_shader_compute64.spv"));

pub struct App<C>
{
	target: Target,
	render: Render,
	compute: C,
	app_data: AppData,
	mouse_left_down: bool,
	mouse_right_down: bool,
}

pub fn run_app(target: Target, event_loop: EventLoop<()>) -> !
{
	let use_double_precision = target.device.features().contains(wgpu::Features::SHADER_F64);

	let (vertex_shader_code, fragment_shader_code, compute_shader_code) =
		if use_double_precision
		{
			(VERTEX64_SHADER_CODE, FRAGMENT_SHADER_CODE, COMPUTE64_SHADER_CODE)
		}
		else
		{
			(VERTEX32_SHADER_CODE, FRAGMENT_SHADER_CODE, COMPUTE32_SHADER_CODE)
		};
	
	let vertex_shader_module = target.device.create_shader_module(
		wgpu::ShaderModuleDescriptor
		{
			label: Some("vertex_shader"),
			source: wgpu::util::make_spirv(vertex_shader_code),
		});
	
	let fragment_shader_module = target.device.create_shader_module(
		wgpu::ShaderModuleDescriptor
		{
			label: Some("fragment_shader"),
			source: wgpu::util::make_spirv(fragment_shader_code),
		});
	
	let compute_shader_module = target.device.create_shader_module(
		wgpu::ShaderModuleDescriptor
		{
			label: Some("compute_shader"),
			source: wgpu::util::make_spirv(compute_shader_code),
		});

	if target.supports_compute_shader
	{
		let cell_size = PhysicalSize::new(256, 256);
		
		let render = Render::new(&target, &vertex_shader_module, &fragment_shader_module, cell_size, use_double_precision);

		let workgroup_size = glam::uvec2(16, 16);
		let compute = crate::compute::ShaderCompute::new(&target, &compute_shader_module, workgroup_size, cell_size, use_double_precision);

		App::new(target, compute, render, cell_size).run(event_loop);
	}
	else
	{
		let cell_size = PhysicalSize::new(32, 32);
		
		let render = Render::new(&target, &vertex_shader_module, &fragment_shader_module, cell_size, use_double_precision);
		
		let compute = crate::compute::ThreadedCompute::new(cell_size);
		App::new(target, compute, render, cell_size).run(event_loop)
	};

}

impl<C: Compute> App<C>
{
	pub fn new(target: Target, compute: C, render: Render, cell_size: PhysicalSize<u32>) -> Self
	{
		let screen_size = target.window.inner_size();
		Self
		{
			target,
			render,
			compute,
			app_data: AppData::new(cell_size, screen_size),
			mouse_left_down: false,
			mouse_right_down: false,
		}
	}

    fn resize(&mut self, new_size: PhysicalSize<u32>)
	{
		self.target.resize(new_size);
		self.app_data.resize(new_size);
    }

	fn reset_fractal_rendering(&mut self)
	{
		self.app_data.cells.clear();
		self.compute.reset();
	}

	fn set_fractal_kind(&mut self, fractal_kind: FractalKind)
	{
		if self.app_data.fractal_params.fractal_kind == fractal_kind
		{
			return;
		}

		self.reset_fractal_rendering();

		self.app_data.fractal_params.fractal_kind = fractal_kind;
		
		self.app_data.require_redraw = true;
	}

	fn set_fractal_variation(&mut self, fractal_variation: FractalVariation)
	{
		if self.app_data.fractal_params.variation == fractal_variation
		{
			return;
		}

		self.reset_fractal_rendering();

		self.app_data.fractal_params.variation = fractal_variation;
		
		self.app_data.require_redraw = true;
	}

	fn set_fractal_rendering(&mut self, rendering_technique: RenderTechnique)
	{
		if self.app_data.fractal_params.render_technique == rendering_technique
		{
			return;
		}

		self.reset_fractal_rendering();

		self.app_data.fractal_params.render_technique = rendering_technique;
		
		self.app_data.require_redraw = true;
	}

	fn do_render(&mut self, commands: &mut wgpu::CommandEncoder, output: &wgpu::SurfaceTexture)
	{
		let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

		let scale = 2.0 / self.app_data.viewport_world_size();
		self.render.set_uniforms(&self.target.queue, &shared::render::Uniforms64
			{
				camera_pos: self.app_data.pos,
				world_to_view_scale: scale,
			});

		self.render.make_render_pass(self.app_data.cells.values(), &view, commands);
	}

	pub fn redraw(&mut self) -> Result<(), wgpu::SurfaceError>
	{
		self.app_data.require_redraw = false;

		// Free cells that are far away
		self.app_data.cleanup_cells();

		let mut commands = self.target.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

		self.compute.update_before_render(&self.target, &self.render, &mut self.app_data, &mut commands);
		
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
					if self.app_data.require_redraw
					{
						self.app_data.require_redraw = false;
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
								VirtualKeyCode::Comma | VirtualKeyCode::Key3 | VirtualKeyCode::Numpad3 => self.set_fractal_kind(FractalKind::Multibrot3),
								VirtualKeyCode::T => self.set_fractal_kind(FractalKind::Tricorn),
								VirtualKeyCode::S => self.set_fractal_kind(FractalKind::BurningShip),
								VirtualKeyCode::C => self.set_fractal_kind(FractalKind::CosLeaf),
								VirtualKeyCode::D => self.set_fractal_kind(FractalKind::MandelbrotNormal),
								VirtualKeyCode::N => self.set_fractal_kind(FractalKind::Newton3),
								VirtualKeyCode::L => self.set_fractal_kind(FractalKind::Lyapunov),
								VirtualKeyCode::J =>
								{
									self.set_fractal_variation(match self.app_data.fractal_params.variation
									{
										FractalVariation::Normal => FractalVariation::JuliaSet,
										FractalVariation::JuliaSet => FractalVariation::Normal,
									});
									(self.app_data.pos, self.app_data.fractal_params.secondary_pos) = (self.app_data.fractal_params.secondary_pos.to_vector(), Complex64::from_vector(self.app_data.pos));
									(self.app_data.zoom, self.app_data.secondary_zoom) = (self.app_data.secondary_zoom, self.app_data.zoom);
								},
								VirtualKeyCode::O =>
								{
									self.set_fractal_rendering(match self.app_data.fractal_params.render_technique
									{
										RenderTechnique::Normal => RenderTechnique::OrbitTrapPoint,
										RenderTechnique::OrbitTrapPoint => RenderTechnique::OrbitTrapCross,
										RenderTechnique::OrbitTrapCross => RenderTechnique::Normal,
									});
								},
								VirtualKeyCode::R =>
								{
									self.app_data.reset();
									self.target.window.request_redraw();
								},
								_ => {},
							}
						},
						WindowEvent::MouseWheel { delta, .. } =>
						{
							match delta
							{
								MouseScrollDelta::LineDelta(_dx, dy) =>
								{
									self.app_data.apply_zoom(*dy as f64);
								},
								MouseScrollDelta::PixelDelta(delta) =>
								{
									self.app_data.apply_zoom(delta.y * 10.0);
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
							if let Some(prev_pos) = self.app_data.prev_mouse_pos
							{
								if self.mouse_left_down
								{
									self.app_data.pos -= dvec2(position.x - prev_pos.x, prev_pos.y - position.y) * self.app_data.pixel_world_size();
									self.target.window.request_redraw();
								}
								else if self.mouse_right_down
								{
									self.reset_fractal_rendering();
									self.app_data.fractal_params.secondary_pos -= Complex64::new(position.x - prev_pos.x, position.y - prev_pos.y) * self.app_data.pixel_world_size();
									self.target.window.request_redraw();
								}
							}
							self.app_data.prev_mouse_pos = Some(*position);
						}
						_ => {}
					}
				},
				_ => {}
			})
	}
}

pub struct AppData
{
	cell_size: u32,
	screen_size: PhysicalSize<u32>,
	cells: BTreeMap<QuadPos, crate::render::Instance>,
    pos: DVec2,
    zoom: f64,
	secondary_zoom: f64,
	pub(crate) fractal_params: shared::fractal::FractalParams64,
	prev_mouse_pos: Option<PhysicalPosition<f64>>,
	require_redraw: bool,
}

impl AppData
{
	pub fn new(cell_size: PhysicalSize<u32>, screen_size: PhysicalSize<u32>) -> Self
	{
		Self
		{
			cell_size: cell_size.width.min(cell_size.height),
			screen_size,
			cells: BTreeMap::new(),
			pos: DVec2::ZERO,
			zoom: 1.0,
			secondary_zoom: 1.0,
			fractal_params: Default::default(),
			prev_mouse_pos: None,
			require_redraw: false,
		}
	}

	fn resize(&mut self, new_screen_size: PhysicalSize<u32>)
	{
		self.screen_size = new_screen_size;
	}

	fn reset(&mut self)
	{
		self.cells = BTreeMap::new();
		self.pos = DVec2::ZERO;
		self.zoom = 1.0;
		self.fractal_params.secondary_pos = Complex64::ZERO;
	}

	fn apply_zoom(&mut self, zoom_value: f64)
	{
		let old_zoom = self.zoom;
		self.zoom *= (-zoom_value * 0.5).exp();

		if let Some(mouse_pos) = self.prev_mouse_pos
		{
			self.pos += dvec2(mouse_pos.x - self.screen_size.width as f64 * 0.5, self.screen_size.height as f64 * 0.5 - mouse_pos.y) * self.base_pixel_world_size() * (old_zoom - self.zoom);
		}
		
		self.require_redraw = true;
	}

	fn base_pixel_world_size(&self) -> f64
	{
		4.0 / self.screen_size.width.min(self.screen_size.height) as f64
	}

	fn pixel_world_size(&self) -> f64
	{
		self.base_pixel_world_size() * self.zoom
	}

	fn viewport_world_size(&self) -> DVec2
	{
		let window_size = dvec2(self.screen_size.width as f64, self.screen_size.height as f64);
		window_size * self.pixel_world_size()
	}

	fn exponent_range(&self) -> (i32, i32)
	{
		let min_exponent = (self.pixel_world_size() * self.cell_size as f64).log2().floor() as i32;
		let max_exponent = (self.zoom.log2().ceil() as i32 + 1).max(min_exponent);
		(min_exponent, max_exponent)
	}

	fn cleanup_cells(&mut self)
	{
		let viewport_size = self.viewport_world_size();
		let exponent_range = self.exponent_range();

		let valid_exponents = (exponent_range.0 - 4) ..= (exponent_range.1 + 4);
		let valid_pos_min = self.pos - viewport_size * 2.0;
		let valid_pos_max = self.pos + viewport_size * 2.0;

		self.cells.retain(|pos, _cell| valid_exponents.contains(&pos.exponent) && pos.cell_bottom_left().cmplt(valid_pos_max).all() && pos.cell_top_right().cmpgt(valid_pos_min).all());
	}

	pub fn visible_cells(&self) -> impl Iterator<Item = QuadPos>
	{
		let viewport_size = self.viewport_world_size();
		let exponent_range = self.exponent_range();

		let cal_pos = self.pos;
		let viewport_min = cal_pos - viewport_size / 2.0;
		let viewport_max = cal_pos + viewport_size / 2.0;

		(exponent_range.0 ..= exponent_range.1).rev().flat_map(move |exponent|
		{
			let cell_size = 2.0_f64.powi(exponent);

			let quad_min = (viewport_min / cell_size).floor().as_i64vec2();
			let quad_max = (viewport_max / cell_size).ceil().as_i64vec2();

			let cells_iter = (quad_min.x .. quad_max.x).flat_map(|x| (quad_min.y .. quad_max.y).map(move |y| i64vec2(x, y)));
			let mut cells: Vec<_> = cells_iter.map(|pos| (pos, (cal_pos - (pos.as_dvec2() + 0.5) * cell_size).length_squared())).collect();
			cells.sort_by(|(_pos1, dist1), (_pos2, dist2)| dist1.partial_cmp(dist2).unwrap());

			cells.into_iter().map(move |(pos, _dist)|
			{
				QuadPos { unscaled_pos: pos, exponent }
			})
		})
	}

	pub fn is_cell_loaded(&self, pos: QuadPos) -> bool
	{
		self.cells.contains_key(&pos)
	}

	pub fn make_cell(&mut self, target: &Target, render: &Render, pos: QuadPos) -> &render::Instance
	{
		let cell_size = pos.cell_size();
		let cell_pos = pos.cell_bottom_left();

		let cell = render.make_instance(target);

		cell.set_data(&target.queue, &shared::render::Instance64
		{
			pos: cell_pos,
			size: DVec2::splat(cell_size),
		});

		self.cells.insert(pos, cell);

		self.require_redraw = true;

		&self.cells[&pos]
	}
}
