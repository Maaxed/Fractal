#![no_std]

use glam::Vec2;
#[cfg(feature = "bytemuck")]
use bytemuck::NoUninit;

#[repr(C)]
#[cfg_attr(feature = "bytemuck", derive(Debug, Copy, Clone, NoUninit))]
pub struct ComputeParams
{
    pub zoom: f32,
    pub padding: u32,
    pub pos: Vec2,
}

impl Default for ComputeParams
{
    fn default() -> Self
    {
        Self
        {
            zoom: 1.0,
            pos: Vec2::ZERO,
            padding: 0,
        }
    }
}
