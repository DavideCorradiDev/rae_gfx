extern crate gfx_hal as hal;

use super::Backend;

pub type Adapter = hal::adapter::Adapter<Backend>;
