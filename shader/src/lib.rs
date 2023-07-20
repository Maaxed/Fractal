#![no_std]
#![deny(warnings)]

use fractal_renderer_shared as shared;
use spirv_std::{spirv, Image, Sampler};
use spirv_std::glam::{UVec3, DVec2, UVec2, Vec2, Vec4, vec4, vec2, uvec2};

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
    #[spirv(uniform, descriptor_set = 0, binding = 0)] params : &shared::ComputeParams,

    // Outputs
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] output: &mut [u32],
)
{
    let size = uvec2(group_count.x * WORKGROUP_SIZE.x, group_count.y * WORKGROUP_SIZE.y);
    let index = id.x + id.y * size.x;

    let c = DVec2::new(id.x as f64 + 0.5, id.y as f64 + 0.5) / size.as_dvec2();
    let pos = params.min_pos + c * (params.max_pos - params.min_pos);

    let color = shared::fractal::compute_fractal_color(pos.into(), params.fractal);
    output[index as usize] = (color_to_byte(color.x) << 16) | (color_to_byte(color.y) << 8) | color_to_byte(color.z) | 0xff000000;
}


const VERTICES: [(Vec2, Vec2); 6] =
[
	(vec2(-1.0, -1.0), vec2(0.0, 1.0)),
	(vec2( 1.0, -1.0), vec2(1.0, 1.0)),
	(vec2(-1.0,  1.0), vec2(0.0, 0.0)),
	(vec2(-1.0,  1.0), vec2(0.0, 0.0)),
	(vec2( 1.0, -1.0), vec2(1.0, 1.0)),
	(vec2( 1.0,  1.0), vec2(1.0, 0.0)),
];

#[spirv(vertex)]
pub fn vertex(
    // Inputs
    #[spirv(vertex_index)] vertex_id: i32,

    // Outputs
	#[spirv(position)] output_pos: &mut Vec4,
    output_uv: &mut Vec2,
)
{
	let (pos, uv) = VERTICES[vertex_id as usize];
    
    *output_pos = vec4(
        pos.x,
        pos.y,
        0.0,
        1.0,
    );

    *output_uv = uv;
}


#[spirv(fragment)]
pub fn fragment(
    // Inputs
    input_uv: Vec2,

    // Outputs
    output_color: &mut Vec4,

    // Uniforms
    #[spirv(descriptor_set = 0, binding = 0)] fractal_texture: &Image!(2D, type=f32, sampled=true),
    #[spirv(descriptor_set = 0, binding = 1)] sampler: &Sampler,
)
{
    *output_color = fractal_texture.sample(*sampler, input_uv);
}