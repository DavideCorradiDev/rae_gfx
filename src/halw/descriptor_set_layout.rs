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

pub struct DescriptorSetLayout {
    value: ManuallyDrop<<Backend as hal::Backend>::DescriptorSetLayout>,
    gpu: Rc<RefCell<Gpu>>,
}

impl DescriptorSetLayout {
    pub fn create<I, J>(
        gpu: Rc<RefCell<Gpu>>,
        bindings: I,
        immutable_samplers: J,
    ) -> Result<Self, hal::device::OutOfMemory>
    where
        I: std::iter::IntoIterator,
        I::Item: std::borrow::Borrow<hal::pso::DescriptorSetLayoutBinding>,
        J: std::iter::IntoIterator,
        J::Item: std::borrow::Borrow<<Backend as hal::Backend>::Sampler>,
    {
        let dsl = unsafe {
            gpu.borrow()
                .device
                .create_descriptor_set_layout(bindings, immutable_samplers)
        }?;
        Ok(Self {
            value: ManuallyDrop::new(dsl),
            gpu,
        })
    }
}

impl Drop for DescriptorSetLayout {
    fn drop(&mut self) {
        unsafe {
            self.gpu
                .borrow()
                .device
                .destroy_descriptor_set_layout(ManuallyDrop::take(&mut self.value));
        }
    }
}

impl Deref for DescriptorSetLayout {
    type Target = <Backend as hal::Backend>::DescriptorSetLayout;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for DescriptorSetLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Debug for DescriptorSetLayout {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "DescriptorSetLayout {{ value: {:?} }}", self.value)
    }
}
