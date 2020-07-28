use crate::{window, window::EventLoopExt};

pub struct CanvasWindow
{
  window: window::Window,
}

impl CanvasWindow
{
  pub fn new(
    event_loop: &window::EventLoop,
  ) -> Result<Self, CanvasWindowCreationError>
  {
    let window = window::Window::new(event_loop)?;
    Ok(Self { window })
  }
}

#[derive(Debug)]
pub enum CanvasWindowCreationError
{
  OsError(window::OsError),
}

impl std::fmt::Display for CanvasWindowCreationError
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    match self
    {
      CanvasWindowCreationError::OsError(e) => write!(f, "OS Error ({})", e),
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

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn canvas_window_creation()
  {
    let event_loop = window::EventLoop::new_any_thread();
    let _window = CanvasWindow::new(&event_loop);
  }
}
