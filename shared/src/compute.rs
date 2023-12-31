use glam::{Vec2 as FVec2, DVec2, UVec2};
use num_traits::AsPrimitive;
use crate::math::*;
use crate::fractal::{FractalParams32, FractalParams64, FractalParams};

#[cfg(feature = "bytemuck")]
use bytemuck::NoUninit;


#[repr(C)]
#[cfg_attr(feature = "bytemuck", derive(NoUninit))]
#[derive(Copy, Clone)]
pub struct Params32
{
    pub min_pos: FVec2,
    pub max_pos: FVec2,
    pub fractal: FractalParams32,
}

impl Default for Params32
{
    fn default() -> Self
    {
        Self
        {
            min_pos: FVec2::splat(-2.0),
            max_pos: FVec2::splat(2.0),
            fractal: FractalParams32::default(),
        }
    }
}

#[repr(C)]
#[cfg_attr(feature = "bytemuck", derive(NoUninit))]
#[derive(Copy, Clone)]
pub struct Params64
{
    pub min_pos: DVec2,
    pub max_pos: DVec2,
    pub fractal: FractalParams64,
}

impl Default for Params64
{
    fn default() -> Self
    {
        Self
        {
            min_pos: DVec2::splat(-2.0),
            max_pos: DVec2::splat(2.0),
            fractal: FractalParams64::default(),
        }
    }
}

impl From<Params64> for Params32
{
    fn from(value: Params64) -> Self
    {
        Self
        {
            min_pos: value.min_pos.as_vec2(),
            max_pos: value.max_pos.as_vec2(),
            fractal: value.fractal.into(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Params<S: Scalar>
{
    pub min_pos: Vec2<S>,
    pub max_pos: Vec2<S>,
    pub fractal: FractalParams<S>,
}

impl From<Params32> for Params<f32>
{
    fn from(value: Params32) -> Self
    {
        Self
        {
            min_pos: value.min_pos,
            max_pos: value.max_pos,
            fractal: value.fractal.into(),
        }
    }
}

impl From<Params64> for Params<f64>
{
    fn from(value: Params64) -> Self
    {
        Self
        {
            min_pos: value.min_pos,
            max_pos: value.max_pos,
            fractal: value.fractal.into(),
        }
    }
}

pub fn color_to_byte(color: f32) -> u32
{
    (color * 255.5) as u32
}

pub fn run<S: Scalar>(id: UVec2, size: UVec2, params : Params<S>) -> u32
where u32: AsPrimitive<S>
{
    let c = Vec2::<S>::new(id.x.as_() + 0.5_f32.into(), id.y.as_() + 0.5_f32.into()) / Vec2::<S>::new(size.x.as_(), size.y.as_());
    let pos = params.min_pos + c * (params.max_pos - params.min_pos);

    let color = crate::fractal::compute_fractal_color(ComplexNumber::from_vector(pos), params.fractal);
    (color_to_byte(color.x) << 16) | (color_to_byte(color.y) << 8) | color_to_byte(color.z) | 0xff000000
}
