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
    use crate::app::AppWrapper;
    use wasm_bindgen::prelude::*;
    use winit::platform::web::WindowExtWebSys;

    #[wasm_bindgen(start)]
    pub fn run_wasm()
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("Couldn't initialize logger");
        
        let event_loop = winit::event_loop::EventLoop::new().expect("Failed to create event loop");
        
        let app = AppWrapper::new(wgpu::Limits::downlevel_webgl2_defaults(),
            |window|
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
            });

        //event_loop.run_app(&mut app).expect("Error while running the app");
        app.run(event_loop).expect("Error while running the app");
    }
}