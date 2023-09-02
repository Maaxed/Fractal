mod shader;
mod threaded;

pub use shader::*;
pub use threaded::*;

use crate::Target;
use crate::app::AppData;
use crate::render::Render;

pub trait Compute: Sized + 'static
{
    fn reset(&mut self);

    fn update_before_render(&mut self, target: &Target, render: &Render, app: &mut AppData, commands: &mut wgpu::CommandEncoder);
}
