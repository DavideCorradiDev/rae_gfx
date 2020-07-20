#[cfg(not(any(
  feature = "dx11",
  feature = "dx12",
  feature = "metal",
  feature = "opengl",
  feature = "vulkan"
)))]
extern crate gfx_backend_empty as backend;

#[cfg(feature = "dx11")]
extern crate gfx_backend_dx11 as backend;

#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as backend;

#[cfg(feature = "metal")]
extern crate gfx_backend_metal as backend;

#[cfg(feature = "opengl")]
extern crate gfx_backend_gl as backend;

#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan as backend;

use backend::Backend;
