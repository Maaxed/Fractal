pub mod escape_time_method;
pub mod mandelbrot;
pub mod multibrot;
pub mod tricorn;
pub mod burning_ship;
pub mod cos_leaf;
pub mod lyapunov;

use crate::FractalParams;
use num_traits::Pow;
use glam::{DVec2, Vec3, vec3};
#[cfg(feature = "bytemuck")]
use bytemuck::NoUninit;

#[cfg(target_arch = "spirv")]
use num_traits::Float;

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

pub fn compute_fractal_color(pos: DVec2, fractal_params: FractalParams) -> Vec3
{
    let v = compute_fractal_value(fractal_params.fractal_kind, pos, fractal_params.secondary_pos);

    if fractal_params.fractal_kind == FractalKind::Lyapunov
    {
        let y: f32 = if v >= 0.0 { 0.0 } else { v.exp().sqrt() };
        let r = y;
        let g = 1.0 - (1.0 - y).pow(0.55);
        let b = if v <= 0.0 { 0.0 } else { 1.0 - (-v).exp().pow(3.0) };
        return vec3(r, g, b);
    }

    // Gradient: black - red - yellow - white
    let threshold1 = 0.2;
    let threshold2 = 0.6;
    let r = if v > threshold1 { 1.0 } else { v / threshold1 };
    let g = if v > threshold2 { 1.0 } else if v > threshold1 { (v - threshold1) / (threshold2 - threshold1) } else { 0.0 };
    let b = if v > threshold2 { (v - threshold2) / (1.0 - threshold2) } else { 0.0 };
    vec3(r, g, b)
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