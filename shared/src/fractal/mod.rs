pub mod mandelbrot;

use glam::DVec2;
#[cfg(feature = "bytemuck")]
use bytemuck::NoUninit;

#[repr(u32)]
#[cfg_attr(feature = "bytemuck", derive(NoUninit))]
#[derive(Debug, Copy, Clone)]
pub enum FractalKind
{
    MandelbrotSet,
    JuliaSet,
}

pub fn compute_fractal_value(fractal_kind: FractalKind, pos: DVec2, secondary_pos: DVec2) -> f32
{
    match fractal_kind
    {
        FractalKind::MandelbrotSet => mandelbrot::mandelbrot_value(pos),
        FractalKind::JuliaSet => mandelbrot::mandelbrot_julia_set(pos, secondary_pos),
    }
}