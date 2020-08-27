extern crate gfx_hal as hal;
use super::Backend;

use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use hal::Instance as HalInstance;

use super::Instance;

#[derive(Debug)]
pub struct Adapter {
    value: hal::adapter::Adapter<Backend>,
    instance: Rc<RefCell<Instance>>,
}

impl Adapter {
    pub fn enumerate(instance: Rc<RefCell<Instance>>) -> Vec<Self> {
        instance
            .borrow()
            .enumerate_adapters()
            .into_iter()
            .map(|x| Self {
                value: x,
                instance: Rc::clone(&instance),
            })
            .collect()
    }
}

impl Deref for Adapter {
    type Target = hal::adapter::Adapter<Backend>;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for Adapter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Drop for Adapter {
    fn drop(&mut self) {
        println!("*** Dropping Adapter {:?}", self.info.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let instance = Rc::new(RefCell::new(Instance::create("Name", 1).unwrap()));
        let _adapters = Adapter::enumerate(instance);
    }
}
