extern crate rae_app;

use rae_app::{
    application::Application,
    event::{
        controller, keyboard, mouse, touch, ControlFlow, DeviceId, EventHandler, EventLoop,
        EventLoopClosed, EventLoopProxy, EventLoopStartCause, ScrollDelta,
    },
    window,
    window::{PhysicalPosition, PhysicalSize, Size, Window, WindowBuilder, WindowId},
};

use rae_gfx::core::{
    CanvasWindow, CanvasWindowBuilder, CanvasWindowCreationError, Instance, InstanceCreationError,
};

type ApplicationEvent = ();

#[derive(Debug)]
enum ApplicationError {
    InstanceCreationFailed(InstanceCreationError),
    WindowCreationFailed(CanvasWindowCreationError),
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::InstanceCreationFailed(e) => {
                write!(f, "Instance creation failed ({})", e)
            }
            ApplicationError::WindowCreationFailed(e) => {
                write!(f, "Window creation failed ({})", e)
            }
        }
    }
}

impl std::error::Error for ApplicationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ApplicationError::InstanceCreationFailed(e) => Some(e),
            ApplicationError::WindowCreationFailed(e) => Some(e),
        }
    }
}

impl From<InstanceCreationError> for ApplicationError {
    fn from(e: InstanceCreationError) -> Self {
        ApplicationError::InstanceCreationFailed(e)
    }
}

impl From<CanvasWindowCreationError> for ApplicationError {
    fn from(e: CanvasWindowCreationError) -> Self {
        ApplicationError::WindowCreationFailed(e)
    }
}

#[derive(Debug)]
struct ApplicationImpl {
    instance: Instance,
    window: CanvasWindow,
}

impl EventHandler<ApplicationError, ApplicationEvent> for ApplicationImpl {
    type Error = ApplicationError;
    type CustomEvent = ApplicationEvent;

    fn new(event_loop: &EventLoop<Self::CustomEvent>) -> Result<Self, Self::Error> {
        let instance = Instance::create()?;
        let window = CanvasWindowBuilder::new()
            .with_title("Triangle")
            .with_inner_size(window::Size::Physical(window::PhysicalSize {
                width: 800,
                height: 600,
            }))
            .build(&instance, event_loop)?;
        Ok(Self { instance, window })
    }
}

fn main() {
    println!("Hello world!");
}
