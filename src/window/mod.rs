extern crate winit;

pub use winit::{
  dpi::*,
  error::*,
  event::*,
  event_loop::{ControlFlow, EventLoopWindowTarget},
  window::*,
};

pub type EventLoop = winit::event_loop::EventLoop<()>;
pub type Event<'a> = winit::event::Event<'a, ()>;

pub mod keyboard
{
  pub use winit::event::{ModifiersState, ScanCode, VirtualKeyCode as KeyCode};
}

#[cfg(target_os = "windows")]
pub use winit::platform::windows::EventLoopExtWindows as EventLoopExt;

#[cfg(target_os = "linux")]
pub use winit::platform::unix::EventLoopExtUnix as EventLoopExt;

#[cfg(target_os = "macos")]
pub use winit::platform::macos::EventLoopExtMacos as EventLoopExt;

#[cfg(target_os = "ios")]
pub use winit::platform::ios::EventLoopExtIos as EventLoopExt;

#[cfg(target_os = "android")]
pub use winit::platform::android::EventLoopExtAndroid as EventLoopExt;
