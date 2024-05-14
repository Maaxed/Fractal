use fractal_renderer_shared as shared;
use winit::dpi::PhysicalSize;


pub struct Render
{
    texture_size: PhysicalSize<u32>,
    use_double_precision: bool,
    render_pipeline: wgpu::RenderPipeline,
    instance_bind_group_layout: wgpu::BindGroupLayout,
    uniform: Uniform,
}

struct Uniform
{
    bind_group_layout: wgpu::BindGroupLayout,
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

pub struct Instance
{
    use_double_precision: bool,
    buffer: wgpu::Buffer,
    fractal_texture: wgpu::Texture,
    bind_group: wgpu::BindGroup,
}

impl Uniform
{
    fn new(target: &crate::Target, use_double_precision: bool) -> Self
    {
        let bind_group_layout = target.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor
            {
                label: None,
                entries: &[
                    wgpu::BindGroupLayoutEntry
                    {
                        binding: 0,
                        count: None,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer
                        {
                            has_dynamic_offset: false,
                            min_binding_size: None,
                            ty: wgpu::BufferBindingType::Uniform,
                        },
                    },
                    wgpu::BindGroupLayoutEntry
                    {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let uniform_data_size = if use_double_precision
        {
            std::mem::size_of::<shared::render::Uniforms64>()
        }
        else
        {
            std::mem::size_of::<shared::render::Uniforms32>()
        };
        
        let buffer = target.device.create_buffer(
            &wgpu::BufferDescriptor
            {
                label: Some("uniforms"),
                size: uniform_data_size as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

        let fractal_sampler = target.device.create_sampler(
            &wgpu::SamplerDescriptor
            {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });
        
        let bind_group = target.device.create_bind_group(
            &wgpu::BindGroupDescriptor
            {
                label: None,
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry
                    {
                        binding: 0,
                        resource: buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry
                    {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&fractal_sampler),
                    },
                ],
            });

        Self
        {
            bind_group_layout,
            buffer,
            bind_group
        }
    }
}

impl Instance
{
    fn new(target: &crate::Target, render: &Render) -> Self
    {
        let instance_data_size = if render.use_double_precision
        {
            std::mem::size_of::<shared::render::Instance64>()
        }
        else
        {
            std::mem::size_of::<shared::render::Instance32>()
        };

        let buffer = target.device.create_buffer(
            &wgpu::BufferDescriptor
            {
                label: Some("instance"),
                size: instance_data_size as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        
		let fractal_texture = target.device.create_texture(
			&wgpu::TextureDescriptor
			{
				label: Some("fractal_texture"),
				size: wgpu::Extent3d
				{
					width: render.texture_size.width,
					height: render.texture_size.height,
					depth_or_array_layers: 1,
				},
				mip_level_count: 1,
				sample_count: 1,
				dimension: wgpu::TextureDimension::D2,
				format: target.config.format,
				usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
				view_formats: &target.config.view_formats,
			}
		);

        let fractal_texture_view = fractal_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let bind_group = target.device.create_bind_group(
            &wgpu::BindGroupDescriptor
            {
                label: None,
                layout: &render.instance_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry
                    {
                        binding: 0,
                        resource: buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry
                    {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&fractal_texture_view),
                    },
                ],
            });

        Self
        {
            use_double_precision: render.use_double_precision,
            buffer,
            fractal_texture,
            bind_group,
        }
    }

    pub fn fractal_texture(&self) -> &wgpu::Texture
    {
        &self.fractal_texture
    }

    pub fn set_data(&self, queue: &wgpu::Queue, instance: &shared::render::Instance64)
    {
        if self.use_double_precision
        {
            queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(instance));
        }
        else
        {
            let instance: shared::render::Instance32 = (*instance).into();
            queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&instance));
        }
    }
}

impl Render
{
    pub fn new(target: &crate::Target, vertex_shader_module: &wgpu::ShaderModule, fragment_shader_module: &wgpu::ShaderModule, texture_size: PhysicalSize<u32>, use_double_precision: bool) -> Self
    {
		let uniform = Uniform::new(target, use_double_precision);
        
        let instance_bind_group_layout = target.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor
            {
                label: None,
                entries: &[
                    wgpu::BindGroupLayoutEntry
                    {
                        binding: 0,
                        count: None,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer
                        {
                            has_dynamic_offset: false,
                            min_binding_size: None,
                            ty: wgpu::BufferBindingType::Uniform,
                        },
                    },
                    wgpu::BindGroupLayoutEntry
                    {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture
                        {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                ],
            });

        let pipeline_layout = target.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor
            {
                label: None,
                bind_group_layouts: &[&uniform.bind_group_layout, &instance_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = target.device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor
            {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState
                {
                    module: vertex_shader_module,
                    entry_point: "vertex",
                    buffers: &[],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState
                {
                    module: fragment_shader_module,
                    entry_point: "fragment",
                    targets: &[Some(target.config.format.into())],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });

        Self
        {
            texture_size,
            use_double_precision,
            render_pipeline,
            instance_bind_group_layout,
            uniform,
        }
    }

    pub fn make_instance(&self, target: &crate::Target) -> Instance
    {
        Instance::new(target, self)
    }

    pub fn make_render_pass<'i>(&self, instances: impl IntoIterator<Item = &'i Instance>, view: &wgpu::TextureView, commands: &mut wgpu::CommandEncoder)
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
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.uniform.bind_group, &[]);

        for instance in instances
        {
            render_pass.set_bind_group(1, &instance.bind_group, &[]);
            render_pass.draw(0..6, 0..1);
        }
    }

    pub fn set_uniforms(
        &self,
        queue: &wgpu::Queue,
        uniform: &shared::render::Uniforms64
    )
    {
        if self.use_double_precision
        {
            queue.write_buffer(&self.uniform.buffer, 0, bytemuck::bytes_of(uniform));
        }
        else
        {
            let uniform: shared::render::Uniforms32 = (*uniform).into();
            queue.write_buffer(&self.uniform.buffer, 0, bytemuck::bytes_of(&uniform));
        }
    }
}
