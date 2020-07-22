extern crate gfx_hal as hal;

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error
{
  InstanceCreationFailed(InstanceCreationError),
}

impl Display for Error
{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
  {
    match self
    {
      Error::InstanceCreationFailed(det) =>
      {
        write!(f, "Failed to create instance ({})", det)
      }
    }
  }
}

impl std::error::Error for Error
{
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
  {
    match self
    {
      Error::InstanceCreationFailed(det) => Some(det),
    }
  }
}

#[derive(Debug)]
pub enum InstanceCreationError
{
  UnsupportedBackend,
}

impl Display for InstanceCreationError
{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
  {
    match self
    {
      InstanceCreationError::UnsupportedBackend =>
      {
        write!(f, "Unsupported backend")
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
