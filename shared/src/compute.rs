use glam::{Vec2 as FVec2, DVec2};
use crate::{math::{Scalar, Vec2}, fractal::{FractalParams32, FractalParams64, FractalParams}};

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
