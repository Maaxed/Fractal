pub mod escape_time_method;
pub mod mandelbrot;
pub mod multibrot;
pub mod tricorn;
pub mod burning_ship;
pub mod cos_leaf;
pub mod newton;
pub mod lyapunov;

use crate::math::*;
use glam::{Vec3, vec3};
#[cfg(feature = "bytemuck")]
use bytemuck::NoUninit;
use self::escape_time_method::EscapeResult;

#[repr(u32)]
#[cfg_attr(feature = "bytemuck", derive(NoUninit))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FractalKind
{
    // Escape time
    MandelbrotSet,
    Multibrot3,
    Tricorn,
    BurningShip,
    CosLeaf,
    MandelbrotNormal,

    // Other
    Newton3,
    Lyapunov,
}

#[repr(u32)]
#[cfg_attr(feature = "bytemuck", derive(NoUninit))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FractalVariation
{
    Normal,
    JuliaSet,
}

#[repr(C)]
#[cfg_attr(feature = "bytemuck", derive(Debug, NoUninit))]
#[derive(Copy, Clone)]
pub struct FractalParams
{
    pub secondary_pos: Complex,
    pub fractal_kind: FractalKind,
    pub variation: FractalVariation,
    padding0: u32,
    padding1: u32,
}

impl Default for FractalParams
{
    fn default() -> Self
    {
        Self
        {
            secondary_pos: Complex::ZERO,
            fractal_kind: FractalKind::MandelbrotSet,
            variation: FractalVariation::Normal,
            padding0: Default::default(),
            padding1: Default::default(),
        }
    }
}

pub fn compute_fractal_color(pos: Complex, params: FractalParams) -> Vec3
{
    let res = match params.fractal_kind
    {
        FractalKind::MandelbrotSet => mandelbrot::mandelbrot_value(pos, params.into()),
        FractalKind::Multibrot3 => multibrot::multibrot3(pos, params.into()),
        FractalKind::Tricorn => tricorn::tricorn(pos, params.into()),
        FractalKind::BurningShip => burning_ship::burning_ship(pos, params.into()),
        FractalKind::CosLeaf => cos_leaf::cos_leaf(pos, params.into()),
        FractalKind::MandelbrotNormal =>
        {
            let res = mandelbrot::mandelbrot_value_normal(pos, params.into());

            return match res
            {
                EscapeResult::StayedInside => vec3(0.0, 0.0, 0.0),
                EscapeResult::Escaped(v) =>
                {
                    let g = v * 0.9 + 0.1;
                    Vec3::splat(g)
                },
            };
        },
        FractalKind::Newton3 => return newton::newton3(pos, params),
        FractalKind::Lyapunov =>
        {
            let v = lyapunov::lyapunov(&[false, true], pos.into());
            let y: f32 = if v >= 0.0 { 0.0 } else { sqrt(exp(v)) };
            let r = y;
            let g = 1.0 - pow(1.0 - y, 0.55);
            let b = if v <= 0.0 { 0.0 } else { 1.0 - pow(exp(-v), 3.0) };
            return vec3(r, g, b);
        },
    };

    /*
    // Gradient: black - red - yellow - white
    let threshold1 = 0.2;
    let threshold2 = 0.6;
    let r = if v > threshold1 { 1.0 } else { v / threshold1 };
    let g = if v > threshold2 { 1.0 } else if v > threshold1 { (v - threshold1) / (threshold2 - threshold1) } else { 0.0 };
    let b = if v > threshold2 { (v - threshold2) / (1.0 - threshold2) } else { 0.0 };

    // Gradient: black - red - yellow - black
    let threshold1 = 0.2;
    let threshold2 = 0.6;
    let r = if v > threshold2 { 1.0 - (v - threshold2) / (1.0 - threshold2) } else if v > threshold1 { 1.0 } else { v / threshold1 };
    let g = if v > threshold2 { 1.0 - (v - threshold2) / (1.0 - threshold2) } else if v > threshold1 { (v - threshold1) / (threshold2 - threshold1) } else { 0.0 };
    let b = 0.0;
    */
    match res
    {
        EscapeResult::StayedInside => vec3(0.0, 0.0, 0.0),
        EscapeResult::Escaped(v) =>
        {
            let v = ln(v + 1.0);

            //Gradient: orange - purple - blue - cyan - white - yellow
            let palette = [vec3(1.0, 0.5, 0.0), vec3(0.5, 0.0, 1.0), vec3(0.0, 0.0, 1.0), vec3(0.0, 1.0, 1.0), vec3(1.0, 1.0, 1.0), vec3(1.0, 1.0, 0.0), vec3(1.0, 0.5, 0.0)];
            let v = v % (palette.len() - 1) as f32;

            let i = floor(v) as usize;
            let t = v % 1.0;
            let c1 = palette[i];
            let c2 = palette[i+1];
            c1 + (c2 - c1) * t
        },
    }
}
