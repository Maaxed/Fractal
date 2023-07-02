use winit::{window::Window, event_loop::EventLoopWindowTarget, dpi::PhysicalSize};


pub struct Target
{
	pub window: Window,
	pub surface: wgpu::Surface,
	pub config: wgpu::SurfaceConfiguration,
	pub device: wgpu::Device,
	pub queue: wgpu::Queue,
}

impl Target
{
	pub async fn new(event_loop: &EventLoopWindowTarget<()>) -> Self
	{
    	let window: Window = Window::new(event_loop).expect("Failed to create window");
		window.set_title("Fractal");

		let backends = wgpu::util::backend_bits_from_env().unwrap_or(wgpu::Backends::PRIMARY);
		let dx12_shader_compiler = wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default();
		let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends, dx12_shader_compiler });
		
		let surface = unsafe { instance.create_surface(&window) }.expect("Failed to create surface");

		let adapter = wgpu::util::initialize_adapter_from_env_or_default(&instance, backends, Some(&surface))
			.await
			.expect("Failed to find an appropriate adapter");

		let (device, queue) = adapter
			.request_device(
				&wgpu::DeviceDescriptor
				{
					label: None,
					features: wgpu::Features::SHADER_F64,
					limits: wgpu::Limits::default(),
				},
				None,
			)
			.await
			.expect("Failed to create device");
		
        let swapchain_capabilities: wgpu::SurfaceCapabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let window_size = window.inner_size();
        let config = wgpu::SurfaceConfiguration
        {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: swapchain_capabilities.present_modes[0],
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

		let this = Self
		{
			window,
			surface,
			config,
			device,
			queue,
		};

        this.configure_surface();

		this
	}

	pub fn configure_surface(&self)
	{
		self.surface.configure(&self.device, &self.config);
	}

	pub fn resize(&mut self, new_size: PhysicalSize<u32>) -> bool
	{
		if new_size.width <= 0 || new_size.height <= 0
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