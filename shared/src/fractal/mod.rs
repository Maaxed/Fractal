pub mod escape_time_method;
pub mod mandelbrot;
pub mod multibrot;
pub mod tricorn;
pub mod burning_ship;
pub mod cos_leaf;
pub mod lyapunov;

use glam::DVec2;
#[cfg(feature = "bytemuck")]
use bytemuck::NoUninit;

#[repr(u32)]
#[cfg_attr(feature = "bytemuck", derive(NoUninit))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FractalKind
{
    // Escape time
    MandelbrotSet,
    JuliaSet,
    Multibrot3,
    Tricorn,
    BurningShip,
    CosLeaf,

    // Other
    Lyapunov,
}

pub fn compute_fractal_value(fractal_kind: FractalKind, pos: DVec2, secondary_pos: DVec2) -> f32
{
    match fractal_kind
    {
        FractalKind::MandelbrotSet => mandelbrot::mandelbrot_value(pos),
        FractalKind::JuliaSet => mandelbrot::mandelbrot_julia_set(pos, secondary_pos),
        FractalKind::Multibrot3 => multibrot::multibrot3(pos),
        FractalKind::Tricorn => tricorn::tricorn(pos),
        FractalKind::BurningShip => burning_ship::burning_ship(pos),
        FractalKind::CosLeaf => cos_leaf::cos_leaf(pos),
        FractalKind::Lyapunov => lyapunov::lyapunov(&[false, true], pos),
    }
}