extern crate gfx_hal as hal;

use std::{cell::RefCell, rc::Rc};

use hal::queue::QueueFamily as HalQueueFamily;

use super::{BeginFrameError, Canvas, EndFrameError, Instance, TextureFormat};
use crate::{halw, window};

#[derive(Debug)]
pub struct CanvasWindow
{
  window: window::Window,
  gpu: Rc<RefCell<halw::Gpu>>,
  surface: halw::Surface,
  surface_color_format: TextureFormat,
  surface_extent: hal::window::Extent2D,
  cmd_buffers: Vec<halw::CommandBuffer>,
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

  pub fn inner_position(
    &self,
  ) -> Result<window::PhysicalPosition<i32>, CanvasWindowOperationError>
  {
    let pos = self.window.inner_position()?;
    Ok(pos)
  }

  pub fn outer_position(
    &self,
  ) -> Result<window::PhysicalPosition<i32>, CanvasWindowOperationError>
  {
    let pos = self.window.outer_position()?;
    Ok(pos)
  }

  pub fn set_outer_position<P>(&self, position: P)
  where
    P: Into<window::Position>,
  {
    self.window.set_outer_position(position);
  }

  pub fn inner_size(&self) -> window::PhysicalSize<u32>
  {
    self.window.inner_size()
  }

  pub fn outer_size(&self) -> window::PhysicalSize<u32>
  {
    self.window.outer_size()
  }

  pub fn set_inner_size<S>(
    &mut self,
    size: S,
  ) -> Result<(), CanvasWindowOperationError>
  where
    S: Into<window::Size>,
  {
    self.window.set_inner_size(size);
    self.resize_canvas_if_necessary()?;
    Ok(())
  }

  pub fn set_min_inner_size<S>(
    &mut self,
    min_size: Option<S>,
  ) -> Result<(), CanvasWindowOperationError>
  where
    S: Into<window::Size>,
  {
    self.window.set_min_inner_size(min_size);
    self.resize_canvas_if_necessary()?;
    Ok(())
  }

  pub fn set_max_inner_size<S>(
    &mut self,
    max_size: Option<S>,
  ) -> Result<(), CanvasWindowOperationError>
  where
    S: Into<window::Size>,
  {
    self.window.set_max_inner_size(max_size);
    self.resize_canvas_if_necessary()?;
    Ok(())
  }

  pub fn set_title(&self, title: &str)
  {
    self.window.set_title(title)
  }

  pub fn set_visible(&self, visible: bool)
  {
    self.window.set_visible(visible)
  }

  pub fn set_resizable(&self, resizable: bool)
  {
    self.window.set_resizable(resizable)
  }

  pub fn set_minimized(&self, minimized: bool)
  {
    self.window.set_minimized(minimized)
  }

  pub fn set_maximized(&self, maximized: bool)
  {
    self.window.set_maximized(maximized)
  }

  pub fn set_fullsceen(&self, fullscreen: Option<window::Fullscreen>)
  {
    self.window.set_fullscreen(fullscreen)
  }

  pub fn fullscreen(&self) -> Option<window::Fullscreen>
  {
    self.window.fullscreen()
  }

  pub fn set_decorations(&self, decorations: bool)
  {
    self.window.set_decorations(decorations)
  }

  pub fn set_always_on_top(&self, always_on_top: bool)
  {
    self.window.set_always_on_top(always_on_top)
  }

  pub fn set_window_icon(&self, window_icon: Option<window::Icon>)
  {
    self.window.set_window_icon(window_icon)
  }

  pub fn set_ime_position<P>(&self, position: P)
  where
    P: Into<window::Position>,
  {
    self.window.set_ime_position(position)
  }

  pub fn set_cursor_icon(&self, cursor: window::CursorIcon)
  {
    self.window.set_cursor_icon(cursor)
  }

  pub fn set_cursor_position<P>(
    &self,
    position: P,
  ) -> Result<(), CanvasWindowOperationError>
  where
    P: Into<window::Position>,
  {
    self.window.set_cursor_position(position)?;
    Ok(())
  }

  pub fn set_cursor_grab(
    &self,
    grab: bool,
  ) -> Result<(), CanvasWindowOperationError>
  {
    self.window.set_cursor_grab(grab)?;
    Ok(())
  }

  pub fn set_cursor_visible(&self, visible: bool)
  {
    self.window.set_cursor_visible(visible)
  }

  pub fn resize_canvas_if_necessary(
    &mut self,
  ) -> Result<(), hal::window::CreationError>
  {
    let current_size = self.inner_size();
    if self.surface_extent.width != current_size.width
      || self.surface_extent.height != current_size.height
    {
      self.configure_swapchain()?;
    }
    Ok(())
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
    let cmd_buffers = Self::create_command_buffers(instance)?;
    let mut canvas_window = Self {
      window,
      gpu: Rc::clone(&instance.gpu_rc()),
      surface,
      surface_color_format: instance.canvas_color_format(),
      surface_extent: hal::window::Extent2D {
        width: 0,
        height: 0,
      },
      cmd_buffers,
    };
    canvas_window.configure_swapchain()?;
    Ok(canvas_window)
  }

  fn create_command_buffers(
    instance: &Instance,
  ) -> Result<Vec<halw::CommandBuffer>, hal::device::OutOfMemory>
  {
    let cmd_pool = halw::CommandPool::create(
      Rc::clone(&instance.gpu_rc()),
      instance.queue_family().id(),
      hal::pool::CommandPoolCreateFlags::empty(),
    )?;
    Ok(halw::CommandBuffer::allocate(
      Rc::new(RefCell::new(cmd_pool)),
      hal::command::Level::Primary,
      Self::FRAME_COUNT,
    ))
  }

  fn configure_swapchain(&mut self) -> Result<(), hal::window::CreationError>
  {
    let size = self.window.inner_size();
    let extent = hal::window::Extent2D {
      width: size.width,
      height: size.height,
    };
    let config = hal::window::SwapchainConfig {
      present_mode: hal::window::PresentMode::FIFO,
      composite_alpha_mode: hal::window::CompositeAlphaMode::POSTMULTIPLIED,
      format: self.surface_color_format,
      extent: extent,
      image_count: Self::FRAME_COUNT as u32,
      image_layers: 1,
      image_usage: hal::window::DEFAULT_USAGE,
    };
    self.surface.configure_swapchain(config)?;
    self.surface_extent = extent;
    Ok(())
  }
}

impl Canvas for CanvasWindow
{
  fn begin_frame(&mut self) -> Result<(), BeginFrameError>
  {
    Ok(())
  }

  fn end_frame(&mut self) -> Result<(), EndFrameError>
  {
    Ok(())
  }
}

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

  pub fn with_inner_size<S: Into<window::Size>>(self, size: S) -> Self
  {
    CanvasWindowBuilder {
      builder: self.builder.with_inner_size(size),
    }
  }

  pub fn with_min_inner_size<S: Into<window::Size>>(self, size: S) -> Self
  {
    CanvasWindowBuilder {
      builder: self.builder.with_min_inner_size(size),
    }
  }

  pub fn with_max_inner_size<S: Into<window::Size>>(self, size: S) -> Self
  {
    CanvasWindowBuilder {
      builder: self.builder.with_max_inner_size(size),
    }
  }

  pub fn with_title<T: Into<String>>(self, title: T) -> Self
  {
    CanvasWindowBuilder {
      builder: self.builder.with_title(title),
    }
  }

  pub fn with_window_icon(self, icon: Option<window::Icon>) -> Self
  {
    CanvasWindowBuilder {
      builder: self.builder.with_window_icon(icon),
    }
  }

  pub fn with_fullscreen(self, monitor: Option<window::Fullscreen>) -> Self
  {
    CanvasWindowBuilder {
      builder: self.builder.with_fullscreen(monitor),
    }
  }

  pub fn with_resizable(self, resizable: bool) -> Self
  {
    CanvasWindowBuilder {
      builder: self.builder.with_resizable(resizable),
    }
  }

  pub fn with_maximized(self, maximized: bool) -> Self
  {
    CanvasWindowBuilder {
      builder: self.builder.with_maximized(maximized),
    }
  }

  pub fn with_visible(self, visible: bool) -> Self
  {
    CanvasWindowBuilder {
      builder: self.builder.with_visible(visible),
    }
  }

  pub fn with_transparent(self, transparent: bool) -> Self
  {
    CanvasWindowBuilder {
      builder: self.builder.with_transparent(transparent),
    }
  }

  pub fn with_decorations(self, decorations: bool) -> Self
  {
    CanvasWindowBuilder {
      builder: self.builder.with_decorations(decorations),
    }
  }

  pub fn with_always_on_top(self, always_on_top: bool) -> Self
  {
    CanvasWindowBuilder {
      builder: self.builder.with_always_on_top(always_on_top),
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
  OutOfMemory(hal::device::OutOfMemory),
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
      CanvasWindowCreationError::OutOfMemory(e) =>
      {
        write!(f, "Out of memory ({})", e)
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
      CanvasWindowCreationError::OutOfMemory(e) => Some(e),
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

impl From<hal::device::OutOfMemory> for CanvasWindowCreationError
{
  fn from(e: hal::device::OutOfMemory) -> Self
  {
    CanvasWindowCreationError::OutOfMemory(e)
  }
}

#[derive(Debug)]
pub enum CanvasWindowOperationError
{
  UnsupportedOperation(window::NotSupportedError),
  ExternalError(window::ExternalError),
  SwapchainConfigurationFailed(hal::window::CreationError),
}

impl std::fmt::Display for CanvasWindowOperationError
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    match self
    {
      CanvasWindowOperationError::UnsupportedOperation(e) =>
      {
        write!(f, "Unsupported operation ({})", e)
      }
      CanvasWindowOperationError::ExternalError(e) =>
      {
        write!(f, "External error ({})", e)
      }
      CanvasWindowOperationError::SwapchainConfigurationFailed(e) =>
      {
        write!(f, "Swapchain configuration failed ({})", e)
      }
    }
  }
}

impl std::error::Error for CanvasWindowOperationError
{
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
  {
    match self
    {
      CanvasWindowOperationError::UnsupportedOperation(e) => Some(e),
      CanvasWindowOperationError::ExternalError(e) => Some(e),
      CanvasWindowOperationError::SwapchainConfigurationFailed(e) => Some(e),
    }
  }
}

impl From<window::NotSupportedError> for CanvasWindowOperationError
{
  fn from(e: window::NotSupportedError) -> Self
  {
    CanvasWindowOperationError::UnsupportedOperation(e)
  }
}

impl From<window::ExternalError> for CanvasWindowOperationError
{
  fn from(e: window::ExternalError) -> Self
  {
    CanvasWindowOperationError::ExternalError(e)
  }
}

impl From<hal::window::CreationError> for CanvasWindowOperationError
{
  fn from(e: hal::window::CreationError) -> Self
  {
    CanvasWindowOperationError::SwapchainConfigurationFailed(e)
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
        .with_visible(false)
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
