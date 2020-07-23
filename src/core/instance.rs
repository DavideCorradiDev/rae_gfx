extern crate gfx_hal as hal;

use std::{cell::RefCell, rc::Rc};

use hal::Instance as HalInstance;

use crate::halw;

pub struct Instance
{
  instance: Rc<RefCell<halw::Instance>>,
  adapter: halw::Adapter,
}

impl Instance
{
  pub const ENGINE_NAME: &'static str = "Red Ape Engine";
  pub const ENGINE_VERSION: u32 = 1;

  pub fn create() -> Result<Self, InstanceCreationError>
  {
    let instance = Rc::new(RefCell::new(Self::create_instance()?));
    let adapter = Self::select_adapter(&*instance.borrow())?;
    Ok(Self { instance, adapter })
  }

  fn create_instance() -> Result<halw::Instance, InstanceCreationError>
  {
    let instance =
      halw::Instance::create(Self::ENGINE_NAME, Self::ENGINE_VERSION)?;
    Ok(instance)
  }

  fn select_adapter(
    instance: &halw::Instance,
  ) -> Result<halw::Adapter, InstanceCreationError>
  {
    let mut adapters = instance.enumerate_adapters();
    adapters.retain(|a| {
      a.info.device_type == hal::adapter::DeviceType::DiscreteGpu
        || a.info.device_type == hal::adapter::DeviceType::IntegratedGpu
    });
    if adapters.is_empty()
    {
      return Err(InstanceCreationError::NoSuitableAdapter);
    }

    adapters.sort_by(|a, b| {
      if a.info.device_type == b.info.device_type
      {
        return std::cmp::Ordering::Equal;
      }
      else if a.info.device_type == hal::adapter::DeviceType::DiscreteGpu
      {
        return std::cmp::Ordering::Less;
      }
      else
      {
        return std::cmp::Ordering::Greater;
      }
    });
    Ok(adapters.remove(0))
  }
}

#[derive(Debug)]
pub enum InstanceCreationError
{
  UnsupportedBackend,
  NoSuitableAdapter,
}

impl std::fmt::Display for InstanceCreationError
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    match self
    {
      InstanceCreationError::UnsupportedBackend =>
      {
        write!(f, "Unsupported backend")
      }
      InstanceCreationError::NoSuitableAdapter =>
      {
        write!(f, "Could not find a suitable adapter")
      }
    }
  }
}

impl std::error::Error for InstanceCreationError
{
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
  {
    match self
    {
      _ => None,
    }
  }
}

impl From<hal::UnsupportedBackend> for InstanceCreationError
{
  fn from(_: hal::UnsupportedBackend) -> InstanceCreationError
  {
    InstanceCreationError::UnsupportedBackend
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn instance_creation()
  {
    let _instance = Instance::create();
  }
}
