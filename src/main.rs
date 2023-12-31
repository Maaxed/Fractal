use fractal_renderer_lib::*;
use winit::window::Window;

fn main()
{
    futures::executor::block_on(run());
}

async fn run()
{
    env_logger::init();
    let event_loop = winit::event_loop::EventLoop::new();
    
    let window: Window = Window::new(&event_loop).expect("Failed to create window");
    let target = Target::new(window, wgpu::Limits::default()).await;

    app::run_app(target, event_loop);
}
