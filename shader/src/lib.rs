#![no_std]

mod mandelbrot;
mod complex;

use spirv_std::spirv;
use spirv_std::glam::{UVec3, Vec2};

#[spirv(compute(threads(1, 1)))]
pub fn compute_mandelbrot(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(num_workgroups)] group_count: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] output: &mut [u32],
)
{
    let index = id.x + id.y * group_count.x;
    let c = Vec2::new(id.x as f32 / (group_count.x as f32 - 1.0), id.y as f32 / (group_count.y as f32 - 1.0)) * 4.0 - Vec2::ONE * 2.0;
    output[index as usize] = mandelbrot::mandelbrot_value(c);
}