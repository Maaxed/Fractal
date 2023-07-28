mod target;
mod compute;
mod render;
mod app;
mod quad_cell;

pub use target::Target;
pub use app::App;

fn main()
{
    futures::executor::block_on(run());
}

async fn run()
{
    env_logger::init();
    let event_loop = winit::event_loop::EventLoop::new();
    
    let target = Target::new(&event_loop).await;

    let app = App::new(target);

    app.run(event_loop);
}
