extern crate gfx_hal as hal;

use std::{cell::RefCell, mem::ManuallyDrop, rc::Rc};

use hal::{
  adapter::PhysicalDevice as HalPhysicalDevice,
  queue::QueueFamily as HalQueueFamily, window::Surface as HalSurface,
  Instance as HalInstance,
};

use super::TextureFormat;
use crate::{halw, window};

pub struct Instance
{
  instance: Rc<RefCell<halw::Instance>>,
  adapter: halw::Adapter,
  gpu: Rc<RefCell<halw::Gpu>>,
  canvas_color_format: TextureFormat,
}

impl Instance
{
  pub const ENGINE_NAME: &'static str = "Red Ape Engine";
  pub const ENGINE_VERSION: u32 = 1;

  pub fn create() -> Result<Self, InstanceCreationError>
  {
    let instance = Rc::new(RefCell::new(Self::create_instance()?));
    let adapter = Self::select_adapter(&*instance.borrow())?;
    let (_, _, mut dummy_surface) =
      Self::create_dummy_surface(Rc::clone(&instance))?;
    let gpu =
      Rc::new(RefCell::new(Self::open_device(&adapter, &dummy_surface)?));
    let canvas_color_format =
      Self::select_canvas_color_format(&adapter, &dummy_surface);
    Ok(Self {
      instance,
      adapter,
      gpu,
      canvas_color_format,
    })
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

  fn create_dummy_surface(
    instance: Rc<RefCell<halw::Instance>>,
  ) -> Result<
    (window::EventLoop, window::Window, halw::Surface),
    InstanceCreationError,
  >
  {
    let dummy_event_loop = window::EventLoop::new();
    let dummy_window = window::WindowBuilder::new()
      .with_visible(false)
      .build(&dummy_event_loop)
      .unwrap();
    let dummy_surface = halw::Surface::create(instance, &dummy_window)?;
    Ok((dummy_event_loop, dummy_window, dummy_surface))
  }

  fn select_canvas_color_format(
    adapter: &halw::Adapter,
    surface: &halw::Surface,
  ) -> hal::format::Format
  {
    let formats = surface.supported_formats(&adapter.physical_device);
    formats.map_or(hal::format::Format::Rgba8Srgb, |formats| {
      formats
        .iter()
        .find(|a| a.base_format().1 == hal::format::ChannelType::Srgb)
        .map(|a| *a)
        .unwrap_or(formats[0])
    })
  }

  fn select_queue_family<'a>(
    adapter: &'a halw::Adapter,
    surface: &halw::Surface,
  ) -> Result<&'a halw::QueueFamily, InstanceCreationError>
  {
    // Eventually add required constraints here.
    match adapter.queue_families.iter().find(|family| {
      surface.supports_queue_family(family)
        && family.queue_type().supports_graphics()
    })
    {
      Some(family) => Ok(family),
      None => Err(InstanceCreationError::NoSuitableQueueFamily),
    }
  }

  fn open_device(
    adapter: &halw::Adapter,
    surface: &halw::Surface,
  ) -> Result<halw::Gpu, InstanceCreationError>
  {
    // Eventually add required GPU features here.
    let queue_family = Self::select_queue_family(adapter, surface)?;
    let gpu = halw::Gpu::open(
      adapter,
      &[(queue_family, &[1.0])],
      hal::Features::empty(),
    )?;
    Ok(gpu)
  }
}

#[derive(Debug)]
pub enum InstanceCreationError
{
  UnsupportedBackend,
  NoSuitableAdapter,
  NoSuitableQueueFamily,
  SurfaceCreationFailed,
  DeviceCreationFailed,
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
      InstanceCreationError::NoSuitableQueueFamily =>
      {
        write!(f, "Could not find a suitable queue family")
      }
      InstanceCreationError::SurfaceCreationFailed =>
      {
        write!(f, "Failed to create window surface")
      }
      InstanceCreationError::DeviceCreationFailed =>
      {
        write!(f, "Failed to create device")
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

impl From<hal::window::InitError> for InstanceCreationError
{
  fn from(_: hal::window::InitError) -> InstanceCreationError
  {
    InstanceCreationError::SurfaceCreationFailed
  }
}

impl From<hal::device::CreationError> for InstanceCreationError
{
  fn from(_: hal::device::CreationError) -> InstanceCreationError
  {
    InstanceCreationError::DeviceCreationFailed
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
