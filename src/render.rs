use winit::dpi::PhysicalSize;


pub struct Render
{
	fixed: Fixed,
    dynamic: Dynamic,
}

struct Fixed
{
    render_pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    fractal_sampler: wgpu::Sampler,
    uniform_buffer: wgpu::Buffer,
}

struct Dynamic
{
    bind_group: wgpu::BindGroup,
	fractal_texture: wgpu::Texture,
}

impl Fixed
{
    fn new(target: &crate::Target, shader_module: &wgpu::ShaderModule) -> Self
    {
        let uniform_buffer = target.device.create_buffer(
            &wgpu::BufferDescriptor
            {
                label: Some("uniforms"),
                size: std::mem::size_of::<fractal_renderer_shared::RenderUniforms>() as u64,
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
        
        let bind_group_layout = target.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor
            {
                label: None,
                entries: &[
                    wgpu::BindGroupLayoutEntry
                    {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture
                        {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
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
                    wgpu::BindGroupLayoutEntry
                    {
                        binding: 2,
                        count: None,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer
                        {
                            has_dynamic_offset: false,
                            min_binding_size: None,
                            ty: wgpu::BufferBindingType::Uniform,
                        },
                    },
                ],
            });

        let pipeline_layout = target.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor
            {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = target.device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor
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
            fractal_sampler,
            bind_group_layout,
            render_pipeline,
            uniform_buffer,
        }
    }
}

impl Dynamic
{
    fn new(target: &crate::Target, fixed: &Fixed, texture_size: PhysicalSize<u32>) -> Self
    {
		let fractal_texture = target.device.create_texture(
			&wgpu::TextureDescriptor
			{
				label: Some("fractal_texture"),
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
				usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
				view_formats: &target.config.view_formats,
			}
		);

        let fractal_texture_view = fractal_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let bind_group = target.device.create_bind_group(
            &wgpu::BindGroupDescriptor
            {
                label: None,
                layout: &fixed.bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry
                    {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&fractal_texture_view),
                    },
                    wgpu::BindGroupEntry
                    {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&fixed.fractal_sampler),
                    },
                    wgpu::BindGroupEntry
                    {
                        binding: 2,
                        resource: fixed.uniform_buffer.as_entire_binding(),
                    },
                ],
            });

        Self
        {
            fractal_texture,
            bind_group,
        }
    }
}

impl Render
{
    pub fn new(target: &crate::Target, shader_module: &wgpu::ShaderModule, texture_size: PhysicalSize<u32>) -> Self
    {
		let fixed = Fixed::new(target, shader_module);
        let dynamic = Dynamic::new(target, &fixed, texture_size);

        Self
        {
            fixed,
            dynamic,
        }
    }

    pub fn resize(&mut self, target: &crate::Target, new_size: PhysicalSize<u32>)
	{
        self.dynamic = Dynamic::new(target, &self.fixed, new_size);
    }

    pub fn fractal_texture(&self) -> &wgpu::Texture
    {
        &self.dynamic.fractal_texture
    }

    pub fn make_render_pass(&self, view: &wgpu::TextureView, commands: &mut wgpu::CommandEncoder)
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
                            store: true,
                        },
                    })],
                depth_stencil_attachment: None,
            });
        render_pass.set_pipeline(&self.fixed.render_pipeline);
        render_pass.set_bind_group(0, &self.dynamic.bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }

    pub fn set_params(
        &self,
        queue: &wgpu::Queue,
        params: &fractal_renderer_shared::RenderUniforms
    )
    {
        queue.write_buffer(&self.fixed.uniform_buffer, 0, bytemuck::bytes_of(params));
    }
}
