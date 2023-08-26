#![no_std]
#![deny(warnings)]

use spirv_std::{spirv, Image, Sampler};
use spirv_std::glam::{Vec2, Vec4};


#[spirv(fragment)]
pub fn fragment(
    // Inputs
    input_uv: Vec2,

    // Outputs
    output_color: &mut Vec4,

    // Uniforms
    #[spirv(descriptor_set = 0, binding = 1)] sampler: &Sampler,
    #[spirv(descriptor_set = 1, binding = 1)] fractal_texture: &Image!(2D, type=f32, sampled=true),
)
{
    *output_color = fractal_texture.sample(*sampler, input_uv);
}
