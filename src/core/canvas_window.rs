extern crate gfx_hal as hal;

use std::{cell::RefCell, rc::Rc};

use super::{Canvas, Instance};
use crate::{halw, window, window::EventLoopExt};

#[derive(Debug)]
pub struct CanvasWindow
{
  window: window::Window,
  gpu: Rc<RefCell<halw::Gpu>>,
  surface: halw::Surface,
}

impl CanvasWindow
{
  pub fn new(
    instance: &Instance,
    event_loop: &window::EventLoop,
  ) -> Result<Self, CanvasWindowCreationError>
  {
    let window = window::Window::new(event_loop)?;
    Self::with_window(instance, window)
  }

  pub fn with_window(
    instance: &Instance,
    window: window::Window,
  ) -> Result<Self, CanvasWindowCreationError>
  {
    let surface =
      halw::Surface::create(Rc::clone(&instance.instance_rc()), &window)?;
    Ok(Self {
      window,
      gpu: Rc::clone(&instance.gpu_rc()),
      surface,
    })
  }
}

impl Canvas for CanvasWindow {}

#[derive(Debug)]
pub enum CanvasWindowCreationError
{
  OsError(window::OsError),
  SurfaceCreationFailed(hal::window::InitError),
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

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn canvas_window_creation()
  {
    let instance = Instance::create().unwrap();
    let event_loop = window::EventLoop::new_any_thread();
    let _window = CanvasWindow::new(&instance, &event_loop).unwrap();
  }
}
