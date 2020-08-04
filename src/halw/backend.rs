extern crate gfx_hal as hal;

#[cfg(all(
  target_os = "windows",
  not(any(
    feature = "empty",
    feature = "dx11",
    feature = "dx12",
    feature = "gl",
    feature = "metal",
    feature = "vulkan"
  ))
))]
extern crate gfx_backend_vulkan as hal_backend;

#[cfg(all(
  any(target_os = "macos", all(target_os = "ios", target_arch = "aarch64")),
  not(any(
    feature = "empty",
    feature = "dx11",
    feature = "dx12",
    feature = "gl",
    feature = "metal",
    feature = "vulkan"
  ))
))]
extern crate gfx_backend_vulkan as hal_backend;

#[cfg(feature = "empty")]
extern crate gfx_backend_empty as hal_backend;

#[cfg(feature = "dx11")]
extern crate gfx_backend_dx11 as hal_backend;

#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as hal_backend;

#[cfg(feature = "gl")]
extern crate gfx_backend_gl as hal_backend;

#[cfg(feature = "metal")]
extern crate gfx_backend_metal as hal_backend;

#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan as hal_backend;

pub use hal_backend::Backend;

pub type Adapter = hal::adapter::Adapter<Backend>;
pub type PhysicalDevice = <Backend as hal::Backend>::PhysicalDevice;
pub type Device = <Backend as hal::Backend>::Device;
pub type QueueGroup = hal::queue::family::QueueGroup<Backend>;
pub type QueueFamily = <Backend as hal::Backend>::QueueFamily;
pub type CommandQueue = <Backend as hal::Backend>::CommandQueue;
pub type GraphicsPipelineDesc<'a> = hal::pso::GraphicsPipelineDesc<'a, Backend>;
pub type SwapchainImage =
  <<Backend as hal::Backend>::Surface as hal::window::PresentationSurface<Backend>>::SwapchainImage;
