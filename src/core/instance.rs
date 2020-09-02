use std::{
    default::Default,
    ops::{Deref, DerefMut},
};

use rae_app::window::Window;

use wgpu::util::DeviceExt;

use raw_window_handle::HasRawWindowHandle;

use super::{
    Adapter, AdapterInfo, Backend, Buffer, BufferInitDescriptor, CommandBuffer, CommandEncoder,
    CommandEncoderDescriptor, Device, Features, Limits, PipelineLayoutDescriptor, PowerPreference,
    Queue, RenderPipelineDescriptor, ShaderModuleSource, SwapChain, SwapChainDescriptor,
    TextureFormat,
};

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct InstanceDescriptor {
    pub backend: Backend,
    pub power_preference: PowerPreference,
    pub required_features: Features,
    pub optional_features: Features,
    pub required_limits: Limits,
}

impl InstanceDescriptor {
    pub fn high_performance() -> Self {
        let mut required_limits = Limits::default();
        required_limits.max_push_constant_size = 128;
        Self {
            backend: Backend::PRIMARY,
            power_preference: PowerPreference::HighPerformance,
            required_features: Features::default() | Features::PUSH_CONSTANTS,
            optional_features: Features::empty(),
            required_limits,
        }
    }
}

impl Default for InstanceDescriptor {
    fn default() -> Self {
        let mut required_limits = Limits::default();
        required_limits.max_push_constant_size = 128;
        Self {
            backend: Backend::PRIMARY,
            power_preference: PowerPreference::Default,
            required_features: Features::default() | Features::PUSH_CONSTANTS,
            optional_features: Features::empty(),
            required_limits,
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
    pub fn new(desc: &InstanceDescriptor) -> Result<Self, InstanceCreationError> {
        let instance = Self::create_instance(desc);
        let adapter = Self::create_adapter(&instance, desc, None)?;
        let (device, queue) = Self::create_device_and_queue(&adapter, desc)?;
        Ok(Self {
            queue,
            adapter,
            device,
            instance,
        })
    }

    // Unsafe: surface creation.
    pub unsafe fn new_with_compatible_window(
        desc: &InstanceDescriptor,
        compatible_window: &Window,
    ) -> Result<(Self, Surface), InstanceCreationError> {
        let instance = wgpu::Instance::new(desc.backend);
        let surface = instance.create_surface(compatible_window);
        let adapter = Self::create_adapter(&instance, desc, Some(&surface))?;
        let (device, queue) = Self::create_device_and_queue(&adapter, desc)?;
        Ok((
            Self {
                queue,
                adapter,
                device,
                instance,
            },
            Surface { value: surface },
        ))
    }
    pub fn color_format(&self) -> TextureFormat {
        TextureFormat::Bgra8UnormSrgb
    }

    pub fn info(&self) -> AdapterInfo {
        self.adapter.get_info()
    }

    pub fn create_swap_chain(
        &self,
        surface: &wgpu::Surface,
        desc: &SwapChainDescriptor,
    ) -> SwapChain {
        self.device.create_swap_chain(surface, desc)
    }

    pub fn create_buffer_init(&self, desc: &BufferInitDescriptor) -> Buffer {
        self.device.create_buffer_init(desc)
    }

    pub fn create_command_encoder(&self, desc: &CommandEncoderDescriptor) -> CommandEncoder {
        self.device.create_command_encoder(desc)
    }

    pub fn submit<I: IntoIterator<Item = CommandBuffer>>(&self, command_buffers: I) {
        self.queue.submit(command_buffers);
    }

    fn create_instance(desc: &InstanceDescriptor) -> wgpu::Instance {
        wgpu::Instance::new(desc.backend)
    }

    fn create_adapter(
        instance: &wgpu::Instance,
        desc: &InstanceDescriptor,
        compatible_surface: Option<&wgpu::Surface>,
    ) -> Result<Adapter, InstanceCreationError> {
        let adapter = match futures::executor::block_on(instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: desc.power_preference,
                compatible_surface,
            },
        )) {
            Some(v) => v,
            None => return Err(InstanceCreationError::AdapterRequestFailed),
        };

        if !adapter.features().contains(desc.required_features) {
            return Err(InstanceCreationError::FeaturesNotAvailable(
                desc.required_features - adapter.features(),
            ));
        }

        Ok(adapter)
    }

    fn create_device_and_queue(
        adapter: &Adapter,
        desc: &InstanceDescriptor,
    ) -> Result<(Device, Queue), InstanceCreationError> {
        let (device, queue) = futures::executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: (desc.optional_features & adapter.features()) | desc.required_features,
                limits: desc.required_limits.clone(),
                shader_validation: true,
            },
            None,
        ))?;
        Ok((device, queue))
    }
}

#[derive(Debug)]
pub struct Surface {
    value: wgpu::Surface,
}

impl Surface {
    pub unsafe fn new<W: HasRawWindowHandle>(instance: &Instance, window: &W) -> Self {
        Self {
            value: instance.instance.create_surface(window),
        }
    }
}

impl Deref for Surface {
    type Target = wgpu::Surface;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[derive(Debug)]
pub struct ShaderModule {
    value: wgpu::ShaderModule,
}

impl ShaderModule {
    pub fn new(instance: &Instance, source: ShaderModuleSource) -> ShaderModule {
        Self {
            value: instance.device.create_shader_module(source),
        }
    }
}

impl Deref for ShaderModule {
    type Target = wgpu::ShaderModule;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for ShaderModule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[derive(Debug)]
pub struct PipelineLayout {
    value: wgpu::PipelineLayout,
}

impl PipelineLayout {
    pub fn new(instance: &Instance, desc: &PipelineLayoutDescriptor) -> PipelineLayout {
        Self {
            value: instance.device.create_pipeline_layout(desc),
        }
    }
}

impl Deref for PipelineLayout {
    type Target = wgpu::PipelineLayout;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for PipelineLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[derive(Debug)]
pub struct RenderPipeline {
    value: wgpu::RenderPipeline,
}

impl RenderPipeline {
    pub fn new(instance: &Instance, desc: &RenderPipelineDescriptor) -> RenderPipeline {
        Self {
            value: instance.device.create_render_pipeline(desc),
        }
    }
}

impl Deref for RenderPipeline {
    type Target = wgpu::RenderPipeline;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for RenderPipeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
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
        let instance = Instance::new(&InstanceDescriptor::default()).unwrap();
        println!("{:?}", instance.info());
    }

    #[test]
    fn new() {
        let instance = Instance::new(&InstanceDescriptor {
            backend: Backend::PRIMARY,
            power_preference: PowerPreference::Default,
            required_features: Features::default(),
            optional_features: Features::empty(),
            required_limits: Limits::default(),
        })
        .unwrap();
        println!("{:?}", instance.info());
    }

    #[test]
    fn new_with_compatible_window() {
        let event_loop = EventLoop::<()>::new_any_thread();
        let window = WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)
            .unwrap();
        let (instance, _surface) = unsafe {
            Instance::new_with_compatible_window(&InstanceDescriptor::default(), &window).unwrap()
        };
        println!("{:?}", instance.info());
    }
}
