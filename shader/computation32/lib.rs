#![no_std]
#![deny(warnings)]

use fractal_renderer_shared as shared;
use shared::color::srgb_to_linear;
use spirv_std::spirv;
use spirv_std::glam::{Vec2, Vec4, vec2};


const VERTICES: [(Vec2, Vec2); 6] =
[
	(vec2(-1.0,-1.0), vec2(0.0, 1.0)),
	(vec2( 1.0,-1.0), vec2(1.0, 1.0)),
	(vec2(-1.0, 1.0), vec2(0.0, 0.0)),
	(vec2(-1.0, 1.0), vec2(0.0, 0.0)),
	(vec2( 1.0,-1.0), vec2(1.0, 1.0)),
	(vec2( 1.0, 1.0), vec2(1.0, 0.0)),
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
    
    *output_pos = (pos, 0.0, 1.0).into();

    *output_uv = uv;
}


#[spirv(fragment)]
pub fn fragment(
    // Inputs
    input_uv: Vec2,

    // Outputs
    output_color: &mut Vec4,

    // Uniforms
    #[spirv(uniform, descriptor_set = 0, binding = 0)] params: &shared::compute::Params32,
)
{
    *output_color = (srgb_to_linear(shared::compute::run_uv(input_uv, (*params).into())), 1.0).into();
}
