extern crate rae_app;

use rae_app::{
    application::Application,
    event::{ControlFlow, EventHandler, EventLoop},
    window,
};

use rae_gfx::core::{
    BeginFrameError, Canvas, CanvasWindow, CanvasWindowBuilder, CanvasWindowCreationError,
    EndFrameError, Instance, InstanceCreationError,
};

type ApplicationEvent = ();

#[derive(Debug)]
enum ApplicationError {
    InstanceCreationFailed(InstanceCreationError),
    WindowCreationFailed(CanvasWindowCreationError),
    BeginFrameFailed(BeginFrameError),
    EndFrameFailed(EndFrameError),
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
            ApplicationError::BeginFrameFailed(e) => write!(f, "Render frame start failed ({})", e),
            ApplicationError::EndFrameFailed(e) => write!(f, "Render frame end failed ({})", e),
        }
    }
}

impl std::error::Error for ApplicationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ApplicationError::InstanceCreationFailed(e) => Some(e),
            ApplicationError::WindowCreationFailed(e) => Some(e),
            ApplicationError::BeginFrameFailed(e) => Some(e),
            ApplicationError::EndFrameFailed(e) => Some(e),
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

impl From<BeginFrameError> for ApplicationError {
    fn from(e: BeginFrameError) -> Self {
        ApplicationError::BeginFrameFailed(e)
    }
}

impl From<EndFrameError> for ApplicationError {
    fn from(e: EndFrameError) -> Self {
        ApplicationError::EndFrameFailed(e)
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

    fn on_variable_update(&mut self, _: std::time::Duration) -> Result<ControlFlow, Self::Error> {
        self.window.begin_frame()?;
        self.window.end_frame()?;
        Ok(ControlFlow::Continue)
    }
}

fn main() {
    const FIXED_FRAMERATE: u64 = 30;
    const VARIABLE_FRAMERATE_CAP: u64 = 60;
    Application::<ApplicationImpl, _, _>::new(FIXED_FRAMERATE, Some(VARIABLE_FRAMERATE_CAP)).run();
}
