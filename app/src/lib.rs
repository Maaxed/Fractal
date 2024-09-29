mod target;
pub mod compute;
pub mod render;
mod quad_cell;
pub mod app;
mod gui;

pub use target::Target;

#[cfg(target_arch="wasm32")]
pub mod wasm
{
	use std::sync::Arc;

	use crate::app::{App, AppWrapper, UserEvent};
	use wasm_bindgen::prelude::*;
	use winit::platform::web::{WindowExtWebSys, EventLoopExtWebSys};
	use winit::window::Window;

	#[wasm_bindgen(start)]
	pub fn run_wasm()
	{
		std::panic::set_hook(Box::new(console_error_panic_hook::hook));
		console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
		
		let event_loop = winit::event_loop::EventLoop::with_user_event().build().expect("Failed to create event loop");

		let proxy = Arc::new(event_loop.create_proxy());
		
		let app = AppWrapper::new(move |window: Window|
			{
				web_sys::window()
					.and_then(|win| win.document())
					.and_then(|doc| doc.body())
					.and_then(|body|
					{
						let canvas = web_sys::Element::from(window.canvas().unwrap());
						body.append_child(&canvas).ok()
					})
					.expect("Couldn't append canvas to document body.");

				let event_proxy = proxy.clone();

				wasm_bindgen_futures::spawn_local(async move
				{
					let app = App::build(wgpu::Limits::downlevel_webgl2_defaults(), window).await;
					event_proxy.send_event(UserEvent::Initialized(app)).expect("Initialized web");
				});
				None
			});

		event_loop.spawn_app(app);
	}
}