extern crate winit;

pub use winit::{
    dpi::*,
    error::*,
    event::{Event, *},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::*,
};

pub mod keyboard {
    pub use winit::event::{ModifiersState, ScanCode, VirtualKeyCode as KeyCode};
}

#[cfg(target_os = "windows")]
pub use winit::platform::windows::EventLoopExtWindows as EventLoopExt;

#[cfg(target_os = "linux")]
pub use winit::platform::unix::EventLoopExtUnix as EventLoopExt;
