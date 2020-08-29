use rae_app::{
    application::Application,
    event::{mouse, ControlFlow, DeviceId, EventHandler, EventLoop},
    window,
    window::WindowId,
};

use rae_gfx::wgpu::core::{
    CanvasWindow, CanvasWindowBuilder, Instance, InstanceConfig, InstanceCreationError,
    WindowWithInstanceCreationError,
};

#[derive(Debug)]
struct ApplicationImpl {
    window: CanvasWindow,
    instance: Instance,
}

impl EventHandler<ApplicationError, ApplicationEvent> for ApplicationImpl {
    type Error = ApplicationError;
    type CustomEvent = ApplicationEvent;

    fn new(event_loop: &EventLoop<Self::CustomEvent>) -> Result<Self, Self::Error> {
        let (window, instance) = unsafe {
            CanvasWindowBuilder::new()
                .with_inner_size(window::Size::Physical(window::PhysicalSize {
                    width: 800,
                    height: 800,
                }))
                .build_with_instance(&InstanceConfig::high_performance(), event_loop)?
        };
        Ok(Self { window, instance })
    }

    fn on_resized(
        &mut self,
        wid: WindowId,
        _size: window::PhysicalSize<u32>,
    ) -> Result<ControlFlow, Self::Error> {
        if wid == self.window.id() {
            self.window.reconfigure_swap_chain(&self.instance);
        }
        Ok(ControlFlow::Continue)
    }

    fn on_cursor_moved(
        &mut self,
        _wid: WindowId,
        _device_id: DeviceId,
        _position: window::PhysicalPosition<f64>,
    ) -> Result<ControlFlow, Self::Error> {
        Ok(ControlFlow::Continue)
    }

    fn on_mouse_button_released(
        &mut self,
        _wid: WindowId,
        _device_id: DeviceId,
        _button: mouse::Button,
    ) -> Result<ControlFlow, Self::Error> {
        Ok(ControlFlow::Continue)
    }

    fn on_variable_update(&mut self, _dt: std::time::Duration) -> Result<ControlFlow, Self::Error> {
        Ok(ControlFlow::Continue)
    }
}

type ApplicationEvent = ();

#[derive(Debug)]
enum ApplicationError {
    WindowCreationFailed(window::OsError),
    InstanceCreationFailed(InstanceCreationError),
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::WindowCreationFailed(e) => {
                write!(f, "Window creation failed ({})", e)
            }
            ApplicationError::InstanceCreationFailed(e) => {
                write!(f, "Instance creation failed ({})", e)
            }
        }
    }
}

impl std::error::Error for ApplicationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ApplicationError::WindowCreationFailed(e) => Some(e),
            ApplicationError::InstanceCreationFailed(e) => Some(e),
        }
    }
}

impl From<window::OsError> for ApplicationError {
    fn from(e: window::OsError) -> Self {
        ApplicationError::WindowCreationFailed(e)
    }
}

impl From<InstanceCreationError> for ApplicationError {
    fn from(e: InstanceCreationError) -> Self {
        ApplicationError::InstanceCreationFailed(e)
    }
}

impl From<WindowWithInstanceCreationError> for ApplicationError {
    fn from(e: WindowWithInstanceCreationError) -> Self {
        match e {
            WindowWithInstanceCreationError::WindowCreationFailed(e) => {
                ApplicationError::WindowCreationFailed(e)
            }
            WindowWithInstanceCreationError::InstanceCreationFailed(e) => {
                ApplicationError::InstanceCreationFailed(e)
            }
        }
    }
}

fn main() {
    const FIXED_FRAMERATE: u64 = 30;
    const VARIABLE_FRAMERATE_CAP: u64 = 60;
    Application::<ApplicationImpl, _, _>::new(FIXED_FRAMERATE, Some(VARIABLE_FRAMERATE_CAP)).run();
}
