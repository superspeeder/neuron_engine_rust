use thiserror::Error;

pub struct RenderSystem {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

#[derive(Debug, Error)]
pub enum RenderSystemCreateError {
    #[error(transparent)]
    RequestAdapterError(#[from] wgpu::RequestAdapterError),

    #[error(transparent)]
    RequestDeviceError(#[from] wgpu::RequestDeviceError),
}

impl RenderSystem {
    pub async fn new() -> Result<Self, RenderSystemCreateError> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backend_options: wgpu::BackendOptions::default(),
            backends: wgpu::Backends::PRIMARY,
            flags: wgpu::InstanceFlags::default(),
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Primary Render Device"),
                required_features: wgpu::Features::POLYGON_MODE_LINE
                    | wgpu::Features::PUSH_CONSTANTS
                    | wgpu::Features::TEXTURE_BINDING_ARRAY,
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
            })
            .await?;

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }

    pub fn instance(&self) -> &wgpu::Instance {
        &self.instance
    }

    pub fn adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }
}
