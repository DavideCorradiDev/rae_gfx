pub use wgpu::BackendBit;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DeviceConfig {
    pub backend: BackendBit,
}

#[derive(Debug)]
pub struct Device {
    instance: wgpu::Instance,
    // adapter: wgpu::Adapter,
    // device: wgpu::Device,
    // queue: wgpu::Queue,
}

impl Device {
    pub fn new(config: DeviceConfig) -> Self {
        let instance = wgpu::Instance::new(config.backend);
        Self { instance }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let _device = Device::new(DeviceConfig {
            backend: BackendBit::PRIMARY,
        });
    }
}
