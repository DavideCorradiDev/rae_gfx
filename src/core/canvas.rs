extern crate gfx_hal as hal;

pub trait Canvas
{
  fn begin_frame(&mut self) -> Result<(), BeginFrameError>;
  fn end_frame(&mut self) -> Result<(), EndFrameError>;
}

#[derive(Debug)]
pub enum BeginFrameError
{
  AlreadyProcessingFrame,
  ImageAcquisitionFailed(hal::window::AcquireError),
}

impl std::fmt::Display for BeginFrameError
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    match self
    {
      BeginFrameError::AlreadyProcessingFrame => write!(
        f,
        "Failed to begin frame: a frame is already being processed"
      ),
      BeginFrameError::ImageAcquisitionFailed(e) =>
      {
        write!(f, "Failed to begin frame: failed to acquire image ({})", e)
      }
    }
  }
}

impl std::error::Error for BeginFrameError
{
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
  {
    match self
    {
      BeginFrameError::ImageAcquisitionFailed(e) => Some(e),
      _ => None,
    }
  }
}

#[derive(Debug)]
pub struct EndFrameError {}

impl std::fmt::Display for EndFrameError
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "Failed to end frame")
  }
}

impl std::error::Error for EndFrameError
{
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
  {
    None
  }
}
