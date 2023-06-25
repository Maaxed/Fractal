
const SHADER_CODE: &[u8] = include_bytes!(env!("fractal_renderer_shader.spv"));

fn main()
{
    futures::executor::block_on(run());
}

async fn run()
{
    let (device, queue) =
    {
        let backends = wgpu::util::backend_bits_from_env().unwrap_or(wgpu::Backends::PRIMARY);
        let dx12_shader_compiler = wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends, dx12_shader_compiler });
        let adapter = wgpu::util::initialize_adapter_from_env_or_default(&instance, backends, None)
            .await
            .expect("Failed to find an appropriate adapter");

        adapter
            .request_device(
                &wgpu::DeviceDescriptor
                {
                    label: None,
                    features: wgpu::Features::TIMESTAMP_QUERY,
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device")
    };

    let shader_module = device.create_shader_module(
        wgpu::ShaderModuleDescriptor
        {
            label: None,
            source: wgpu::util::make_spirv(SHADER_CODE),
        });

    let bind_group_layout = device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor
        {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
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

    let pipeline_layout = device.create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor
        {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

    let compute_pipeline = device.create_compute_pipeline(
        &wgpu::ComputePipelineDescriptor
        {
            label: None,
            layout: Some(&pipeline_layout),
            module: &shader_module,
            entry_point: "compute_mandelbrot",
        });
    
    let size = 64;
    
    let readback_buffer = device.create_buffer(
        &wgpu::BufferDescriptor
        {
            label: None,
            size: size as wgpu::BufferAddress,
            // Can be read to the CPU, and can be copied from the shader's storage buffer
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

    let storage_buffer = device.create_buffer(
        &wgpu::BufferDescriptor
        {
            label: Some("Output"),
            size: size as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
    
    let bind_group = device.create_bind_group(
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
    
    let mut commands = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    {
        let mut compute_pass = commands.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.dispatch_workgroups(size, 1, 1);
    }

    commands.copy_buffer_to_buffer(
        &storage_buffer,
        0,
        &readback_buffer,
        0,
        size as wgpu::BufferAddress,
    );
    
    queue.submit(Some(commands.finish()));

    let buffer_slice = readback_buffer.slice(..);
    buffer_slice.map_async(wgpu::MapMode::Read, |r| r.unwrap());
    device.poll(wgpu::Maintain::Wait);

    let result = buffer_slice.get_mapped_range()
        .chunks_exact(4)
        .map(|b| u32::from_ne_bytes(b.try_into().unwrap()))
        .collect::<Vec<_>>();

    dbg!(result);
    
    readback_buffer.unmap();
}
