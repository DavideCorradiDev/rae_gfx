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
