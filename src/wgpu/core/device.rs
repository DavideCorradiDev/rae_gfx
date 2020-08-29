pub use wgpu::{BackendBit as Backend, Features, Limits, PowerPreference};

#[derive(Debug, Clone)]
pub struct DeviceConfig<'a> {
    pub backend: Backend,
    pub power_preference: PowerPreference,
    pub compatible_surface: Option<&'a wgpu::Surface>,
    pub required_features: Features,
    pub optional_features: Features,
    pub required_limits: Limits,
}

#[derive(Debug)]
pub struct Device {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
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

        let available_features = adapter.features();
        // TODO: write what features are missing in the error.
        if !available_features.contains(config.required_features) {
            return Err(DeviceCreationError::NoSuitableAdapter);
        }

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: (config.optional_features & available_features)
                        | config.required_features,
                    limits: config.required_limits.clone(),
                    shader_validation: true,
                },
                None,
            )
            .await?;

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeviceCreationError {
    NoSuitableAdapter,
    DeviceRequestFailed(wgpu::RequestDeviceError),
}

impl std::fmt::Display for DeviceCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceCreationError::NoSuitableAdapter => write!(f, "No suitable adapter"),
            DeviceCreationError::DeviceRequestFailed(e) => {
                write!(f, "Device request failed ({})", e)
            }
        }
    }
}

impl std::error::Error for DeviceCreationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DeviceCreationError::DeviceRequestFailed(e) => Some(e),
            _ => None,
        }
    }
}

impl From<wgpu::RequestDeviceError> for DeviceCreationError {
    fn from(e: wgpu::RequestDeviceError) -> Self {
        DeviceCreationError::DeviceRequestFailed(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let _device = futures::executor::block_on(Device::new(&DeviceConfig {
            backend: Backend::PRIMARY,
            power_preference: PowerPreference::Default,
            compatible_surface: None,
            required_features: Features::default(),
            optional_features: Features::empty(),
            required_limits: Limits::default(),
        }))
        .unwrap();
    }
}
