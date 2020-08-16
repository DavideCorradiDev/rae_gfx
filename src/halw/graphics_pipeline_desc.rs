extern crate gfx_hal as hal;

use super::Backend;

pub use hal::pso::{Primitive, Rasterizer};
pub type GraphicsPipelineDesc<'a> = hal::pso::GraphicsPipelineDesc<'a, Backend>;
