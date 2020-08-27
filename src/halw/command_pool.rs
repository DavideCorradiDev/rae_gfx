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

pub struct CommandPool {
    value: ManuallyDrop<<Backend as hal::Backend>::CommandPool>,
    gpu: Rc<RefCell<Gpu>>,
}

impl CommandPool {
    pub fn create(
        gpu: Rc<RefCell<Gpu>>,
        family: hal::queue::QueueFamilyId,
        create_flags: hal::pool::CommandPoolCreateFlags,
    ) -> Result<Self, hal::device::OutOfMemory> {
        let command_pool = unsafe {
            gpu.borrow()
                .device
                .create_command_pool(family, create_flags)
        }?;
        Ok(Self {
            value: ManuallyDrop::new(command_pool),
            gpu,
        })
    }
}

impl Drop for CommandPool {
    fn drop(&mut self) {
        println!("*** Dropping Command Pool {:?}", self);
        unsafe {
            self.gpu
                .borrow()
                .device
                .destroy_command_pool(ManuallyDrop::take(&mut self.value));
        }
    }
}

impl Deref for CommandPool {
    type Target = <Backend as hal::Backend>::CommandPool;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for CommandPool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Debug for CommandPool {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "CommandPool {{ value: {:?} }}", self.value)
    }
}
