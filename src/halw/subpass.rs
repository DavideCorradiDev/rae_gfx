extern crate gfx_hal as hal;

use super::Backend;

pub type Subpass<'a> = hal::pass::Subpass<'a, Backend>;
