extern crate gfx_hal as hal;
extern crate lazy_static;
extern crate winit;

use std::{
    cell::{Ref, RefCell, RefMut},
    mem::ManuallyDrop,
    ops::Deref,
    rc::Rc,
    sync::{Arc, RwLock},
};

use hal::{
    queue::QueueFamily as HalQueueFamily, window::Surface as HalSurface, Backend as HalBackend,
    Instance as HalInstance,
};

use super::TextureFormat;
use crate::{halw, window, window::EventLoopExt};

lazy_static::lazy_static! {
    static ref INSTANCE: Arc<RwLock<halw::Instance>>
        = Arc::new(RwLock::new(halw::Instance::create("Red Ape Engine", 1).unwrap()));
}

pub struct Instance {
    adapter: Rc<RefCell<halw::Adapter>>,
    gpu: Rc<RefCell<halw::Gpu>>,
    canvas_color_format: TextureFormat,
}

impl Instance {
    pub fn create() -> Result<Self, InstanceCreationError> {
        let adapter = Rc::new(RefCell::new(Self::select_adapter(
            INSTANCE.read().unwrap().deref(),
        )?));
        let (_a, _b, mut dummy_surface) =
            Self::create_dummy_surface(INSTANCE.read().unwrap().deref())?;
        let gpu = Rc::new(RefCell::new(Self::open_device(
            adapter.borrow().deref(),
            &dummy_surface,
        )?));
        let canvas_color_format =
            Self::select_canvas_color_format(adapter.borrow().deref(), &dummy_surface);
        Self::destroy_dummy_surface(INSTANCE.read().unwrap().deref(), &mut dummy_surface);
        Ok(Self {
            adapter,
            gpu,
            canvas_color_format,
        })
    }

    pub fn instance_arc(&self) -> &Arc<RwLock<halw::Instance>> {
        &INSTANCE
    }

    pub fn adapter(&self) -> Ref<halw::Adapter> {
        self.adapter.borrow()
    }

    pub fn adapter_mut(&mut self) -> RefMut<halw::Adapter> {
        self.adapter.borrow_mut()
    }

    pub fn adapter_rc(&self) -> &Rc<RefCell<halw::Adapter>> {
        &self.adapter
    }

    pub fn gpu(&self) -> Ref<halw::Gpu> {
        self.gpu.borrow()
    }

    pub fn gpu_mut(&mut self) -> RefMut<halw::Gpu> {
        self.gpu.borrow_mut()
    }

    pub fn gpu_rc(&self) -> &Rc<RefCell<halw::Gpu>> {
        &self.gpu
    }

    pub fn canvas_color_format(&self) -> TextureFormat {
        self.canvas_color_format
    }

    #[cfg(feature = "empty")]
    fn adapter_selection_criteria(_adapter: &hal::adapter::Adapter<halw::Backend>) -> bool {
        true
    }

    #[cfg(not(feature = "empty"))]
    fn adapter_selection_criteria(adapter: &hal::adapter::Adapter<halw::Backend>) -> bool {
        adapter.info.device_type == hal::adapter::DeviceType::DiscreteGpu
            || adapter.info.device_type == hal::adapter::DeviceType::IntegratedGpu
    }

    fn select_adapter(instance: &halw::Instance) -> Result<halw::Adapter, InstanceCreationError> {
        let mut adapters = instance.enumerate_adapters();
        adapters.retain(Self::adapter_selection_criteria);
        if adapters.is_empty() {
            return Err(InstanceCreationError::NoSuitableAdapter);
        }

        adapters.sort_by(|a, b| {
            if a.info.device_type == b.info.device_type {
                return std::cmp::Ordering::Equal;
            } else if a.info.device_type == hal::adapter::DeviceType::DiscreteGpu {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Greater;
            }
        });
        Ok(adapters.remove(0))
    }

    fn create_dummy_surface(
        instance: &halw::Instance,
    ) -> Result<
        (
            window::EventLoop<()>,
            window::Window,
            ManuallyDrop<<halw::Backend as HalBackend>::Surface>,
        ),
        InstanceCreationError,
    > {
        let dummy_event_loop = window::EventLoop::new_any_thread();
        let dummy_window = window::WindowBuilder::new()
            .with_visible(false)
            .build(&dummy_event_loop)
            .unwrap();
        let dummy_surface = ManuallyDrop::new(unsafe { instance.create_surface(&dummy_window) }?);
        Ok((dummy_event_loop, dummy_window, dummy_surface))
    }

    fn destroy_dummy_surface(
        instance: &halw::Instance,
        dummy_surface: &mut ManuallyDrop<<halw::Backend as HalBackend>::Surface>,
    ) {
        unsafe {
            instance.destroy_surface(ManuallyDrop::take(dummy_surface));
        }
    }

    fn select_canvas_color_format(
        adapter: &halw::Adapter,
        surface: &<halw::Backend as HalBackend>::Surface,
    ) -> hal::format::Format {
        let formats = surface.supported_formats(&adapter.physical_device);
        formats.map_or(hal::format::Format::Rgba8Srgb, |formats| {
            formats
                .iter()
                .find(|a| a.base_format().1 == hal::format::ChannelType::Srgb)
                .map(|a| *a)
                .unwrap_or(formats[0])
        })
    }

    fn select_queue_family<'a>(
        adapter: &'a halw::Adapter,
        surface: &<halw::Backend as HalBackend>::Surface,
    ) -> Result<&'a halw::QueueFamily, InstanceCreationError> {
        // Eventually add required constraints here.
        match adapter.queue_families.iter().find(|family| {
            surface.supports_queue_family(family) && family.queue_type().supports_graphics()
        }) {
            Some(family) => Ok(family),
            None => Err(InstanceCreationError::NoSuitableQueueFamily),
        }
    }

    fn open_device(
        adapter: &halw::Adapter,
        surface: &<halw::Backend as HalBackend>::Surface,
    ) -> Result<halw::Gpu, InstanceCreationError> {
        // Eventually add required GPU features here.
        let queue_family = Self::select_queue_family(adapter, surface)?;
        let gpu = halw::Gpu::open(adapter, &[(queue_family, &[1.0])], hal::Features::empty())?;
        Ok(gpu)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstanceCreationError {
    UnsupportedBackend,
    NoSuitableAdapter,
    NoSuitableQueueFamily,
    SurfaceCreationFailed,
    DeviceCreationFailed,
}

impl std::fmt::Display for InstanceCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstanceCreationError::UnsupportedBackend => write!(f, "Unsupported backend"),
            InstanceCreationError::NoSuitableAdapter => write!(f, "No suitable adapter"),
            InstanceCreationError::NoSuitableQueueFamily => write!(f, "No suitable queue family"),
            InstanceCreationError::SurfaceCreationFailed => {
                write!(f, "Window surface creation failed")
            }
            InstanceCreationError::DeviceCreationFailed => write!(f, "Device creation failed"),
        }
    }
}

impl std::error::Error for InstanceCreationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<hal::UnsupportedBackend> for InstanceCreationError {
    fn from(_: hal::UnsupportedBackend) -> Self {
        InstanceCreationError::UnsupportedBackend
    }
}

impl From<hal::window::InitError> for InstanceCreationError {
    fn from(_: hal::window::InitError) -> Self {
        InstanceCreationError::SurfaceCreationFailed
    }
}

impl From<hal::device::CreationError> for InstanceCreationError {
    fn from(_: hal::device::CreationError) -> Self {
        InstanceCreationError::DeviceCreationFailed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instance_creation() {
        let _instance = Instance::create().unwrap();
    }

    #[test]
    fn instance_creation_2() {
        let _instance = Instance::create().unwrap();
    }

    #[test]
    fn instance_creation_3() {
        let _instance = Instance::create().unwrap();
    }
}
