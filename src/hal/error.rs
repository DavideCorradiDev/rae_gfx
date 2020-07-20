use std::fmt;

#[derive(Debug)]
pub enum Error
{
  UnsupportedBackend,
}

impl fmt::Display for Error
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
  {
    match self
    {
      Error::UnsupportedBackend => write!(f, "Unsupported backend"),
    }
  }
}
