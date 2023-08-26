use glam::{Vec2 as FVec2, DVec2};
#[cfg(feature = "bytemuck")]
use bytemuck::NoUninit;

use crate::math::{Scalar, Vec2};

#[repr(C)]
#[cfg_attr(feature = "bytemuck", derive(NoUninit))]
#[derive(Copy, Clone)]
pub struct Uniforms32
{
    pub camera_pos: FVec2,
    pub world_to_view_scale: FVec2,
}

impl Default for Uniforms32
{
    fn default() -> Self
    {
        Self
        {
            camera_pos: FVec2::ZERO,
            world_to_view_scale: FVec2::ONE,
        }
    }
}

#[repr(C)]
#[cfg_attr(feature = "bytemuck", derive(NoUninit))]
#[derive(Copy, Clone)]
pub struct Instance32
{
    pub pos: FVec2,
    pub size: FVec2,
}

impl Default for Instance32
{
    fn default() -> Self
    {
        Self
        {
            pos: FVec2::ZERO,
            size: FVec2::ONE,
        }
    }
}

#[repr(C)]
#[cfg_attr(feature = "bytemuck", derive(NoUninit))]
#[derive(Copy, Clone)]
pub struct Uniforms64
{
    pub camera_pos: DVec2,
    pub world_to_view_scale: DVec2,
}

impl Default for Uniforms64
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

impl From<Uniforms64> for Uniforms32
{
    fn from(value: Uniforms64) -> Self
    {
        Self
        {
            camera_pos: value.camera_pos.as_vec2(),
            world_to_view_scale: value.world_to_view_scale.as_vec2()
        }    
    }
}

#[repr(C)]
#[cfg_attr(feature = "bytemuck", derive(NoUninit))]
#[derive(Copy, Clone)]
pub struct Instance64
{
    pub pos: DVec2,
    pub size: DVec2,
}

impl Default for Instance64
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

impl From<Instance64> for Instance32
{
    fn from(value: Instance64) -> Self
    {
        Self
        {
            pos: value.pos.as_vec2(),
            size: value.size.as_vec2()
        }    
    }
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct Uniforms<S: Scalar>
{
    pub camera_pos: Vec2<S>,
    pub world_to_view_scale: Vec2<S>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Instance<S: Scalar>
{
    pub pos: Vec2<S>,
    pub size: Vec2<S>,
}

impl From<Uniforms32> for Uniforms<f32>
{
    fn from(value: Uniforms32) -> Self
    {
        Self
        {
            camera_pos: value.camera_pos,
            world_to_view_scale: value.world_to_view_scale,
        }
    }
}

impl From<Uniforms64> for Uniforms<f64>
{
    fn from(value: Uniforms64) -> Self
    {
        Self
        {
            camera_pos: value.camera_pos,
            world_to_view_scale: value.world_to_view_scale,
        }
    }
}

impl From<Instance32> for Instance<f32>
{
    fn from(value: Instance32) -> Self
    {
        Self
        {
            pos: value.pos,
            size: value.size,
        }
    }
}

impl From<Instance64> for Instance<f64>
{
    fn from(value: Instance64) -> Self
    {
        Self
        {
            pos: value.pos,
            size: value.size,
        }
    }
}
