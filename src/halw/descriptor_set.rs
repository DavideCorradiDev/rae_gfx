extern crate gfx_hal as hal;

use hal::pso::DescriptorPool as HalDescriptorPool;
use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use super::{Backend, DescriptorPool, DescriptorSetLayout};

pub struct DescriptorSet {
    value: ManuallyDrop<<Backend as hal::Backend>::DescriptorSet>,
    pool: Rc<RefCell<DescriptorPool>>,
}

impl DescriptorSet {
    pub fn allocate_one(
        pool: Rc<RefCell<DescriptorPool>>,
        layout: &DescriptorSetLayout,
    ) -> Result<Self, hal::pso::AllocationError> {
        let descriptor_set = unsafe { pool.borrow_mut().allocate_set(layout) }?;
        Ok(Self {
            value: ManuallyDrop::new(descriptor_set),
            pool,
        })
    }
}

impl Drop for DescriptorSet {
    fn drop(&mut self) {
        unsafe {
            self.pool
                .borrow_mut()
                .free_sets(std::iter::once(ManuallyDrop::take(&mut self.value)));
        }
    }
}

impl Deref for DescriptorSet {
    type Target = <Backend as hal::Backend>::DescriptorSet;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for DescriptorSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Debug for DescriptorSet {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "DescriptorSet {{ value: {:?} }}", self.value)
    }
}
