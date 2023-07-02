#![no_std]

pub mod complex;
pub mod fractal;

use glam::DVec2;
#[cfg(feature = "bytemuck")]
use bytemuck::NoUninit;



#[repr(C)]
#[cfg_attr(feature = "bytemuck", derive(Debug, NoUninit))]
#[derive(Copy, Clone)]
pub struct ComputeParams
{
    pub pos: DVec2,
    pub secondary_pos: DVec2,
    pub zoom: f64,
    pub fractal_kind: fractal::FractalKind,
    pub padding: u32,
}

impl Default for ComputeParams
{
    fn default() -> Self
    {
        Self
        {
            pos: DVec2::ZERO,
            secondary_pos: DVec2::ZERO,
            zoom: 1.0,
            fractal_kind: fractal::FractalKind::MandelbrotSet,
            padding: 0,
        }
    }
}
