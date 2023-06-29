use wgpu::{ComputePipeline, Buffer, BindGroup, CommandEncoder, BufferAddress, BufferView};

use crate::Target;

pub struct Compute
{
    pub size: u32,
    storage_buffer: Buffer,
    bind_group: BindGroup,
    compute_pipeline: ComputePipeline,
}

impl Compute
{
    pub fn new(shader_module: &wgpu::ShaderModule, target: &Target, size: u32) -> Self
    {
        let mem_size = (size * size * std::mem::size_of::<u32>() as u32) as wgpu::BufferAddress;

        let bind_group_layout = target.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor
            {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry
                {
                    binding: 0,
                    count: None,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer
                    {
                        has_dynamic_offset: false,
                        min_binding_size: None,
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                    },
                }],
            });

        let pipeline_layout = target.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor
            {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let storage_buffer = target.device.create_buffer(
            &wgpu::BufferDescriptor
            {
                label: Some("output"),
                size: mem_size,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            });
    
        let bind_group = target.device.create_bind_group(
            &wgpu::BindGroupDescriptor
            {
                label: None,
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry
                {
                    binding: 0,
                    resource: storage_buffer.as_entire_binding(),
                }],
            });

        let compute_pipeline = target.device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor
            {
                label: None,
                layout: Some(&pipeline_layout),
                module: shader_module,
                entry_point: "compute_mandelbrot",
            });

        Self
        {
            size,
            storage_buffer,
            bind_group,
            compute_pipeline
        }
    }

    pub fn buffer(&self) -> &Buffer
    {
        &self.storage_buffer
    }

    pub fn make_compute_pass(&self, commands: &mut CommandEncoder)
    {
        let mut compute_pass = commands.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        compute_pass.set_bind_group(0, &self.bind_group, &[]);
        compute_pass.set_pipeline(&self.compute_pipeline);
        compute_pass.dispatch_workgroups(self.size, self.size, 1);
    }

    pub fn copy_buffer(
        &self,
        commands: &mut CommandEncoder,
        destination: &Buffer,
        destination_offset: BufferAddress,
    )
    {
        commands.copy_buffer_to_buffer(
            self.buffer(),
            0,
            destination,
            destination_offset,
            self.buffer().size(),
        );
    }

    pub fn copy_buffer_to_texture(
        &self,
        commands: &mut wgpu::CommandEncoder,
        destination: &wgpu::Texture,
    )
    {
        commands.copy_buffer_to_texture(
            wgpu::ImageCopyBuffer
            {
                buffer: self.buffer(),
                layout: wgpu::ImageDataLayout
                {
                    offset: 0,
                    bytes_per_row: Some(self.size * std::mem::size_of::<u32>() as u32),
                    rows_per_image: None
                }
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

    pub fn read_buffer(&self, buffer: &BufferView) -> Vec<u32>
    {
        buffer
            .chunks_exact(4)
            .map(|bytes| u32::from_ne_bytes(bytes.try_into().unwrap()))
            .collect::<Vec<_>>()
    }
}