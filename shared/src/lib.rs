#![no_std]

use glam::DVec2;
#[cfg(feature = "bytemuck")]
use bytemuck::NoUninit;

#[repr(u32)]
#[cfg_attr(feature = "bytemuck", derive(Debug, Copy, Clone, NoUninit))]
pub enum FractalKind
{
    MandelbrotSet,
    JuliaSet
}

#[repr(C)]
#[cfg_attr(feature = "bytemuck", derive(Debug, Copy, Clone, NoUninit))]
pub struct ComputeParams
{
    pub pos: DVec2,
    pub secondary_pos: DVec2,
    pub zoom: f64,
    pub fractal_kind: FractalKind,
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
            fractal_kind: FractalKind::MandelbrotSet,
            padding: 0,
        }
    }
}
