mod shader;

pub use shader::ShaderCompute;
use crate::Target;
use crate::app::AppData;
use crate::render::Render;

pub trait Compute: Sized + 'static
{
    fn reset(&mut self);

    fn update_before_render(&mut self, target: &Target, render: &Render, app: &mut AppData, commands: &mut wgpu::CommandEncoder);
}
