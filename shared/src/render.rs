use glam::DVec2;
#[cfg(feature = "bytemuck")]
use bytemuck::NoUninit;

#[repr(C)]
#[cfg_attr(feature = "bytemuck", derive(Debug, NoUninit))]
#[derive(Copy, Clone)]
pub struct Uniforms
{
    pub camera_pos: DVec2,
    pub world_to_view_scale: DVec2,
}

impl Default for Uniforms
{
    fn default() -> Self
    {
        Self
        {
            camera_pos: DVec2::ZERO,
            world_to_view_scale: DVec2::ONE,
        }
    }
}

#[repr(C)]
#[cfg_attr(feature = "bytemuck", derive(Debug, NoUninit))]
#[derive(Copy, Clone)]
pub struct Instance
{
    pub pos: DVec2,
    pub size: DVec2,
}

impl Default for Instance
{
    fn default() -> Self
    {
        Self
        {
            pos: DVec2::ZERO,
            size: DVec2::ONE,
        }
    }
}
