extern crate gfx_hal as hal;

use hal::adapter::PhysicalDevice as HalPhysicalDevice;

use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
    rc::Rc,
};

use super::{Adapter, Backend, QueueFamily};

pub struct Gpu {
    value: hal::adapter::Gpu<Backend>,
    adapter: Rc<RefCell<Adapter>>,
}

impl Gpu {
    pub fn open(
        adapter: Rc<RefCell<Adapter>>,
        families: &[(&QueueFamily, &[hal::queue::QueuePriority])],
        requested_features: hal::Features,
    ) -> Result<Self, hal::device::CreationError> {
        let gpu = unsafe {
            adapter
                .borrow()
                .physical_device
                .open(families, requested_features)
        }?;
        Ok(Self {
            value: gpu,
            adapter,
        })
    }
}

impl Deref for Gpu {
    type Target = hal::adapter::Gpu<Backend>;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for Gpu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Debug for Gpu {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Gpu {{ value: {:?} }}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use galvanic_assert::{matchers::*, *};

    use super::*;
    use crate::halw::Instance;

    #[test]
    fn creation() {
        let instance = Rc::new(RefCell::new(Instance::create("Name", 1).unwrap()));
        let mut adapters = Adapter::enumerate(instance);
        assert_that!(&adapters.len(), not(eq(0)));
        let adapter = Rc::new(RefCell::new(adapters.remove(0)));
        let queue_family = &adapter.borrow().queue_families[0];
        let _gpu = Gpu::open(
            Rc::clone(&adapter),
            &[(queue_family, &[1.0])],
            hal::Features::empty(),
        );
    }
}
