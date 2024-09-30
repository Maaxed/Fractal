use std::collections::BTreeMap;
use std::fmt::Debug;

use egui::InnerResponse;
use fractal_renderer_shared as shared;
use shared::math::*;
use shared::fractal::{FractalKind, FractalVariation, RenderTechnique};
use glam::{dvec2, DVec2, i64vec2};
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowAttributes;

use crate::gui::EguiRenderer;
use crate::{Target, render};
use crate::quad_cell::QuadPos;
use crate::compute::Compute;
use crate::render::Render;

const VERTEX32_SHADER_CODE: &[u8] = include_bytes!(env!("fractal_renderer_shader_vertex32.spv"));
const VERTEX64_SHADER_CODE: &[u8] = include_bytes!(env!("fractal_renderer_shader_vertex64.spv"));
const FRAGMENT_SHADER_CODE: &[u8] = include_bytes!(env!("fractal_renderer_shader_fragment.spv"));
const COMPUTE32_SHADER_CODE: &[u8] = include_bytes!(env!("fractal_renderer_shader_computation32.spv")); // fractal_renderer_shader_compute32.spv
const COMPUTE64_SHADER_CODE: &[u8] = include_bytes!(env!("fractal_renderer_shader_computation64.spv")); // fractal_renderer_shader_compute64.spv

type ShaderCompute = crate::compute::ShaderRenderCompute;

pub struct AppWrapper<Init, C>
{
	init_function: Init,
	app: AppLifeCycle<C>
}

enum AppLifeCycle<C>
{
	Suspended,
	Initializing,
	Running(App<C>),
}

pub enum UserEvent<C>
{
	Initialized(App<C>),
}

impl<C> Debug for UserEvent<C>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Self::Initialized(_) => f.write_str("UserEvent::Initialized")
		}
	}
}


impl<Init, C> AppWrapper<Init, C>
{
	pub fn new(init_function: Init) -> Self
	{
		Self
		{
			init_function,
			app: AppLifeCycle::Suspended,
		}
	}

	/*pub fn run(self, event_loop: winit::event_loop::EventLoop<()>) -> Result<(), winit::error::EventLoopError>
	{
    	let window_attributes = WindowBuilder::default().with_title("Fractal").with_maximized(true);
		let window = window_attributes.build(&event_loop).expect("Failed to create window");

		let mut app = self.build_app(window);
		event_loop.run(|event, ctx|
		{
			match event
			{
				WinitEvent::AboutToWait => app.about_to_wait(ctx),
				WinitEvent::WindowEvent { window_id, event } => app.window_event(ctx, window_id, event),
				_ => {},
			}
		})
	}*/
}

impl<Init: Fn(winit::window::Window) -> Option<App<C>>, C: Compute> ApplicationHandler<UserEvent<C>> for AppWrapper<Init, C>
{
	fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop)
	{
		let window_attributes = WindowAttributes::default().with_title("Fractal").with_maximized(true);
		let window = event_loop.create_window(window_attributes).expect("Failed to create window");

		self.app = if let Some(app) = (self.init_function)(window)
		{
			AppLifeCycle::Running(app)
		}
		else
		{
			AppLifeCycle::Initializing
		};
	}

	fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, event: UserEvent<C>)
	{
		match event
		{
			UserEvent::Initialized(app) =>
			{
				assert!(matches!(self.app, AppLifeCycle::Initializing));

				self.app = AppLifeCycle::Running(app);
			},
		}
	}

	fn window_event(
		&mut self,
		event_loop: &winit::event_loop::ActiveEventLoop,
		window_id: winit::window::WindowId,
		event: WindowEvent,
	)
	{
		if let AppLifeCycle::Running(app) = &mut self.app
		{
			app.window_event(event_loop, window_id, event);
		}
	}

	fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop)
	{
		if let AppLifeCycle::Running(app) = &mut self.app
		{
			app.about_to_wait(event_loop);
		}
	}
}

pub struct App<C>
{
	target: Target,
	gui: EguiRenderer,
	render: Render,
	compute: C,
	app_data: AppData,
	mouse_left_down: bool,
	mouse_right_down: bool,
}

impl App<ShaderCompute>
{
	pub async fn build(device_limits: wgpu::Limits, window: winit::window::Window) -> Self
	{
		let target = Target::new(window, device_limits).await;
		
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

		/*let (cell_size, compute) = if target.supports_compute_shader
		{
			let cell_size = PhysicalSize::new(256, 256);

			let workgroup_size = glam::uvec2(16, 16);
			let compute = crate::compute::ShaderCompute::new(&target, &compute_shader_module, workgroup_size, cell_size, use_double_precision);

			(cell_size, AnyCompute::Shader(compute))
		}
		else
		{
			let cell_size = PhysicalSize::new(32, 32);
			
			let compute = crate::compute::ThreadedCompute::new(cell_size);
			
			(cell_size, AnyCompute::Threaded(compute))
		};*/

		let cell_size = PhysicalSize::new(256, 256);
		let compute = ShaderCompute::new(&target, &compute_shader_module, &compute_shader_module, cell_size, use_double_precision);

		let render = Render::new(&target, &vertex_shader_module, &fragment_shader_module, cell_size, use_double_precision);
		
		App::new(target, compute, render, cell_size)
	}
}

impl<C: Compute> App<C>
{

	fn new(target: Target, compute: C, render: Render, cell_size: PhysicalSize<u32>) -> Self
	{
		let gui = EguiRenderer::new(&target);
		let screen_size = target.window.inner_size();
		Self
		{
			target,
			gui,
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

		self.gui.draw(&self.target, commands, &view, |ui| {self.app_data.gui(ui);});
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

	fn window_event(
		&mut self,
		event_loop: &winit::event_loop::ActiveEventLoop,
		window_id: winit::window::WindowId,
		event: WindowEvent,
	)
	{
		let response = self.gui.handle_input(&self.target, &event);

		if response.repaint
		{
			self.app_data.require_redraw = true;
		}

		if window_id != self.target.window.id()
		{
			return;
		}

		match &event
		{
			WindowEvent::RedrawRequested =>
			{
				match self.redraw()
				{
					Ok(_) => {},
					// Reconfigure the surface if lost
					Err(wgpu::SurfaceError::Lost) => self.target.configure_surface(),
					// The system is out of memory, we should probably quit
					Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
					// All other errors (Outdated, Timeout) should be resolved by the next frame
					Err(e @ wgpu::SurfaceError::Outdated | e @ wgpu::SurfaceError::Timeout) => eprintln!("{:?}", e),
				}
			},
			WindowEvent::CloseRequested => event_loop.exit(),
			WindowEvent::Resized(physical_size) =>
			{
				self.resize(*physical_size);
			},
			WindowEvent::KeyboardInput
			{
				event: KeyEvent
					{
						state: ElementState::Pressed,
						physical_key: PhysicalKey::Code(keycode),
						..
					},
				..
			} =>
			{
				if response.consumed
				{
					return;
				}

				match keycode
				{
					KeyCode::Escape => event_loop.exit(),
					KeyCode::KeyM => self.app_data.set_fractal_kind(FractalKind::MandelbrotSet),
					KeyCode::Comma | KeyCode::Digit3 | KeyCode::Numpad3 => self.app_data.set_fractal_kind(FractalKind::Multibrot3),
					KeyCode::KeyT => self.app_data.set_fractal_kind(FractalKind::Tricorn),
					KeyCode::KeyS => self.app_data.set_fractal_kind(FractalKind::BurningShip),
					KeyCode::KeyC => self.app_data.set_fractal_kind(FractalKind::CosLeaf),
					KeyCode::KeyN => self.app_data.set_fractal_kind(FractalKind::Newton3),
					KeyCode::KeyL => self.app_data.set_fractal_kind(FractalKind::Lyapunov),
					KeyCode::KeyJ =>
					{
						self.app_data.set_fractal_variation(match self.app_data.fractal_params.variation
						{
							FractalVariation::Normal => FractalVariation::JuliaSet,
							FractalVariation::JuliaSet => FractalVariation::Normal,
						});
						(self.app_data.pos, self.app_data.fractal_params.secondary_pos) = (self.app_data.fractal_params.secondary_pos.to_vector(), Complex64::from_vector(self.app_data.pos));
						(self.app_data.zoom, self.app_data.secondary_zoom) = (self.app_data.secondary_zoom, self.app_data.zoom);
					},
					KeyCode::KeyO =>
					{
						self.app_data.set_fractal_rendering(match self.app_data.fractal_params.render_technique
						{
							RenderTechnique::Normal => RenderTechnique::OrbitTrapPoint,
							RenderTechnique::OrbitTrapPoint => RenderTechnique::OrbitTrapCross,
							RenderTechnique::OrbitTrapCross => RenderTechnique::NormalMap,
							RenderTechnique::NormalMap => RenderTechnique::Normal,
						});
					},
					KeyCode::KeyR =>
					{
						self.app_data.reset();
						self.target.window.request_redraw();
					},
					_ => {},
				}
			},
			WindowEvent::MouseWheel { delta, .. } =>
			{
				if response.consumed
				{
					return;
				}
				
				match delta
				{
					MouseScrollDelta::LineDelta(_dx, dy) =>
					{
						self.app_data.apply_zoom(*dy as f64, self.app_data.prev_mouse_pos);
					},
					MouseScrollDelta::PixelDelta(delta) =>
					{
						self.app_data.apply_zoom(delta.y * 0.01, self.app_data.prev_mouse_pos);
					},
				}
			},
			WindowEvent::MouseInput { button, state, ..} =>
			{
				match button
				{
					MouseButton::Left =>
					{
						if !response.consumed || self.mouse_left_down
						{
							self.mouse_left_down = *state == ElementState::Pressed;
						}
					},
					MouseButton::Right =>
					{
						if !response.consumed || self.mouse_right_down
						{
							self.mouse_right_down = *state == ElementState::Pressed;
						}
					},
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
						self.app_data.reset_fractal_rendering();
						self.app_data.fractal_params.secondary_pos -= Complex64::new(position.x - prev_pos.x, position.y - prev_pos.y) * self.app_data.pixel_world_size();
						self.target.window.request_redraw();
					}
				}
				self.app_data.prev_mouse_pos = Some(*position);
			},
			WindowEvent::PinchGesture { delta, phase: TouchPhase::Moved, .. } =>
			{
				if response.consumed
				{
					return;
				}
				
				self.app_data.apply_zoom(*delta, None);
			},
			WindowEvent::Touch(touch) =>
			{
				match touch.phase
				{
					TouchPhase::Started =>
					{
						if response.consumed
						{
							return;
						}

						for prev_touch in self.app_data.prev_touch_pos.each_mut()
						{
							if let Some((id, pos)) = prev_touch
							{
								if *id == touch.id
								{
									*pos = touch.location;
									break;
								}
							}
							else
							{
								*prev_touch = Some((touch.id, touch.location));
								break;
							}
						}
					},
					TouchPhase::Ended | TouchPhase::Cancelled =>
					{
						if let Some((id, _pos)) = self.app_data.prev_touch_pos[0]
						{
							if id == touch.id
							{
								// Shift elements of the array
								self.app_data.prev_touch_pos[0] = self.app_data.prev_touch_pos[1];
								self.app_data.prev_touch_pos[1] = None;
							}
							else if let Some((id, _pos)) = self.app_data.prev_touch_pos[1]
							{
								if id == touch.id
								{
									self.app_data.prev_touch_pos[1] = None;
								}
							}
						}
					}
					TouchPhase::Moved =>
					{
						if response.consumed
						{
							return;
						}

						let Some((id1, pos1)) = self.app_data.prev_touch_pos[0]
						else
						{
							return;
						};

						if let Some((id2, pos2)) = self.app_data.prev_touch_pos[1]
						{
							let second_pos = if touch.id == id1
							{
								self.app_data.prev_touch_pos[0] = Some((touch.id, touch.location));
								pos2
							}
							else if touch.id == id2
							{
								self.app_data.prev_touch_pos[1] = Some((touch.id, touch.location));
								pos1
							}
							else
							{
								return;
							};

							let pos1 = dvec2(pos1.x, pos1.y);
							let pos2 = dvec2(pos2.x, pos2.y);
							let touch_pos = dvec2(touch.location.x, touch.location.y);
							let second_pos = dvec2(second_pos.x, second_pos.y);

							let old_dist = pos1.distance(pos2);
							let new_dist = touch_pos.distance(second_pos);

							let center = (touch_pos + second_pos) * 0.5;

							let delta_dist = new_dist - old_dist;

							self.app_data.apply_zoom(delta_dist, Some(PhysicalPosition::new(center.x, center.y)));
						}
						else if touch.id == id1
						{
							self.app_data.pos -= dvec2(touch.location.x - pos1.x, pos1.y - touch.location.y) * self.app_data.pixel_world_size();
							self.target.window.request_redraw();
							
							self.app_data.prev_touch_pos[0] = Some((touch.id, touch.location));
						}
					}
				}
			},
			_ => {}
		}
	}

	fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop)
	{
		if self.app_data.require_redraw
		{
			self.app_data.require_redraw = false;
			self.target.window.request_redraw();
		}
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
	prev_touch_pos: [Option<(u64, PhysicalPosition<f64>)>; 2],
	require_redraw: bool,
}

impl AppData
{
	pub fn new(cell_size: PhysicalSize<u32>, screen_size: PhysicalSize<u32>) -> Self
	{
		let mut this = Self
		{
			cell_size: cell_size.width.min(cell_size.height),
			screen_size: PhysicalSize::default(),
			cells: BTreeMap::new(),
			pos: DVec2::ZERO,
			zoom: 1.0,
			secondary_zoom: 1.0,
			fractal_params: Default::default(),
			prev_mouse_pos: None,
			prev_touch_pos: [None, None],
			require_redraw: false,
		};

		this.resize(screen_size);

		this
	}

	fn resize(&mut self, new_screen_size: PhysicalSize<u32>)
	{
		self.screen_size = PhysicalSize
		{
			width: new_screen_size.width.max(1),
			height: new_screen_size.height.max(1),
		};
	}

	fn reset(&mut self)
	{
		self.cells = BTreeMap::new();
		self.pos = DVec2::ZERO;
		self.zoom = 1.0;
		self.fractal_params.secondary_pos = Complex64::ZERO;
		self.fractal_params.iteration_limit = self.fractal_params.fractal_kind.default_iteration_limit();
	}

	fn apply_zoom(&mut self, zoom_value: f64, zoom_center: Option<PhysicalPosition<f64>>)
	{
		let old_zoom = self.zoom;
		self.zoom *= (-zoom_value * 0.5).exp();

		if let Some(zoom_center) = zoom_center
		{
			self.pos += dvec2(zoom_center.x - self.screen_size.width as f64 * 0.5, self.screen_size.height as f64 * 0.5 - zoom_center.y) * self.base_pixel_world_size() * (old_zoom - self.zoom);
		}
		
		self.require_redraw = true;
	}

	fn reset_fractal_rendering(&mut self)
	{
		self.cells.clear();
		self.require_redraw = true;
	}

	fn set_fractal_kind(&mut self, fractal_kind: FractalKind)
	{
		if self.fractal_params.fractal_kind == fractal_kind
		{
			return;
		}

		self.fractal_params.fractal_kind = fractal_kind;
		
		self.fractal_params.iteration_limit = self.fractal_params.fractal_kind.default_iteration_limit();
		self.reset_fractal_rendering();
	}

	fn set_fractal_variation(&mut self, fractal_variation: FractalVariation)
	{
		if self.fractal_params.variation == fractal_variation
		{
			return;
		}

		self.fractal_params.variation = fractal_variation;
		
		self.reset_fractal_rendering();
	}

	fn set_fractal_rendering(&mut self, rendering_technique: RenderTechnique)
	{
		if self.fractal_params.render_technique == rendering_technique
		{
			return;
		}

		self.fractal_params.render_technique = rendering_technique;
		
		self.reset_fractal_rendering();
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

	pub fn gui(&mut self, ctx: &egui::Context) -> bool
	{
		let response = egui::Window::new("Fractal")
			.resizable(false)
			.show(ctx, |ui|
			{
				fn select_in_list<T: Eq>(ui: &mut egui::Ui, selected_value: &T, list: impl IntoIterator<Item = (T, &'static str)>) -> Option<T>
				{
					ui.horizontal(|ui|
					{
						let mut new_selected = None;
	
						for (value, name) in list
						{
							if ui.selectable_label(*selected_value == value, name).clicked()
							{
								if *selected_value != value
								{
									new_selected = Some(value);
								}
							}
						}
	
						new_selected
					}).inner
				}

				egui::Grid::new("config_grid")
					.num_columns(2)
					.striped(true)
					.show(ui, |ui|
					{
						let mut changed = false;
						
						ui.label("Fractal Kind");
						if let Some(fractal_kind) = select_in_list(ui, &self.fractal_params.fractal_kind, [
							(FractalKind::MandelbrotSet, "Mandelbrot Set"),
							(FractalKind::Multibrot3, "Multibrot 3"),
							(FractalKind::Tricorn, "Tricorn"),
							(FractalKind::BurningShip, "Burning Ship"),
							(FractalKind::CosLeaf, "Cos Leaf"),
							(FractalKind::Newton3, "Newton 3"),
							(FractalKind::Lyapunov, "Lyapunov"),
						])
						{
							self.set_fractal_kind(fractal_kind);
							changed = true;
						}
						ui.end_row();
		
						ui.label("Variation");
						if let Some(fractal_variation) = select_in_list(ui, &self.fractal_params.variation, [
							(FractalVariation::Normal, "Classic"),
							(FractalVariation::JuliaSet, "Julia Set"),
						])
						{
							self.set_fractal_variation(fractal_variation);
							// Swap primary and secondary pos/zoom
							(self.pos, self.fractal_params.secondary_pos) = (self.fractal_params.secondary_pos.to_vector(), Complex64::from_vector(self.pos));
							(self.zoom, self.secondary_zoom) = (self.secondary_zoom, self.zoom);
							changed = true;
						}
						ui.end_row();
		
						ui.label("Render Technique");
						if let Some(rendering_technique) = select_in_list(ui, &self.fractal_params.render_technique, [
							(RenderTechnique::Normal, "Normal"),
							(RenderTechnique::OrbitTrapPoint, "Orbit Trap Point"),
							(RenderTechnique::OrbitTrapCross, "Orbit Trap Cross"),
							(RenderTechnique::NormalMap, "Normal Map"),
						])
						{
							self.set_fractal_rendering(rendering_technique);
							changed = true;
						}
						ui.end_row();
						
						ui.label("Iteration Limit");
						ui.horizontal(|ui|
						{
							changed |= ui.add(egui::DragValue::new(&mut self.fractal_params.iteration_limit).speed(1)).changed();
						});
						ui.end_row();
						
						ui.label("Position");
						ui.horizontal(|ui|
						{
							let speed = self.zoom * 0.05;
							ui.add(egui::DragValue::new(&mut self.pos.x).speed(speed).prefix("x: "));
							ui.add(egui::DragValue::new(&mut self.pos.y).speed(speed).prefix("y: "));
						});
						ui.end_row();
						
						ui.label("C Constant");
						ui.horizontal(|ui|
						{
							let speed = self.zoom * 0.05;
							changed |= ui.add(egui::DragValue::new(self.fractal_params.secondary_pos.re_mut()).speed(speed).prefix("x: ")).changed();
							changed |= ui.add(egui::DragValue::new(self.fractal_params.secondary_pos.im_mut()).speed(speed).prefix("y: ")).changed();
						});
						ui.end_row();
						
						ui.label("Zoom");
						ui.horizontal(|ui|
						{
							let speed = self.zoom * 0.02;
							ui.add(egui::DragValue::new(&mut self.zoom).speed(speed).suffix("x").range(0.000000000000001..=f64::MAX));
						});
						ui.end_row();
		
						ui.label("");
						if ui.button("Reset").clicked()
						{
							self.reset();
							changed = true;
						}
						ui.end_row();
		
						changed
					}).inner
			});

		let changed = match response
		{
			Some(InnerResponse{ inner: Some(true), .. }) => true,
			_ => false,
		};

		if changed
		{
			self.reset_fractal_rendering();
		}
		changed
	}
}
