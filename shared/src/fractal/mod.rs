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

    // Other
    Newton3,
    Lyapunov,
}

impl FractalKind
{
    pub fn default_iteration_limit(&self) -> u32
    {
        use FractalKind::*;
        
        match self
        {
            MandelbrotSet => mandelbrot::ITERATION_COUNT,
            Multibrot3 => multibrot::ITERATION_COUNT,
            Tricorn => tricorn::ITERATION_COUNT,
            BurningShip => burning_ship::ITERATION_COUNT,
            CosLeaf => cos_leaf::ITERATION_COUNT,
            Newton3 => newton::ITERATION_COUNT,
            Lyapunov => lyapunov::ITERATION_COUNT,
        }
    }
}

#[repr(u32)]
#[cfg_attr(feature = "bytemuck", derive(NoUninit))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FractalVariation
{
    Normal,
    JuliaSet,
}

#[repr(u32)]
#[cfg_attr(feature = "bytemuck", derive(NoUninit))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RenderTechnique
{
    Normal,
    OrbitTrapPoint,
    OrbitTrapCross,
    NormalMap,
}

#[repr(u32)]
#[cfg_attr(feature = "bytemuck", derive(NoUninit))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ColorPalette
{
    Default,
    Flames,
    Temperature,
}

#[repr(C)]
#[cfg_attr(feature = "bytemuck", derive(NoUninit))]
#[derive(Copy, Clone)]
pub struct FractalParams32
{
    pub secondary_pos: Complex32,
    pub fractal_kind: FractalKind,
    pub variation: FractalVariation,
    pub render_technique: RenderTechnique,
    pub iteration_limit: u32,
    pub color_palette: ColorPalette,
    pub color_frequency: f32,
}

impl Default for FractalParams32
{
    fn default() -> Self
    {
        Self
        {
            secondary_pos: Complex32::ZERO,
            fractal_kind: FractalKind::MandelbrotSet,
            variation: FractalVariation::Normal,
            render_technique: RenderTechnique::Normal,
            iteration_limit: 0,
            color_palette: ColorPalette::Default,
            color_frequency: 1.0,
        }
    }
}

#[repr(C)]
#[cfg_attr(feature = "bytemuck", derive(NoUninit))]
#[derive(Copy, Clone)]
pub struct FractalParams64
{
    pub secondary_pos: Complex64,
    pub fractal_kind: FractalKind,
    pub variation: FractalVariation,
    pub render_technique: RenderTechnique,
    pub iteration_limit: u32,
    pub color_palette: ColorPalette,
    pub color_frequency: f32,
    padding0: u32,
    padding1: u32,
}

impl Default for FractalParams64
{
    fn default() -> Self
    {
        Self
        {
            secondary_pos: Complex64::ZERO,
            fractal_kind: FractalKind::MandelbrotSet,
            variation: FractalVariation::Normal,
            render_technique: RenderTechnique::Normal,
            iteration_limit: FractalKind::MandelbrotSet.default_iteration_limit(),
            color_palette: ColorPalette::Default,
            color_frequency: 1.0,
            padding0: 0,
            padding1: 0,
        }
    }
}

impl From<FractalParams64> for FractalParams32
{
    fn from(value: FractalParams64) -> Self
    {
        Self
        {
            secondary_pos: value.secondary_pos.to_complex32(),
            fractal_kind: value.fractal_kind,
            variation: value.variation,
            render_technique: value.render_technique,
            iteration_limit: value.iteration_limit,
            color_palette: value.color_palette,
            color_frequency: value.color_frequency,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FractalParams<S: Scalar>
{
    pub secondary_pos: Complex<S>,
    pub fractal_kind: FractalKind,
    pub variation: FractalVariation,
    pub render_technique: RenderTechnique,
    pub iteration_limit: u32,
    pub color_palette: ColorPalette,
    pub color_frequency: f32,
}

impl From<FractalParams32> for FractalParams<f32>
{
    fn from(value: FractalParams32) -> Self
    {
        Self
        {
            secondary_pos: value.secondary_pos,
            fractal_kind: value.fractal_kind,
            variation: value.variation,
            render_technique: value.render_technique,
            iteration_limit: value.iteration_limit,
            color_palette: value.color_palette,
            color_frequency: value.color_frequency,
        }
    }
}

impl From<FractalParams64> for FractalParams<f64>
{
    fn from(value: FractalParams64) -> Self
    {
        Self
        {
            secondary_pos: value.secondary_pos,
            fractal_kind: value.fractal_kind,
            variation: value.variation,
            render_technique: value.render_technique,
            iteration_limit: value.iteration_limit,
            color_palette: value.color_palette,
            color_frequency: value.color_frequency,
        }
    }
}

pub fn compute_fractal_color<S: Scalar>(pos: Complex<S>, params: FractalParams<S>) -> Vec3
{
    let res = match params.fractal_kind
    {
        FractalKind::MandelbrotSet => mandelbrot::mandelbrot_value(pos, params),
        FractalKind::Multibrot3 => multibrot::multibrot3(pos, params),
        FractalKind::Tricorn => tricorn::tricorn(pos, params),
        FractalKind::BurningShip => burning_ship::burning_ship(pos, params),
        FractalKind::CosLeaf => cos_leaf::cos_leaf(pos, params),
        FractalKind::Newton3 => return newton::newton3(pos, params),
        FractalKind::Lyapunov =>
        {
            let v = lyapunov::lyapunov::<S, 2>(&[false, true], pos.to_vector(), params.iteration_limit);
            let y: f32 = if v >= 0.0 { 0.0 } else { sqrt(exp(v)) };
            let r = y;
            let g = 1.0 - pow(1.0 - y, 0.55);
            let b = if v <= 0.0 { 0.0 } else { 1.0 - pow(exp(-v), 3.0) };
            return vec3(r, g, b);
        },
    };

    match res
    {
        EscapeResult::StayedInside => vec3(0.0, 0.0, 0.0),
        EscapeResult::Escaped(v) =>
        {
            if params.render_technique == RenderTechnique::NormalMap
            {
                let g = v * 0.9 + 0.1;
                Vec3::splat(g)
            }
            else
            {
                let v = ln(v);

                let v = v * params.color_frequency;

                match params.color_palette
                {
                    // orange purple blue cyan white yellow
                    ColorPalette::Default => sample_palette(v, &[vec3(1.0, 0.5, 0.0), vec3(0.5, 0.0, 1.0), vec3(0.0, 0.0, 1.0), vec3(0.0, 1.0, 1.0), vec3(1.0, 1.0, 1.0), vec3(1.0, 1.0, 0.0)]),
                    // yellow red black red yellow white
                    ColorPalette::Flames => sample_palette(v, &[vec3(1.0, 1.0, 0.0), vec3(1.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0), vec3(1.0, 0.0, 0.0), vec3(1.0, 1.0, 0.0), vec3(1.0, 1.0, 1.0)]),
                    // cyan purple black red yellow white
                    ColorPalette::Temperature => sample_palette(v, &[vec3(0.0, 1.0, 1.0), vec3(0.5, 0.0, 1.0), vec3(0.0, 0.0, 0.0), vec3(1.0, 0.0, 0.0), vec3(1.0, 1.0, 0.0), vec3(1.0, 1.0, 1.0)]),
                }
            }
        },
    }
}

fn sample_palette<const N: usize>(v: f32, palette: &[Vec3; N]) -> Vec3
{
    let v = rem_euclid(v, N as f32);

    let i = floor(v) as usize;
    let t = v % 1.0;
    let t = t*t * (3.0 - 2.0*t);
    let c1 = palette[i];
    let c2 = palette[(i+1) % N];
    c1 + (c2 - c1) * t
}