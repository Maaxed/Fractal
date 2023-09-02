#![no_std]
#![deny(warnings)]

use fractal_renderer_shared as shared;
use spirv_std::spirv;
use spirv_std::glam::{UVec3, UVec2, uvec2, Vec3Swizzles};

pub fn color_to_byte(color: f32) -> u32
{
    (color * 255.5) as u32
}

const WORKGROUP_SIZE: UVec2 = uvec2(16, 16);
#[spirv(compute(threads(16, 16)))]
pub fn compute_mandelbrot(
    // Inputs
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(num_workgroups)] group_count: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] params : &shared::compute::Params64,

    // Outputs
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] output: &mut [u32],
)
{
    let size = uvec2(group_count.x * WORKGROUP_SIZE.x, group_count.y * WORKGROUP_SIZE.y);
    let index = id.x + id.y * size.x;

    output[index as usize] = shared::compute::run(id.xy(), size, (*params).into());
}
