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

pub struct Buffer {
    value: ManuallyDrop<<Backend as hal::Backend>::Buffer>,
    gpu: Rc<RefCell<Gpu>>,
}

impl Buffer {
    pub fn create(
        gpu: Rc<RefCell<Gpu>>,
        size: u64,
        usage: hal::buffer::Usage,
    ) -> Result<Self, hal::buffer::CreationError> {
        let buffer = unsafe { gpu.borrow().device.create_buffer(size, usage) }?;
        Ok(Self {
            value: ManuallyDrop::new(buffer),
            gpu,
        })
    }

    pub fn requirements(&self) -> hal::memory::Requirements {
        unsafe {
            self.gpu
                .borrow()
                .device
                .get_buffer_requirements(&self.value)
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.gpu
                .borrow()
                .device
                .destroy_buffer(ManuallyDrop::take(&mut self.value));
        }
    }
}

impl Deref for Buffer {
    type Target = <Backend as hal::Backend>::Buffer;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Debug for Buffer {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Buffer {{ value: {:?} }}", self.value)
    }
}
