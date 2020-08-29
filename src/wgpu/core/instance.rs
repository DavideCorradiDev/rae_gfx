use std::default::Default;

use rae_app::window::Window;

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
    queue: Queue,
    device: Device,
    adapter: Adapter,
    instance: wgpu::Instance,
}

impl Instance {
    pub fn new(
        config: &InstanceConfig,
        compatible_surface: Option<&Surface>,
    ) -> Result<Self, InstanceCreationError> {
        let instance = Self::create_instance(config);
        let adapter = Self::create_adapter(&instance, config, compatible_surface)?;
        let (device, queue) = Self::create_device_and_queue(&adapter, config)?;
        Ok(Self {
            queue,
            adapter,
            device,
            instance,
        })
    }

    pub unsafe fn new_with_surface(
        config: &InstanceConfig,
        compatible_window: &Window,
    ) -> Result<(Self, Surface), InstanceCreationError> {
        let instance = Self::create_instance(config);
        let surface = instance.create_surface(compatible_window);
        let adapter = Self::create_adapter(&instance, config, Some(&surface))?;
        let (device, queue) = Self::create_device_and_queue(&adapter, config)?;
        Ok((
            Self {
                queue,
                adapter,
                device,
                instance,
            },
            surface,
        ))
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

    fn create_instance(config: &InstanceConfig) -> wgpu::Instance {
        wgpu::Instance::new(config.backend)
    }

    fn create_adapter(
        instance: &wgpu::Instance,
        config: &InstanceConfig,
        compatible_surface: Option<&Surface>,
    ) -> Result<Adapter, InstanceCreationError> {
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

        Ok(adapter)
    }

    fn create_device_and_queue(
        adapter: &Adapter,
        config: &InstanceConfig,
    ) -> Result<(Device, Queue), InstanceCreationError> {
        let (device, queue) = futures::executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: (config.optional_features & adapter.features())
                    | config.required_features,
                limits: config.required_limits.clone(),
                shader_validation: true,
            },
            None,
        ))?;
        Ok((device, queue))
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

    use rae_app::{
        event::{EventLoop, EventLoopAnyThread},
        window::WindowBuilder,
    };

    #[test]
    fn default_config() {
        let instance = Instance::new(&InstanceConfig::default(), None).unwrap();
        println!("{:?}", instance.info());
    }

    #[test]
    fn new() {
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
    fn new_with_surface() {
        let event_loop = EventLoop::<()>::new_any_thread();
        let window = WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)
            .unwrap();
        let (instance, _surface) =
            unsafe { Instance::new_with_surface(&InstanceConfig::default(), &window).unwrap() };
        println!("{:?}", instance.info());
    }
}
