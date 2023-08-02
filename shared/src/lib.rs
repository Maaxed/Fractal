#![no_std]
#![deny(warnings)]

pub mod complex;
pub mod fractal;
pub mod render;

use glam::DVec2;
#[cfg(feature = "bytemuck")]
use bytemuck::NoUninit;


#[repr(C)]
#[cfg_attr(feature = "bytemuck", derive(Debug, NoUninit))]
#[derive(Copy, Clone)]
pub struct ComputeParams
{
    pub min_pos: DVec2,
    pub max_pos: DVec2,
    pub fractal: fractal::FractalParams,
}

impl Default for ComputeParams
{
    fn default() -> Self
    {
        Self
        {
            min_pos: DVec2::splat(-2.0),
            max_pos: DVec2::splat(2.0),
            fractal: fractal::FractalParams::default(),
        }
    }
}
