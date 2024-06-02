use fractal_renderer_lib::app::AppWrapper;

fn main() -> Result<(), winit::error::EventLoopError>
{
    env_logger::init();
    let event_loop = winit::event_loop::EventLoop::new().expect("Failed to create event loop");
    
    let app = AppWrapper::new(wgpu::Limits::default(), |_window| {});

    //event_loop.run_app(&mut app)
    app.run(event_loop)
}
