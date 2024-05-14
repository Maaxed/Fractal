use fractal_renderer_lib::*;
use winit::window::{Window, WindowAttributes};

fn main()
{
    futures::executor::block_on(run());
}

async fn run()
{
    env_logger::init();
    let event_loop = winit::event_loop::EventLoop::new().expect("Failed to create event loop");
    
    let window_attributes = WindowAttributes::default().with_title("Fractal");
    let window: Window = event_loop.create_window(window_attributes).expect("Failed to create window");
    let target = Target::new(&window, wgpu::Limits::default()).await;

    app::run_app(target, event_loop);
}
