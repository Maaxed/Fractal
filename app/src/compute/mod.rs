mod compute_shader;
mod render_shader;
mod threaded;

pub use compute_shader::*;
pub use render_shader::*;
pub use threaded::*;

use crate::Target;
use crate::app::AppData;
use crate::render::Render;

pub trait Compute: Sized + 'static
{
    fn reset(&mut self);

    fn update_before_render(&mut self, target: &Target, render: &Render, app: &mut AppData, commands: &mut wgpu::CommandEncoder);
}

pub enum AnyCompute
{
    Shader(ShaderCompute),
    Threaded(ThreadedCompute),
}

impl Compute for AnyCompute
{
    fn reset(&mut self)
    {
        match self
        {
            Self::Shader(shader) => shader.reset(),
            Self::Threaded(threaded) => threaded.reset(),
        }
    }

    fn update_before_render(&mut self, target: &Target, render: &Render, app: &mut AppData, commands: &mut wgpu::CommandEncoder)
    {
        match self
        {
            Self::Shader(shader) => shader.update_before_render(target, render, app, commands),
            Self::Threaded(threaded) => threaded.update_before_render(target, render, app, commands),
        }
    }
}