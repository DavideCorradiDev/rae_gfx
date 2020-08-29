use std::default::Default;

use wgpu::util::DeviceExt;

use raw_window_handle::HasRawWindowHandle;

use super::{
    Adapter, AdapterInfo, Backend, Buffer, BufferInitDescriptor, Device, Features, Limits,
    PipelineLayout, PipelineLayoutDescriptor, PowerPreference, Queue, RenderPipeline,
    RenderPipelineDescriptor, ShaderModule, ShaderModuleSource, Surface, SwapChain,
    SwapChainDescriptor, TextureFormat,
};

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize)]
pub struct InstanceConfig {
    pub backend: Backend,
    pub power_preference: PowerPreference,
    pub required_features: Features,
    pub optional_features: Features,
    pub required_limits: Limits,
}

impl InstanceConfig {
    pub fn high_performance() -> Self {
        Self {
            backend: Backend::PRIMARY,
            power_preference: PowerPreference::HighPerformance,
            required_features: Features::default(),
            optional_features: Features::empty(),
            required_limits: Limits::default(),
        }
    }
}

impl Default for InstanceConfig {
    fn default() -> Self {
        Self {
            backend: Backend::PRIMARY,
            power_preference: PowerPreference::Default,
            required_features: Features::default(),
            optional_features: Features::empty(),
            required_limits: Limits::default(),
        }
    }
}

#[derive(Debug)]
pub struct Instance {
    instance: wgpu::Instance,
    adapter: Adapter,
    device: Device,
    queue: Queue,
}

impl Instance {
    pub fn new(
        config: &InstanceConfig,
        compatible_surface: Option<&wgpu::Surface>,
    ) -> Result<Self, InstanceCreationError> {
        let instance = wgpu::Instance::new(config.backend);

        let adapter = match futures::executor::block_on(instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: config.power_preference,
                compatible_surface,
            },
        )) {
            Some(v) => v,
            None => return Err(InstanceCreationError::AdapterRequestFailed),
        };

        if !adapter.features().contains(config.required_features) {
            return Err(InstanceCreationError::FeaturesNotAvailable(
                config.required_features - adapter.features(),
            ));
        }

        let (device, queue) = futures::executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: (config.optional_features & adapter.features())
                    | config.required_features,
                limits: config.required_limits.clone(),
                shader_validation: true,
            },
            None,
        ))?;

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }

    pub fn color_format(&self) -> TextureFormat {
        TextureFormat::Bgra8UnormSrgb
    }

    pub fn info(&self) -> AdapterInfo {
        self.adapter.get_info()
    }

    pub unsafe fn create_surface<W: HasRawWindowHandle>(&self, window: &W) -> Surface {
        self.instance.create_surface(window)
    }

    pub fn create_swap_chain(&self, surface: &Surface, desc: &SwapChainDescriptor) -> SwapChain {
        self.device.create_swap_chain(surface, desc)
    }

    pub fn create_shader_module(&self, source: ShaderModuleSource) -> ShaderModule {
        self.device.create_shader_module(source)
    }

    pub fn create_pipeline_layout(&self, desc: &PipelineLayoutDescriptor) -> PipelineLayout {
        self.device.create_pipeline_layout(desc)
    }

    pub fn create_render_pipeline(&self, desc: &RenderPipelineDescriptor) -> RenderPipeline {
        self.device.create_render_pipeline(desc)
    }

    pub fn create_buffer_init(&self, desc: &BufferInitDescriptor) -> Buffer {
        self.device.create_buffer_init(desc)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstanceCreationError {
    AdapterRequestFailed,
    FeaturesNotAvailable(Features),
    DeviceRequestFailed(wgpu::RequestDeviceError),
}

impl std::fmt::Display for InstanceCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstanceCreationError::AdapterRequestFailed => write!(f, "Adapter request failed"),
            InstanceCreationError::FeaturesNotAvailable(features) => {
                write!(f, "Required features are not available ({:?})", features)
            }
            InstanceCreationError::DeviceRequestFailed(e) => {
                write!(f, "Device request failed ({})", e)
            }
        }
    }
}

impl std::error::Error for InstanceCreationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            InstanceCreationError::DeviceRequestFailed(e) => Some(e),
            _ => None,
        }
    }
}

impl From<wgpu::RequestDeviceError> for InstanceCreationError {
    fn from(e: wgpu::RequestDeviceError) -> Self {
        InstanceCreationError::DeviceRequestFailed(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let instance = Instance::new(
            &InstanceConfig {
                backend: Backend::PRIMARY,
                power_preference: PowerPreference::Default,
                required_features: Features::default(),
                optional_features: Features::empty(),
                required_limits: Limits::default(),
            },
            None,
        )
        .unwrap();
        println!("{:?}", instance.info());
    }

    #[test]
    fn default_config() {
        let instance = Instance::new(&InstanceConfig::default(), None).unwrap();
        println!("{:?}", instance.info());
    }
}