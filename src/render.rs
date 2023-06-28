use wgpu::{RenderPipeline, CommandEncoder, TextureView};

use crate::Target;

pub struct Render
{
    render_pipeline: RenderPipeline,
}

impl Render
{
    pub fn new (shader_module: &wgpu::ShaderModule, target: &Target) -> Self
    {
        let pipeline_layout = target.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor
            {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = target.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor
            {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState
                {
                    module: shader_module,
                    entry_point: "vertex",
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState
                {
                    module: shader_module,
                    entry_point: "fragment",
                    targets: &[Some(target.config.format.into())],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });

        Self
        {
            render_pipeline,
        }
    }

    pub fn make_render_pass(&self, view: &TextureView, commands: &mut CommandEncoder)
    {
        let mut render_pass = commands.begin_render_pass(
            &wgpu::RenderPassDescriptor
            {
                label: None,
                color_attachments: &[Some(
                    wgpu::RenderPassColorAttachment
                    {
                        view,
                        resolve_target: None,
                        ops: wgpu::Operations
                        {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                            store: true,
                        },
                    })],
                depth_stencil_attachment: None,
            });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.draw(0..6, 0..1);
    }
}
