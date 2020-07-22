extern crate gfx_hal as hal;

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

pub type Instance = <Backend as hal::Backend>::Instance;
pub type Adapter = hal::adapter::Adapter<Backend>;
pub type PhysicalDevice = <Backend as hal::Backend>::PhysicalDevice;
pub type Device = <Backend as hal::Backend>::Device;
pub type Gpu = hal::adapter::Gpu<Backend>;
// Needs RAII wrapper.
pub type Surface = <Backend as hal::Backend>::Surface;
// Needs RAII wrapper.
pub type Swapchain = <Backend as hal::Backend>::Swapchain;
pub type QueueFamily = <Backend as hal::Backend>::QueueFamily;
pub type CommandQueue = <Backend as hal::Backend>::CommandQueue;
// Needs RAII wrapper.
pub type ImageView = <Backend as hal::Backend>::ImageView;
// Needs RAII wrapper.
pub type PipelineCache = <Backend as hal::Backend>::PipelineCache;
pub type GraphicsPipelineDesc<'a> = hal::pso::GraphicsPipelineDesc<'a, Backend>;
