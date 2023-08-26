#![no_std]
#![deny(warnings)]

use fractal_renderer_shared as shared;
use shared::math::ComplexNumber;
use spirv_std::spirv;
use spirv_std::glam::{UVec3, DVec2, UVec2, uvec2};

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

    let c = DVec2::new(id.x as f64 + 0.5, id.y as f64 + 0.5) / size.as_dvec2();
    let pos = params.min_pos + c * (params.max_pos - params.min_pos);

    let color = shared::fractal::compute_fractal_color(ComplexNumber::from_vector(pos), params.fractal.into());
    output[index as usize] = (color_to_byte(color.x) << 16) | (color_to_byte(color.y) << 8) | color_to_byte(color.z) | 0xff000000;
}
