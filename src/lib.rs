mod target;
mod compute;
mod render;
mod quad_cell;
pub mod app;

pub use target::Target;

#[cfg(target_arch="wasm32")]
pub mod wasm
{
    use super::*;

    use wasm_bindgen::prelude::*;
    use winit::dpi::PhysicalSize;
    use winit::platform::web::WindowExtWebSys;
    use winit::window::Window;

    #[wasm_bindgen(start)]
    pub async fn run_wasm()
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("Couldn't initialize logger");
        
        let event_loop = winit::event_loop::EventLoop::new();
        
    	let window: Window = Window::new(&event_loop).expect("Failed to create window");

        window.set_inner_size(PhysicalSize::new(450, 400));
        
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body|
            {
                let canvas = web_sys::Element::from(window.canvas());
                body.append_child(&canvas).ok()
            })
            .expect("Couldn't append canvas to document body.");

        let target = Target::new(window, wgpu::Limits::downlevel_webgl2_defaults()).await;

        app::run_app(target, event_loop);
    }
}