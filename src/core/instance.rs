extern crate gfx_hal as hal;

use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use hal::queue::QueueFamily as HalQueueFamily;

use crate::halw;

#[derive(Debug)]
pub struct Instance {
    gpu: Rc<RefCell<halw::Gpu>>,
    adapter: Rc<RefCell<halw::Adapter>>,
    instance: Rc<RefCell<halw::Instance>>,
}

impl Instance {
    pub fn create() -> Result<Self, InstanceCreationError> {
        let instance = Rc::new(RefCell::new(halw::Instance::create("Read Ape Engine", 1)?));
        let adapter = Rc::new(RefCell::new(Self::select_adapter(Rc::clone(&instance))?));
        let gpu = Rc::new(RefCell::new(Self::open_device(Rc::clone(&adapter))?));
        Ok(Self {
            gpu,
            adapter,
            instance,
        })
    }

    pub fn instance(&self) -> Ref<halw::Instance> {
        self.instance.borrow()
    }

    pub fn instance_mut(&mut self) -> RefMut<halw::Instance> {
        self.instance.borrow_mut()
    }

    pub fn instance_rc(&self) -> &Rc<RefCell<halw::Instance>> {
        &self.instance
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

    #[cfg(feature = "empty")]
    fn adapter_selection_criteria(_adapter: &hal::adapter::Adapter<halw::Backend>) -> bool {
        true
    }

    #[cfg(not(feature = "empty"))]
    fn adapter_selection_criteria(adapter: &halw::Adapter) -> bool {
        adapter.info.device_type == hal::adapter::DeviceType::DiscreteGpu
            || adapter.info.device_type == hal::adapter::DeviceType::IntegratedGpu
    }

    fn select_adapter(
        instance: Rc<RefCell<halw::Instance>>,
    ) -> Result<halw::Adapter, InstanceCreationError> {
        let mut adapters = halw::Adapter::enumerate(instance);
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

    fn select_queue_family<'a>(
        adapter: &'a halw::Adapter,
    ) -> Result<&'a halw::QueueFamily, InstanceCreationError> {
        // Eventually add required constraints here.
        match adapter
            .queue_families
            .iter()
            .find(|family| family.queue_type().supports_graphics())
        {
            Some(family) => Ok(family),
            None => Err(InstanceCreationError::NoSuitableQueueFamily),
        }
    }

    fn open_device(
        adapter: Rc<RefCell<halw::Adapter>>,
    ) -> Result<halw::Gpu, InstanceCreationError> {
        // Eventually add required GPU features here.
        let adapter_ref = &adapter.borrow();
        let queue_family = Self::select_queue_family(adapter_ref)?;
        let gpu = halw::Gpu::open(
            Rc::clone(&adapter),
            &[(queue_family, &[1.0])],
            hal::Features::empty(),
        )?;
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
    fn double_instance_creation() {
        let _instance1 = Instance::create().unwrap();
        let _instance2 = Instance::create().unwrap();
    }
}
