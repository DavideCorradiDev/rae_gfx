extern crate gfx_hal as hal;

use std::{cell::RefCell, rc::Rc};

use super::{Canvas, Instance, TextureFormat};
use crate::{halw, window};

#[derive(Debug)]
pub struct CanvasWindow
{
  window: window::Window,
  gpu: Rc<RefCell<halw::Gpu>>,
  surface: halw::Surface,
  canvas_color_format: TextureFormat,
  canvas_extent: hal::window::Extent2D,
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

  pub fn id(&self) -> window::WindowId
  {
    self.window.id()
  }

  pub fn scale_factor(&self) -> f64
  {
    self.window.scale_factor()
  }

  pub fn request_redraw(&self)
  {
    self.window.request_redraw()
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
    let mut canvas_window = Self {
      window,
      gpu: Rc::clone(&instance.gpu_rc()),
      surface,
      canvas_color_format: instance.canvas_color_format(),
      canvas_extent: hal::window::Extent2D {
        width: 0,
        height: 0,
      },
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
    self.surface.configure_swapchain(config)?;
    self.canvas_extent = extent;
    Ok(())
  }
}

impl Canvas for CanvasWindow {}

pub struct CanvasWindowBuilder
{
  builder: window::WindowBuilder,
}

impl CanvasWindowBuilder
{
  pub fn new() -> Self
  {
    CanvasWindowBuilder {
      builder: window::WindowBuilder::new(),
    }
  }

  pub fn with_title(self, title: &str) -> Self
  {
    CanvasWindowBuilder {
      builder: self.builder.with_title(title),
    }
  }

  pub fn with_visibility(self, visible: bool) -> Self
  {
    CanvasWindowBuilder {
      builder: self.builder.with_visible(visible),
    }
  }

  pub fn with_inner_size<S>(self, size: S) -> Self
  where
    S: Into<window::Size>,
  {
    CanvasWindowBuilder {
      builder: self.builder.with_inner_size(size),
    }
  }

  pub fn build<T>(
    self,
    instance: &Instance,
    window_target: &window::EventLoopWindowTarget<T>,
  ) -> Result<CanvasWindow, CanvasWindowCreationError>
  where
    T: 'static,
  {
    let window = self.builder.build(window_target)?;
    CanvasWindow::with_window(instance, window)
  }
}

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
  extern crate galvanic_assert;

  use galvanic_assert::{matchers::*, *};

  use super::*;
  use crate::window::EventLoopExt;

  struct TestFixture
  {
    pub instance: Instance,
    pub event_loop: window::EventLoop,
  }

  impl TestFixture
  {
    pub fn setup() -> Self
    {
      let instance = Instance::create().unwrap();
      let event_loop = window::EventLoop::new_any_thread();
      Self {
        instance,
        event_loop,
      }
    }

    pub fn new_window(&self) -> CanvasWindow
    {
      let b = CanvasWindowBuilder::new();
      b.with_title("Test")
        .with_inner_size(window::Size::Physical(window::PhysicalSize {
          width: 640,
          height: 480,
        }))
        .with_visibility(false)
        .build(&self.instance, &self.event_loop)
        .unwrap()
    }
  }

  #[test]
  fn new()
  {
    let tf = TestFixture::setup();
    let _window = CanvasWindow::new(&tf.instance, &tf.event_loop).unwrap();
  }

  #[test]
  fn id()
  {
    let tf = TestFixture::setup();
    let window1 = tf.new_window();
    let window2 = tf.new_window();
    assert_that!(&window1.id(), not(eq(window2.id())));
  }

  #[test]
  fn scale_factor()
  {
    let tf = TestFixture::setup();
    let window = tf.new_window();
    assert_that!(&window.scale_factor(), not(eq(0.)));
  }
}
