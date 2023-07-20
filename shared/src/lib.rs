#![no_std]
#![deny(warnings)]

pub mod complex;
pub mod fractal;

use complex::Complex;
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
    pub fractal: FractalParams,
}

#[repr(C)]
#[cfg_attr(feature = "bytemuck", derive(Debug, NoUninit))]
#[derive(Copy, Clone)]
pub struct FractalParams
{
    pub secondary_pos: Complex,
    pub fractal_kind: fractal::FractalKind,
    padding0: u32,
    padding1: u32,
    padding2: u32,
}

impl Default for ComputeParams
{
    fn default() -> Self
    {
        Self
        {
            min_pos: DVec2::splat(-2.0),
            max_pos: DVec2::splat(2.0),
            fractal: FractalParams::default(),
        }
    }
}

impl Default for FractalParams
{
    fn default() -> Self
    {
        Self
        {
            secondary_pos: Complex::ZERO,
            fractal_kind: fractal::FractalKind::MandelbrotSet,
            padding0: Default::default(),
            padding1: Default::default(),
            padding2: Default::default(),
        }
    }
}
