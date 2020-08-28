extern crate gfx_hal as hal;

use hal::device::Device as HalDevice;
use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use super::{Backend, Gpu};

pub struct Semaphore {
    value: ManuallyDrop<<Backend as hal::Backend>::Semaphore>,
    gpu: Rc<RefCell<Gpu>>,
}

impl Semaphore {
    pub fn create(gpu: Rc<RefCell<Gpu>>) -> Result<Self, hal::device::OutOfMemory> {
        let semaphore = gpu.borrow().device.create_semaphore()?;
        Ok(Self {
            value: ManuallyDrop::new(semaphore),
            gpu,
        })
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        unsafe {
            self.gpu
                .borrow()
                .device
                .destroy_semaphore(ManuallyDrop::take(&mut self.value));
        }
    }
}

impl Deref for Semaphore {
    type Target = <Backend as hal::Backend>::Semaphore;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for Semaphore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Debug for Semaphore {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Semaphore {{ value: {:?} }}", self.value)
    }
}
