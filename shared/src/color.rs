use glam::{vec3, Vec3};

use crate::math::pow;



pub fn linear_to_srgb(value: Vec3) -> Vec3
{
   vec3(component_linear_to_srgb(value.x), component_linear_to_srgb(value.y), component_linear_to_srgb(value.z))
}

fn component_linear_to_srgb(linear_component: f32) -> f32
{
    if linear_component <= 0.0031308
    {
        linear_component * 12.92
    }
    else
    {
        1.055 * pow(linear_component, 1.0 / 2.4) - 0.055
    }
}

pub fn srgb_to_linear(value: Vec3) -> Vec3
{
   vec3(component_srgb_to_linear(value.x), component_srgb_to_linear(value.y), component_srgb_to_linear(value.z))
}

fn component_srgb_to_linear(linear_component: f32) -> f32
{
    if linear_component <= 0.04045
    {
        linear_component / 12.92
    }
    else
    {
        pow((linear_component + 0.055) / 1.055, 2.4)
    }
}