extern crate gfx_hal as hal;
use super::Backend;

use std::ops::{Deref, DerefMut};

use hal::Instance as HalInstance;

use super::Instance;

#[derive(Debug)]
pub struct Adapter {
    value: hal::adapter::Adapter<Backend>,
}

impl Adapter {
    pub fn enumerate(instance: &Instance) -> Vec<Self> {
        instance
            .enumerate_adapters()
            .into_iter()
            .map(|x| Self { value: x })
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
