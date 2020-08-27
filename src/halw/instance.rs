extern crate gfx_hal as hal;

use hal::Instance as HalInstance;
use std::{
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
};

use super::Backend;

pub struct Instance {
    value: <Backend as hal::Backend>::Instance,
}

impl Instance {
    pub fn create(name: &str, version: u32) -> Result<Self, hal::UnsupportedBackend> {
        let instance = <Backend as hal::Backend>::Instance::create(name, version)?;
        Ok(Self { value: instance })
    }
}

impl Deref for Instance {
    type Target = <Backend as hal::Backend>::Instance;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for Instance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Debug for Instance {
    #[cfg(not(feature = "gl"))]
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Instance {{ value: {:?} }}", self.value)
    }

    #[cfg(feature = "gl")]
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Instance {{}}")
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        println!("*** Dropping Instance {:?}", self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let _instance = Instance::create("Name", 1).unwrap();
    }
}
