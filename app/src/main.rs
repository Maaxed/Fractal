use fractal_renderer_lib::app::{App, AppWrapper};

fn main() -> Result<(), winit::error::EventLoopError>
{
	env_logger::init();
	let event_loop = winit::event_loop::EventLoop::with_user_event().build().expect("Failed to create event loop");
	
	let mut app = AppWrapper::new(|window|
	{
		use pollster::FutureExt;
		Some(App::build(wgpu::Limits::default(), window).block_on())
	});
	
	event_loop.run_app(&mut app)
}
