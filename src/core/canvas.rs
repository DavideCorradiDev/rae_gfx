extern crate gfx_hal as hal;

pub trait Canvas
{
  fn is_processing_frame(&self) -> bool;
  fn begin_frame(&mut self) -> Result<(), BeginFrameError>;
  fn end_frame(&mut self) -> Result<(), EndFrameError>;
  fn synchronize(&self) -> Result<(), SynchronizeFrameError>;
}

#[derive(Debug)]
pub enum BeginFrameError
{
  AlreadyProcessingFrame,
  ImageAcquisitionFailed(hal::window::AcquireError),
  FrameSynchronizationFailed(SynchronizeFrameError),
  OutOfMemory(hal::device::OutOfMemory),
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
      BeginFrameError::FrameSynchronizationFailed(e) => write!(
        f,
        "Failed to begin frame: failed to synchronize frame ({})",
        e
      ),
      BeginFrameError::OutOfMemory(e) =>
      {
        write!(f, "Failed to begin frame: out of memory ({})", e)
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
      BeginFrameError::FrameSynchronizationFailed(e) => Some(e),
      BeginFrameError::OutOfMemory(e) => Some(e),
      _ => None,
    }
  }
}

impl From<hal::window::AcquireError> for BeginFrameError
{
  fn from(e: hal::window::AcquireError) -> Self
  {
    BeginFrameError::ImageAcquisitionFailed(e)
  }
}

impl From<SynchronizeFrameError> for BeginFrameError
{
  fn from(e: SynchronizeFrameError) -> Self
  {
    BeginFrameError::FrameSynchronizationFailed(e)
  }
}

impl From<hal::device::OutOfMemory> for BeginFrameError
{
  fn from(e: hal::device::OutOfMemory) -> Self
  {
    BeginFrameError::OutOfMemory(e)
  }
}

#[derive(Debug)]
pub enum EndFrameError
{
  NotProcessingFrame,
  ImageAcquisitionFailed,
  SurfacePresentationFailed(hal::window::PresentError),
}

impl std::fmt::Display for EndFrameError
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    match self
    {
      EndFrameError::NotProcessingFrame =>
      {
        write!(f, "No frame is being processed")
      }
      EndFrameError::ImageAcquisitionFailed =>
      {
        write!(f, "Failed to acquire image")
      }
      EndFrameError::SurfacePresentationFailed(e) =>
      {
        write!(f, "Failed to present surface ({})", e)
      }
    }
  }
}

impl std::error::Error for EndFrameError
{
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
  {
    match self
    {
      EndFrameError::SurfacePresentationFailed(e) => Some(e),
      _ => None,
    }
  }
}

impl From<hal::window::PresentError> for EndFrameError
{
  fn from(e: hal::window::PresentError) -> Self
  {
    EndFrameError::SurfacePresentationFailed(e)
  }
}

#[derive(Debug)]
pub enum SynchronizeFrameError
{
  OutOfMemory(hal::device::OutOfMemory),
  DeviceLost(hal::device::DeviceLost),
}

impl std::fmt::Display for SynchronizeFrameError
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    match self
    {
      SynchronizeFrameError::OutOfMemory(e) =>
      {
        write!(f, "Falied to synchronize frame: out of memory ({})", e)
      }
      SynchronizeFrameError::DeviceLost(e) =>
      {
        write!(f, "Failed to synchronize frame: device lost ({})", e)
      }
    }
  }
}

impl std::error::Error for SynchronizeFrameError
{
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
  {
    match self
    {
      SynchronizeFrameError::OutOfMemory(e) => Some(e),
      SynchronizeFrameError::DeviceLost(e) => Some(e),
    }
  }
}

impl From<hal::device::OutOfMemory> for SynchronizeFrameError
{
  fn from(e: hal::device::OutOfMemory) -> Self
  {
    SynchronizeFrameError::OutOfMemory(e)
  }
}

impl From<hal::device::DeviceLost> for SynchronizeFrameError
{
  fn from(e: hal::device::DeviceLost) -> Self
  {
    SynchronizeFrameError::DeviceLost(e)
  }
}

impl From<hal::device::OomOrDeviceLost> for SynchronizeFrameError
{
  fn from(e: hal::device::OomOrDeviceLost) -> Self
  {
    match e
    {
      hal::device::OomOrDeviceLost::OutOfMemory(e) =>
      {
        SynchronizeFrameError::OutOfMemory(e)
      }
      hal::device::OomOrDeviceLost::DeviceLost(e) =>
      {
        SynchronizeFrameError::DeviceLost(e)
      }
    }
  }
}
