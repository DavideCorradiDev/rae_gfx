use std::default::Default;

pub use wgpu::{
    include_spirv, AdapterInfo as DeviceInfo, BackendBit as Backend, BindGroupLayout,
    BlendDescriptor, BufferAddress, ColorStateDescriptor, ColorWrite, CullMode, Features,
    FrontFace, IndexFormat, InputStepMode, Limits, PipelineLayout, PipelineLayoutDescriptor,
    PowerPreference, PrimitiveTopology, ProgrammableStageDescriptor, PushConstantRange,
    RasterizationStateDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderModule,
    ShaderModuleSource, TextureFormat, VertexAttributeDescriptor, VertexBufferDescriptor,
    VertexFormat, VertexStateDescriptor,
};

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize)]
pub struct DeviceConfig {
    pub backend: Backend,
    pub power_preference: PowerPreference,
    pub required_features: Features,
    pub optional_features: Features,
    pub required_limits: Limits,
}

impl DeviceConfig {
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

impl Default for DeviceConfig {
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
pub struct Device {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl Device {
    pub fn new(
        config: &DeviceConfig,
        compatible_surface: Option<&wgpu::Surface>,
    ) -> Result<Self, DeviceCreationError> {
        let instance = wgpu::Instance::new(config.backend);

        let adapter = match futures::executor::block_on(instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: config.power_preference,
                compatible_surface,
            },
        )) {
            Some(v) => v,
            None => return Err(DeviceCreationError::AdapterRequestFailed),
        };

        if !adapter.features().contains(config.required_features) {
            return Err(DeviceCreationError::FeaturesNotAvailable(
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

    pub fn info(&self) -> DeviceInfo {
        self.adapter.get_info()
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
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeviceCreationError {
    AdapterRequestFailed,
    FeaturesNotAvailable(Features),
    DeviceRequestFailed(wgpu::RequestDeviceError),
}

impl std::fmt::Display for DeviceCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceCreationError::AdapterRequestFailed => write!(f, "Adapter request failed"),
            DeviceCreationError::FeaturesNotAvailable(features) => {
                write!(f, "Required features are not available ({:?})", features)
            }
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
        let device = Device::new(
            &DeviceConfig {
                backend: Backend::PRIMARY,
                power_preference: PowerPreference::Default,
                required_features: Features::default(),
                optional_features: Features::empty(),
                required_limits: Limits::default(),
            },
            None,
        )
        .unwrap();
        println!("{:?}", device.info());
    }

    #[test]
    fn default_config() {
        let device = Device::new(&DeviceConfig::default(), None).unwrap();
        println!("{:?}", device.info());
    }
}
