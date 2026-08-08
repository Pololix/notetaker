use pollster::block_on;
use std::sync::Arc;

pub struct RendererState {
    window: Arc<dyn wgpu::DisplayAndWindowHandle>,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl RendererState {
    pub fn new<T: wgpu::DisplayAndWindowHandle + 'static>(
        window: Arc<T>,
        width: u32,
        height: u32,
    ) -> Self {
        let instance = wgpu::Instance::default();

        let surface = instance
            .create_surface(window.clone())
            .expect("Failed to create a window");

        let mut adapter_opts = wgpu::RequestAdapterOptions::default();
        adapter_opts.compatible_surface = Some(&surface);
        let adapter =
            block_on(instance.request_adapter(&adapter_opts)).expect("Failed to create an adapter");

        let /*mut*/ config = surface
            .get_default_config(&adapter, width, height)
            .expect("Failed to fecth default surface configuration");
        //let capabilities = surface.get_capabilities(&adapter);
        //then here change any necessary fields

        let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor::default()))
            .expect("Failed to create a device");

        surface.configure(&device, &config);

        Self {
            window,
            surface,
            config,
            adapter,
            device,
            queue,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width != 0 && height != 0 {
            return;
        }

        self.config.width = width;
        self.config.height = height;

        self.surface.configure(&self.device, &self.config);
    }
}
