use std::sync::Arc;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum RendererStateError {
    #[error("Failed to create a rendering surface: {0}")]
    SurfaceCreation(#[from] wgpu::CreateSurfaceError),

    #[error("Failed to create an adapter")]
    AdapterRequest(#[from] wgpu::RequestAdapterError),

    #[error("Failed to retrieve default surface configuration")]
    SurfaceConfiguration,

    #[error("Failed to create a graphics device: {0}")]
    DeviceCreation(#[from] wgpu::RequestDeviceError),
}

pub struct RendererState {
    pub window: Arc<dyn wgpu::DisplayAndWindowHandle>,
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl RendererState {
    pub fn new<T: wgpu::DisplayAndWindowHandle + 'static>(
        window: Arc<T>,
        viewport: (u32, u32),
    ) -> Result<Self, RendererStateError> {
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window.clone())?;

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }))?;

        let config = surface
            .get_default_config(&adapter, viewport.0, viewport.1)
            .ok_or(RendererStateError::SurfaceConfiguration)?;
        //let capabilities = surface.get_capabilities(&adapter);
        //then here change any necessary fields

        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default()))
                .expect("Failed to create a device");

        surface.configure(&device, &config);

        Ok(Self {
            window,
            surface,
            config,
            adapter,
            device,
            queue,
        })
    }
}
