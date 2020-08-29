pub use wgpu::{BackendBit, PowerPreference};

#[derive(Debug, Clone, Copy)]
pub struct DeviceConfig<'a> {
    pub backend: BackendBit,
    pub power_preference: PowerPreference,
    pub compatible_surface: Option<&'a wgpu::Surface>,
}

#[derive(Debug)]
pub struct Device {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    // device: wgpu::Device,
    // queue: wgpu::Queue,
}

impl Device {
    pub async fn new(config: &DeviceConfig<'_>) -> Result<Self, DeviceCreationError> {
        let instance = wgpu::Instance::new(config.backend);
        let adapter = match instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: config.power_preference,
                compatible_surface: config.compatible_surface,
            })
            .await
        {
            Some(v) => v,
            None => return Err(DeviceCreationError::NoSuitableAdapter),
        };
        Ok(Self { instance, adapter })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeviceCreationError {
    UnsupportedBackend,
    NoSuitableAdapter,
    NoSuitableQueueFamily,
    SurfaceCreationFailed,
    DeviceCreationFailed,
}

impl std::fmt::Display for DeviceCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceCreationError::UnsupportedBackend => write!(f, "Unsupported backend"),
            DeviceCreationError::NoSuitableAdapter => write!(f, "No suitable adapter"),
            DeviceCreationError::NoSuitableQueueFamily => write!(f, "No suitable queue family"),
            DeviceCreationError::SurfaceCreationFailed => {
                write!(f, "Window surface creation failed")
            }
            DeviceCreationError::DeviceCreationFailed => write!(f, "Device creation failed"),
        }
    }
}

impl std::error::Error for DeviceCreationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let _device = futures::executor::block_on(Device::new(&DeviceConfig {
            backend: BackendBit::PRIMARY,
            power_preference: PowerPreference::Default,
            compatible_surface: None,
        }))
        .unwrap();
    }
}
