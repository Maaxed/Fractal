use winit::{window::Window, dpi::PhysicalSize};


pub struct Target
{
	pub window: Window,
	pub surface: wgpu::Surface,
	pub config: wgpu::SurfaceConfiguration,
	pub device: wgpu::Device,
	pub queue: wgpu::Queue,
	pub supports_compute_shader: bool,
}

impl Target
{
	pub async fn new(window: Window, device_limits: wgpu::Limits) -> Self
	{
		let backends = wgpu::util::backend_bits_from_env().unwrap_or(wgpu::Backends::all());
		let dx12_shader_compiler = wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default();
		let gles_minor_version = wgpu::util::gles_minor_version_from_env().unwrap_or_default();
		let instance = wgpu::Instance::new(
			wgpu::InstanceDescriptor
			{
				backends,
				flags: wgpu::InstanceFlags::default(),
				dx12_shader_compiler,
				gles_minor_version
			}
		);
		
		let surface = unsafe { instance.create_surface(&window) }.expect("Failed to create surface");

		let adapter = wgpu::util::initialize_adapter_from_env_or_default(&instance, Some(&surface))
			.await
			.expect("Failed to find an appropriate adapter");

		let (device, queue) = Self::request_device(&adapter, device_limits).await.expect("Failed to find a compatible device");
		
        let window_size = window.inner_size();
        let config = surface.get_default_config(&adapter, window_size.width, window_size.height).expect("Surface not supported by adapter");

		let supports_compute_shader = adapter.get_downlevel_capabilities().flags.contains(wgpu::DownlevelFlags::COMPUTE_SHADERS);

		let this = Self
		{
			window,
			surface,
			config,
			device,
			queue,
			supports_compute_shader,
		};

        this.configure_surface();

		this
	}

	async fn request_device(adapter: &wgpu::Adapter, device_limits: wgpu::Limits) -> Result<(wgpu::Device, wgpu::Queue), wgpu::RequestDeviceError>
	{
		let optional_features = wgpu::Features::SHADER_F64;
		let available_features = adapter.features();

		adapter
			.request_device(
				&wgpu::DeviceDescriptor
				{
					label: None,
					features: optional_features & available_features,
					limits: device_limits.using_resolution(adapter.limits()),
				},
				None,
			).await
	}

	pub fn configure_surface(&self)
	{
		self.surface.configure(&self.device, &self.config);
	}

	pub fn resize(&mut self, new_size: PhysicalSize<u32>) -> bool
	{
		if new_size.width == 0 || new_size.height == 0
		{
			return false;
		}

		if new_size.width == self.config.width && new_size.height == self.config.height
		{
			return false;
		}

		self.config.width = new_size.width;
		self.config.height = new_size.height;
		self.configure_surface();
		true
	}
}