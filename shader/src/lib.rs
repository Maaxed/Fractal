#![no_std]

mod mandelbrot;
mod complex;

use spirv_std::spirv;
use spirv_std::glam::UVec3;

#[spirv(compute(threads(1, 1)))]
pub fn compute_mandelbrot(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] output: &mut [u32],
)
{
    output[id.x as usize] = id.x;
}