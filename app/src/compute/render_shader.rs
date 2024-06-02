use fractal_renderer_shared as shared;
use crate::app::AppData;
use crate::render::Render;
use crate::Target;
use crate::quad_cell::QuadPos;
use wgpu::{BindGroup, Buffer, CommandEncoder, RenderPipeline, Texture};
use winit::dpi::PhysicalSize;
use glam::dvec2;


pub struct ShaderRenderCompute
{
    use_double_precision: bool,
    render_pipeline: RenderPipeline,
    param_uniform_buffer: Buffer,
    bind_group: BindGroup,
    output_texture: Texture,
}

impl ShaderRenderCompute
{
    pub fn new(target: &Target, vertex_shader_module: &wgpu::ShaderModule, fragment_shader_module: &wgpu::ShaderModule, texture_size: PhysicalSize<u32>, use_double_precision: bool) -> Self
    {
        let data_size = if use_double_precision
        {
            std::mem::size_of::<shared::compute::Params64>()
        }
        else
        {
            std::mem::size_of::<shared::compute::Params32>()
        };

        let param_uniform_buffer = target.device.create_buffer(
            &wgpu::BufferDescriptor
            {
                label: Some("param_uniform"),
                size: data_size as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        
        let bind_group_layout = target.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor
            {
                label: Some("computation_render_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry
                    {
                        binding: 0,
                        count: None,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer
                        {
                            has_dynamic_offset: false,
                            min_binding_size: None,
                            ty: wgpu::BufferBindingType::Uniform,
                        },
                    }
                ],
            });

        let pipeline_layout = target.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor
            {
                label: Some("computation_render_pipeline_layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = target.device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor
            {
                label: Some("computation_render_pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState
                {
                    module: vertex_shader_module,
                    entry_point: "vertex",
                    buffers: &[],
                    //compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState
                {
                    module: fragment_shader_module,
                    entry_point: "fragment",
                    targets: &[Some(target.config.format.into())],
                    //compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });
    
        let bind_group = target.device.create_bind_group(
            &wgpu::BindGroupDescriptor
            {
                label: Some("computation_render_bind_group"),
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry
                    {
                        binding: 0,
                        resource: param_uniform_buffer.as_entire_binding(),
                    },
                ],
            });

        let output_texture = target.device.create_texture(
            &wgpu::TextureDescriptor
            {
                label: Some("output_texture"),
                size: wgpu::Extent3d
                {
                    width: texture_size.width,
                    height: texture_size.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: target.config.format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
                view_formats: &target.config.view_formats,
            }
        );
        
        Self
        {
            render_pipeline,
            param_uniform_buffer,
            use_double_precision,
            bind_group,
            output_texture
        }
    }

    fn make_computation_render_pass(&self, commands: &mut CommandEncoder)
    {
        let output_texture_view = self.output_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut render_pass = commands.begin_render_pass(
            &wgpu::RenderPassDescriptor
            {
                label: None,
                color_attachments: &[Some(
                    wgpu::RenderPassColorAttachment
                    {
                        view: &output_texture_view,
                        resolve_target: None,
                        ops: wgpu::Operations
                        {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }

    fn set_params(
        &self,
        queue: &wgpu::Queue,
        params: &shared::compute::Params64
    )
    {
        if self.use_double_precision
        {
            queue.write_buffer(&self.param_uniform_buffer, 0, bytemuck::bytes_of(params));
        }
        else
        {
            let params: shared::compute::Params32 = (*params).into();
            queue.write_buffer(&self.param_uniform_buffer, 0, bytemuck::bytes_of(&params));
        }
    }

    fn copy_output_to_texture(
        &self,
        commands: &mut wgpu::CommandEncoder,
        destination: &wgpu::Texture,
    )
    {
        commands.copy_texture_to_texture(
            wgpu::ImageCopyTexture
            {
                texture: &self.output_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyTexture
            {
                texture: destination,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            destination.size()
        );
    }

    fn compute_cell(&self, target: &Target, render: &Render, app: &mut AppData, commands: &mut wgpu::CommandEncoder, pos: QuadPos)
    {
		let cell_size = pos.cell_size();
		let cell_pos = pos.cell_bottom_left();
        
		self.set_params(&target.queue, &shared::compute::Params64
		{
			min_pos: cell_pos + dvec2(0.0, cell_size),
			max_pos: cell_pos + dvec2(cell_size, 0.0),
			fractal: app.fractal_params,
		});

        let cell = app.make_cell(target, render, pos);

		self.make_computation_render_pass(commands);
		self.copy_output_to_texture(commands, cell.fractal_texture());
    }
}

impl super::Compute for ShaderRenderCompute
{
    fn reset(&mut self)
    { }

    fn update_before_render(&mut self, target: &Target, render: &Render, app: &mut AppData, commands: &mut wgpu::CommandEncoder)
    {
        // Find new cell to load
		for pos in app.visible_cells()
		{
            if !app.is_cell_loaded(pos)
            {
                self.compute_cell(target, render, app, commands, pos);
                return;
            }
		}
    }
}
