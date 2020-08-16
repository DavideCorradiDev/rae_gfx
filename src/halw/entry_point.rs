extern crate gfx_hal as hal;

use super::Backend;

pub type EntryPoint<'a> = hal::pso::EntryPoint<'a, Backend>;
