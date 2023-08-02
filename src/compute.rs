use wgpu::{ComputePipeline, Buffer, BindGroup, CommandEncoder};
use winit::dpi::PhysicalSize;

use crate::Target;

pub struct Compute
{
    fixed: Fixed,
    dynamic: Dynamic,
}

struct Fixed
{
    workgroup_size: glam::UVec2,
    compute_pipeline: ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    params_buffer: Buffer,
}

struct Dynamic
{
    size: PhysicalSize<u32>,
    bind_group: BindGroup,
    output_buffer: Buffer,
}

impl Fixed
{
    fn new(target: &Target, shader_module: &wgpu::ShaderModule, workgroup_size: glam::UVec2) -> Self
    {
        let params_buffer = target.device.create_buffer(
            &wgpu::BufferDescriptor
            {
                label: Some("params"),
                size: std::mem::size_of::<fractal_renderer_shared::ComputeParams>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        
        let bind_group_layout = target.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor
            {
                label: Some("compute_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry
                    {
                        binding: 0,
                        count: None,
                        visibility: wgpu::ShaderStages::COMPUTE,
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
                        count: None,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer
                        {
                            has_dynamic_offset: false,
                            min_binding_size: None,
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                        },
                    }
                ],
            });

        let pipeline_layout = target.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor
            {
                label: Some("compute_pipeline_layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let compute_pipeline = target.device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor
            {
                label: Some("compute_pipeline"),
                layout: Some(&pipeline_layout),
                module: shader_module,
                entry_point: "compute_mandelbrot",
            });
        
        Self
        {
            workgroup_size,
            compute_pipeline,
            bind_group_layout,
            params_buffer,
        }
    }
}

impl Dynamic
{
    fn new(target: &Target, fixed: &Fixed, texture_size: PhysicalSize<u32>) -> Self
    {
        let size = PhysicalSize
        {
            width: wgpu::util::align_to(texture_size.width, fixed.workgroup_size.x.max(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT / std::mem::size_of::<u32>() as u32)),
            height: wgpu::util::align_to(texture_size.height, fixed.workgroup_size.y)
        };
        
        let mem_size = (size.width * size.height * std::mem::size_of::<u32>() as u32) as wgpu::BufferAddress;
        
        let output_buffer = target.device.create_buffer(
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
                label: Some("compute_bind_group"),
                layout: &fixed.bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry
                    {
                        binding: 0,
                        resource: fixed.params_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry
                    {
                        binding: 1,
                        resource: output_buffer.as_entire_binding(),
                    }
                ],
            });

        Self
        {
            size,
            bind_group,
            output_buffer,
        }
    }
}

impl Compute
{
    pub fn new(target: &Target, shader_module: &wgpu::ShaderModule, workgroup_size: glam::UVec2, texture_size: PhysicalSize<u32>) -> Self
    {
        let fixed = Fixed::new(target, shader_module, workgroup_size);
        let dynamic = Dynamic::new(target, &fixed, texture_size);

        Self
        {
            fixed,
            dynamic,
        }
    }

    pub fn buffer(&self) -> &Buffer
    {
        &self.dynamic.output_buffer
    }

    pub fn make_compute_pass(&self, commands: &mut CommandEncoder)
    {
        let mut compute_pass = commands.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        compute_pass.set_bind_group(0, &self.dynamic.bind_group, &[]);
        compute_pass.set_pipeline(&self.fixed.compute_pipeline);
        compute_pass.dispatch_workgroups(self.dynamic.size.width / self.fixed.workgroup_size.x, self.dynamic.size.height / self.fixed.workgroup_size.y, 1);
    }

    pub fn set_params(
        &self,
        queue: &wgpu::Queue,
        params: &fractal_renderer_shared::ComputeParams
    )
    {
        queue.write_buffer(&self.fixed.params_buffer, 0, bytemuck::bytes_of(params));
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
                    bytes_per_row: Some(self.dynamic.size.width * std::mem::size_of::<u32>() as u32),
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
}