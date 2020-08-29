pub use wgpu::BackendBit;

#[derive(Debug)]
pub struct Device {
    instance: wgpu::Instance,
    // adapter: wgpu::Adapter,
    // device: wgpu::Device,
    // queue: wgpu::Queue,
}

impl Device {
    pub fn new(backend: BackendBit) -> Self {
        let instance = wgpu::Instance::new(backend);
        Self { instance }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let _instance = instance::new(BackendBit::PRIMARY);
    }
}
