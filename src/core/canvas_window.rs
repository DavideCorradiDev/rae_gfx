extern crate gfx_hal as hal;

use std::{cell::RefCell, rc::Rc};

use super::{Canvas, Instance, Size, TextureFormat};
use crate::{halw, window};

#[derive(Debug)]
pub struct CanvasWindow
{
  window: window::Window,
  gpu: Rc<RefCell<halw::Gpu>>,
  surface: halw::Surface,
  canvas_color_format: TextureFormat,
  canvas_size: Size<u32>,
}

impl CanvasWindow
{
  const FRAME_COUNT: usize = 3;
  pub fn new(
    instance: &Instance,
    event_loop: &window::EventLoop,
  ) -> Result<Self, CanvasWindowCreationError>
  {
    let window = window::Window::new(event_loop)?;
    Self::with_window(instance, window)
  }

  fn with_window(
    instance: &Instance,
    window: window::Window,
  ) -> Result<Self, CanvasWindowCreationError>
  {
    let surface = halw::Surface::create(
      Rc::clone(&instance.instance_rc()),
      Rc::clone(&instance.gpu_rc()),
      &window,
    )?;
    let canvas_size = Size {
      width: 0,
      height: 0,
    };
    let mut canvas_window = Self {
      window,
      gpu: Rc::clone(&instance.gpu_rc()),
      surface,
      canvas_color_format: instance.canvas_color_format(),
      canvas_size,
    };
    canvas_window.configure_swapchain(instance.canvas_color_format())?;
    Ok(canvas_window)
  }

  fn configure_swapchain(
    &mut self,
    canvas_color_format: TextureFormat,
  ) -> Result<(), hal::window::CreationError>
  {
    let size = self.window.inner_size();
    let extent = hal::window::Extent2D {
      width: size.width,
      height: size.height,
    };
    let config = hal::window::SwapchainConfig {
      present_mode: hal::window::PresentMode::FIFO,
      composite_alpha_mode: hal::window::CompositeAlphaMode::POSTMULTIPLIED,
      format: canvas_color_format,
      extent: extent,
      image_count: Self::FRAME_COUNT as u32,
      image_layers: 1,
      image_usage: hal::window::DEFAULT_USAGE,
    };
    self.surface.configure_swapchain(config)
  }
}

impl Canvas for CanvasWindow {}

#[derive(Debug)]
pub enum CanvasWindowCreationError
{
  OsError(window::OsError),
  SurfaceCreationFailed(hal::window::InitError),
  SwapchainCreationFailed(hal::window::CreationError),
}

impl std::fmt::Display for CanvasWindowCreationError
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    match self
    {
      CanvasWindowCreationError::OsError(e) => write!(f, "OS Error ({})", e),
      CanvasWindowCreationError::SurfaceCreationFailed(e) =>
      {
        write!(f, "Surface creation failed ({})", e)
      }
      CanvasWindowCreationError::SwapchainCreationFailed(e) =>
      {
        write!(f, "Swapchain configuration failed ({})", e)
      }
    }
  }
}

impl std::error::Error for CanvasWindowCreationError
{
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
  {
    match self
    {
      CanvasWindowCreationError::OsError(e) => Some(e),
      CanvasWindowCreationError::SurfaceCreationFailed(e) => Some(e),
      CanvasWindowCreationError::SwapchainCreationFailed(e) => Some(e),
    }
  }
}

impl From<window::OsError> for CanvasWindowCreationError
{
  fn from(e: window::OsError) -> Self
  {
    CanvasWindowCreationError::OsError(e)
  }
}

impl From<hal::window::InitError> for CanvasWindowCreationError
{
  fn from(e: hal::window::InitError) -> Self
  {
    CanvasWindowCreationError::SurfaceCreationFailed(e)
  }
}

impl From<hal::window::CreationError> for CanvasWindowCreationError
{
  fn from(e: hal::window::CreationError) -> Self
  {
    CanvasWindowCreationError::SwapchainCreationFailed(e)
  }
}

#[cfg(test)]
mod tests
{
  use super::*;
  use crate::window::EventLoopExt;

  #[test]
  fn canvas_window_creation()
  {
    let instance = Instance::create().unwrap();
    let event_loop = window::EventLoop::new_any_thread();
    let _window = CanvasWindow::new(&instance, &event_loop).unwrap();
  }
}
