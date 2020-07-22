use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {}

impl Display for Error
{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "Placeholder")
  }
}

impl std::error::Error for Error
{
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
  {
    None
  }
}
