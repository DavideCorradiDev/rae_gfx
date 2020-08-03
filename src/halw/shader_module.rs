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

pub struct ShaderModule {
    value: ManuallyDrop<<Backend as hal::Backend>::ShaderModule>,
    gpu: Rc<RefCell<Gpu>>,
}

impl ShaderModule {
    pub fn from_spirv(
        gpu: Rc<RefCell<Gpu>>,
        spirv_data: &[u32],
    ) -> Result<Self, hal::device::ShaderError> {
        let shader_module = unsafe { gpu.borrow().device.create_shader_module(spirv_data) }?;
        Ok(Self {
            value: ManuallyDrop::new(shader_module),
            gpu,
        })
    }
}

impl Drop for ShaderModule {
    fn drop(&mut self) {
        unsafe {
            self.gpu
                .borrow()
                .device
                .destroy_shader_module(ManuallyDrop::take(&mut self.value));
        }
    }
}

impl Deref for ShaderModule {
    type Target = <Backend as hal::Backend>::ShaderModule;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for ShaderModule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Debug for ShaderModule {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "ShaderModule {{ value: {:?} }}", self.value)
    }
}
