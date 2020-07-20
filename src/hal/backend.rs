#[cfg(not(any(
  feature = "dx11",
  feature = "dx12",
  feature = "metal",
  feature = "opengl",
  feature = "vulkan"
)))]
extern crate gfx_backend_empty as hal_backend;

#[cfg(feature = "dx11")]
extern crate gfx_backend_dx11 as hal_backend;

#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as hal_backend;

#[cfg(feature = "metal")]
extern crate gfx_backend_metal as hal_backend;

#[cfg(feature = "opengl")]
extern crate gfx_backend_gl as hal_backend;

#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan as hal_backend;

pub use hal_backend::Backend;
