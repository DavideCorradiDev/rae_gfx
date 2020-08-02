extern crate gfx_hal as hal;

use std::{borrow::Borrow, cell::RefCell, ops::DerefMut, rc::Rc};

use hal::{
  command::CommandBuffer as HalCommandBuffer,
  queue::CommandQueue as HalCommandQueue,
  window::PresentationSurface as HalPresentatationSurface,
};

use super::{
  BeginFrameError, Canvas, EndFrameError, Instance, SynchronizeFrameError,
  TextureFormat,
};
use crate::{halw, window};

#[derive(Debug)]
pub struct CanvasWindow
{
  window: window::Window,
  gpu: Rc<RefCell<halw::Gpu>>,
  render_pass: halw::RenderPass,
  surface: halw::Surface,
  surface_color_format: TextureFormat,
  surface_extent: hal::window::Extent2D,
  cmd_buffers: Vec<halw::CommandBuffer>,
  semaphores: Vec<halw::Semaphore>,
  fences: Vec<halw::Fence>,
  current_image: Option<halw::SwapchainImage>,
  current_framebuffer: Option<halw::Framebuffer>,
  current_frame_idx: usize,
}

impl CanvasWindow
{
  const IMAGE_COUNT: usize = 3;

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
    let render_pass = Self::create_render_pass(instance)?;
    let surface = halw::Surface::create(
      Rc::clone(&instance.instance_rc()),
      Rc::clone(&instance.adapter_rc()),
      Rc::clone(&instance.gpu_rc()),
      &window,
    )?;
    let cmd_buffers = Self::create_command_buffers(instance)?;
    let semaphores = Self::create_semaphores(instance)?;
    let fences = Self::create_fences(instance)?;
    let mut canvas_window = Self {
      window,
      gpu: Rc::clone(&instance.gpu_rc()),
      render_pass,
      surface,
      surface_color_format: instance.canvas_color_format(),
      surface_extent: hal::window::Extent2D {
        width: 0,
        height: 0,
      },
      cmd_buffers,
      semaphores,
      fences,
      current_image: None,
      current_framebuffer: None,
      current_frame_idx: 0,
    };
    canvas_window.configure_swapchain()?;
    Ok(canvas_window)
  }

  fn create_render_pass(
    instance: &Instance,
  ) -> Result<halw::RenderPass, hal::device::OutOfMemory>
  {
    let color_attachment = hal::pass::Attachment {
      format: Some(instance.canvas_color_format()),
      samples: 1,
      ops: hal::pass::AttachmentOps::new(
        hal::pass::AttachmentLoadOp::Clear,
        hal::pass::AttachmentStoreOp::Store,
      ),
      stencil_ops: hal::pass::AttachmentOps::DONT_CARE,
      layouts: hal::image::Layout::Undefined..hal::image::Layout::Present,
    };
    let subpass = hal::pass::SubpassDesc {
      colors: &[(0, hal::image::Layout::ColorAttachmentOptimal)],
      depth_stencil: None,
      inputs: &[],
      resolves: &[],
      preserves: &[],
    };
    halw::RenderPass::create(
      Rc::clone(&instance.gpu_rc()),
      &[color_attachment],
      &[subpass],
      &[],
    )
  }

  fn create_command_buffers(
    instance: &Instance,
  ) -> Result<Vec<halw::CommandBuffer>, hal::device::OutOfMemory>
  {
    let cmd_pool = halw::CommandPool::create(
      Rc::clone(&instance.gpu_rc()),
      instance.gpu().queue_groups[0].family,
      hal::pool::CommandPoolCreateFlags::empty(),
    )?;
    Ok(halw::CommandBuffer::allocate(
      Rc::new(RefCell::new(cmd_pool)),
      hal::command::Level::Primary,
      Self::IMAGE_COUNT,
    ))
  }

  fn create_semaphores(
    instance: &Instance,
  ) -> Result<Vec<halw::Semaphore>, hal::device::OutOfMemory>
  {
    let mut semaphores = Vec::with_capacity(Self::IMAGE_COUNT);
    for _ in 0..Self::IMAGE_COUNT
    {
      semaphores.push(halw::Semaphore::create(Rc::clone(&instance.gpu_rc()))?);
    }
    Ok(semaphores)
  }

  fn create_fences(
    instance: &Instance,
  ) -> Result<Vec<halw::Fence>, hal::device::OutOfMemory>
  {
    let mut fences = Vec::with_capacity(Self::IMAGE_COUNT);
    for _ in 0..Self::IMAGE_COUNT
    {
      fences.push(halw::Fence::create(Rc::clone(&instance.gpu_rc()), true)?);
    }
    Ok(fences)
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
      image_count: Self::IMAGE_COUNT as u32,
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
  fn image_count(&self) -> usize
  {
    CanvasWindow::IMAGE_COUNT
  }

  fn is_processing_frame(&self) -> bool
  {
    self.current_framebuffer.is_some()
  }

  fn begin_frame(&mut self) -> Result<(), BeginFrameError>
  {
    if self.is_processing_frame()
    {
      return Err(BeginFrameError::AlreadyProcessingFrame);
    }

    self.synchronize()?;

    let (image, _) = unsafe { self.surface.acquire_image(!0) }?;
    let framebuffer = halw::Framebuffer::create(
      Rc::clone(&self.gpu),
      &self.render_pass,
      std::iter::once(image.borrow()),
      hal::image::Extent {
        width: self.surface_extent.width,
        height: self.surface_extent.height,
        depth: 1,
      },
    )?;
    let viewport_rect = hal::pso::Rect {
      x: 0,
      y: 0,
      w: self.surface_extent.width as i16,
      h: self.surface_extent.height as i16,
    };
    let viewport = hal::pso::Viewport {
      rect: viewport_rect,
      depth: 0.0..1.0,
    };
    let cmd_buf = &mut self.cmd_buffers[self.current_frame_idx];

    unsafe {
      cmd_buf.reset(true);
      cmd_buf.begin_primary(hal::command::CommandBufferFlags::ONE_TIME_SUBMIT);
      cmd_buf.set_scissors(0, &[viewport.rect]);
      cmd_buf.set_viewports(
        0,
        &[hal::pso::Viewport {
          rect: viewport_rect,
          depth: 0.0..1.0,
        }],
      );
      cmd_buf.begin_render_pass(
        &self.render_pass,
        &framebuffer,
        viewport.rect,
        &[hal::command::ClearValue {
          color: hal::command::ClearColor {
            float32: [0., 0., 0., 1.],
          },
        }],
        hal::command::SubpassContents::Inline,
      )
    }

    self.current_image = Some(image);
    self.current_framebuffer = Some(framebuffer);
    Ok(())
  }

  fn end_frame(&mut self) -> Result<(), EndFrameError>
  {
    if !self.is_processing_frame()
    {
      return Err(EndFrameError::NotProcessingFrame);
    }

    let cmd_buf = &mut self.cmd_buffers[self.current_frame_idx].deref_mut();
    unsafe {
      cmd_buf.end_render_pass();
      cmd_buf.finish();
    }

    let fence = &self.fences[self.current_frame_idx];
    let semaphore = &*self.semaphores[self.current_frame_idx];
    let queue = &mut self.gpu.borrow_mut().queue_groups[0].queues[0];
    let image = match std::mem::replace(&mut self.current_image, None)
    {
      Some(image) => image,
      None => return Err(EndFrameError::ImageAcquisitionFailed),
    };
    let _framebuffer = std::mem::replace(&mut self.current_framebuffer, None);
    self.current_frame_idx = (self.current_frame_idx + 1) % Self::IMAGE_COUNT;
    let submission = hal::queue::Submission {
      command_buffers: std::iter::once(&*cmd_buf),
      wait_semaphores: None,
      signal_semaphores: std::iter::once(semaphore),
    };
    unsafe {
      queue.submit(submission, Some(fence));
      queue.present_surface(&mut self.surface, image, Some(semaphore))?;
    }

    Ok(())
  }

  fn synchronize(&self) -> Result<(), SynchronizeFrameError>
  {
    let fence = &self.fences[self.current_frame_idx];
    fence.wait(!0)?;
    fence.reset()?;
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
  fn window_creation()
  {
    let tf = TestFixture::setup();
    let _window = tf.new_window();
  }

  #[test]
  fn image_count()
  {
    let tf = TestFixture::setup();
    let window = tf.new_window();
    assert_that!(&window.image_count(), eq(3));
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
