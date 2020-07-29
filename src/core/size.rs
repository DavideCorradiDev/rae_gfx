extern crate gfx_hal as hal;

#[derive(Debug, Clone, Copy)]
pub struct Size<T>
{
  pub width: T,
  pub height: T,
}

impl<T> Size<T>
{
  pub fn new(width: T, height: T) -> Self
  {
    Self { width, height }
  }
}

impl From<hal::window::Extent2D> for Size<u32>
{
  fn from(extent: hal::window::Extent2D) -> Self
  {
    Self {
      width: extent.width,
      height: extent.height,
    }
  }
}

impl From<Size<u32>> for hal::window::Extent2D
{
  fn from(size: Size<u32>) -> Self
  {
    Self {
      width: size.width,
      height: size.height,
    }
  }
}
