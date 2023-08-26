#![no_std]
#![deny(warnings)]

use fractal_renderer_shared as shared;
use spirv_std::spirv;
use spirv_std::glam::{Vec2, Vec4, vec2};


const VERTICES: [(Vec2, Vec2); 6] =
[
	(vec2(0.0, 0.0), vec2(0.0, 1.0)),
	(vec2(1.0, 0.0), vec2(1.0, 1.0)),
	(vec2(0.0, 1.0), vec2(0.0, 0.0)),
	(vec2(0.0, 1.0), vec2(0.0, 0.0)),
	(vec2(1.0, 0.0), vec2(1.0, 1.0)),
	(vec2(1.0, 1.0), vec2(1.0, 0.0)),
];

#[spirv(vertex)]
pub fn vertex(
    // Inputs
    #[spirv(vertex_index)] vertex_id: i32,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] uniforms: &shared::render::Uniforms32,
    #[spirv(uniform, descriptor_set = 1, binding = 0)] instance: &shared::render::Instance32,

    // Outputs
	#[spirv(position)] output_pos: &mut Vec4,
    output_uv: &mut Vec2,
)
{
	let (corner_pos, uv) = VERTICES[vertex_id as usize];

    let pos = (instance.pos + corner_pos * instance.size - uniforms.camera_pos) * uniforms.world_to_view_scale;
    
    *output_pos = (pos, 0.0, 1.0).into();

    *output_uv = uv;
}
