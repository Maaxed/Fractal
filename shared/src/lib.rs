#![no_std]

use glam::DVec2;
#[cfg(feature = "bytemuck")]
use bytemuck::NoUninit;

#[repr(C)]
#[cfg_attr(feature = "bytemuck", derive(Debug, Copy, Clone, NoUninit))]
pub struct ComputeParams
{
    pub pos: DVec2,
    pub zoom: f64,
    pub padding: f64,
}

impl Default for ComputeParams
{
    fn default() -> Self
    {
        Self
        {
            pos: DVec2::ZERO,
            zoom: 1.0,
            padding: 0.0
        }
    }
}
