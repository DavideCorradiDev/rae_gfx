use rae_app::{
    event::{EventLoop, EventLoopWindowTarget},
    window,
    window::{ExternalError, NotSupportedError, OsError, Window, WindowBuilder, WindowId},
};

use super::{Instance, Surface};

#[derive(Debug)]
pub struct CanvasWindow {
    surface: Surface,
    window: Window,
}

impl CanvasWindow {
    pub unsafe fn new<T: 'static>(
        instance: &Instance,
        event_loop: &EventLoop<T>,
    ) -> Result<Self, CanvasWindowCreationError> {
        let window = Window::new(event_loop)?;
        Self::with_window(instance, window)
    }

    pub fn id(&self) -> WindowId {
        self.window.id()
    }

    pub fn scale_factor(&self) -> f64 {
        self.window.scale_factor()
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw()
    }

    pub fn inner_position(
        &self,
    ) -> Result<window::PhysicalPosition<i32>, CanvasWindowOperationError> {
        let pos = self.window.inner_position()?;
        Ok(pos)
    }

    pub fn outer_position(
        &self,
    ) -> Result<window::PhysicalPosition<i32>, CanvasWindowOperationError> {
        let pos = self.window.outer_position()?;
        Ok(pos)
    }

    pub fn set_outer_position<P>(&self, position: P)
    where
        P: Into<window::Position>,
    {
        self.window.set_outer_position(position);
    }

    pub fn inner_size(&self) -> window::PhysicalSize<u32> {
        self.window.inner_size()
    }

    pub fn outer_size(&self) -> window::PhysicalSize<u32> {
        self.window.outer_size()
    }

    pub fn set_inner_size<S>(&mut self, size: S) -> Result<(), CanvasWindowOperationError>
    where
        S: Into<window::Size>,
    {
        self.window.set_inner_size(size);
        // self.resize_canvas_if_necessary()?;
        Ok(())
    }

    pub fn set_min_inner_size<S>(
        &mut self,
        min_size: Option<S>,
    ) -> Result<(), CanvasWindowOperationError>
    where
        S: Into<window::Size>,
    {
        self.window.set_min_inner_size(min_size);
        // self.resize_canvas_if_necessary()?;
        Ok(())
    }

    pub fn set_max_inner_size<S>(
        &mut self,
        max_size: Option<S>,
    ) -> Result<(), CanvasWindowOperationError>
    where
        S: Into<window::Size>,
    {
        self.window.set_max_inner_size(max_size);
        // self.resize_canvas_if_necessary()?;
        Ok(())
    }

    pub fn set_title(&self, title: &str) {
        self.window.set_title(title)
    }

    pub fn set_visible(&self, visible: bool) {
        self.window.set_visible(visible)
    }

    pub fn set_resizable(&self, resizable: bool) {
        self.window.set_resizable(resizable)
    }

    pub fn set_minimized(&self, minimized: bool) {
        self.window.set_minimized(minimized)
    }

    pub fn set_maximized(&self, maximized: bool) {
        self.window.set_maximized(maximized)
    }

    pub fn set_fullsceen(&self, fullscreen: Option<window::Fullscreen>) {
        self.window.set_fullscreen(fullscreen)
    }

    pub fn fullscreen(&self) -> Option<window::Fullscreen> {
        self.window.fullscreen()
    }

    pub fn set_decorations(&self, decorations: bool) {
        self.window.set_decorations(decorations)
    }

    pub fn set_always_on_top(&self, always_on_top: bool) {
        self.window.set_always_on_top(always_on_top)
    }

    pub fn set_window_icon(&self, window_icon: Option<window::Icon>) {
        self.window.set_window_icon(window_icon)
    }

    pub fn set_ime_position<P>(&self, position: P)
    where
        P: Into<window::Position>,
    {
        self.window.set_ime_position(position)
    }

    pub fn set_cursor_icon(&self, cursor: window::CursorIcon) {
        self.window.set_cursor_icon(cursor)
    }

    pub fn set_cursor_position<P>(&self, position: P) -> Result<(), CanvasWindowOperationError>
    where
        P: Into<window::Position>,
    {
        self.window.set_cursor_position(position)?;
        Ok(())
    }

    pub fn set_cursor_grab(&self, grab: bool) -> Result<(), CanvasWindowOperationError> {
        self.window.set_cursor_grab(grab)?;
        Ok(())
    }

    pub fn set_cursor_visible(&self, visible: bool) {
        self.window.set_cursor_visible(visible)
    }

    // pub fn resize_canvas_if_necessary(&mut self) -> Result<(), CanvasWindowOperationError> {
    //     let current_size = self.inner_size();
    //     if self.surface_extent.width != current_size.width
    //         || self.surface_extent.height != current_size.height
    //     {
    //         self.configure_swapchain()?;
    //     }
    //     Ok(())
    // }

    unsafe fn with_window(
        instance: &Instance,
        window: Window,
    ) -> Result<Self, CanvasWindowCreationError> {
        let surface = instance.create_surface(&window);
        Ok(Self { surface, window })
    }
}

pub struct CanvasWindowBuilder {
    builder: WindowBuilder,
}

impl CanvasWindowBuilder {
    pub fn new() -> Self {
        CanvasWindowBuilder {
            builder: window::WindowBuilder::new(),
        }
    }

    pub fn with_inner_size<S: Into<window::Size>>(self, size: S) -> Self {
        CanvasWindowBuilder {
            builder: self.builder.with_inner_size(size),
        }
    }

    pub fn with_min_inner_size<S: Into<window::Size>>(self, size: S) -> Self {
        CanvasWindowBuilder {
            builder: self.builder.with_min_inner_size(size),
        }
    }

    pub fn with_max_inner_size<S: Into<window::Size>>(self, size: S) -> Self {
        CanvasWindowBuilder {
            builder: self.builder.with_max_inner_size(size),
        }
    }

    pub fn with_title<T: Into<String>>(self, title: T) -> Self {
        CanvasWindowBuilder {
            builder: self.builder.with_title(title),
        }
    }

    pub fn with_window_icon(self, icon: Option<window::Icon>) -> Self {
        CanvasWindowBuilder {
            builder: self.builder.with_window_icon(icon),
        }
    }

    pub fn with_fullscreen(self, monitor: Option<window::Fullscreen>) -> Self {
        CanvasWindowBuilder {
            builder: self.builder.with_fullscreen(monitor),
        }
    }

    pub fn with_resizable(self, resizable: bool) -> Self {
        CanvasWindowBuilder {
            builder: self.builder.with_resizable(resizable),
        }
    }

    pub fn with_maximized(self, maximized: bool) -> Self {
        CanvasWindowBuilder {
            builder: self.builder.with_maximized(maximized),
        }
    }

    pub fn with_visible(self, visible: bool) -> Self {
        CanvasWindowBuilder {
            builder: self.builder.with_visible(visible),
        }
    }

    pub fn with_transparent(self, transparent: bool) -> Self {
        CanvasWindowBuilder {
            builder: self.builder.with_transparent(transparent),
        }
    }

    pub fn with_decorations(self, decorations: bool) -> Self {
        CanvasWindowBuilder {
            builder: self.builder.with_decorations(decorations),
        }
    }

    pub fn with_always_on_top(self, always_on_top: bool) -> Self {
        CanvasWindowBuilder {
            builder: self.builder.with_always_on_top(always_on_top),
        }
    }

    pub unsafe fn build<T>(
        self,
        instance: &Instance,
        window_target: &EventLoopWindowTarget<T>,
    ) -> Result<CanvasWindow, CanvasWindowCreationError>
    where
        T: 'static,
    {
        let window = self.builder.build(window_target)?;
        CanvasWindow::with_window(instance, window)
    }
}

// TODO PartialEq and Clone
#[derive(Debug)]
pub enum CanvasWindowCreationError {
    OsError(OsError),
    // SurfaceCreationFailed(hal::window::InitError),
    // SwapchainCreationFailed(hal::window::CreationError),
    // OutOfMemory(hal::device::OutOfMemory),
}

impl std::fmt::Display for CanvasWindowCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CanvasWindowCreationError::OsError(e) => write!(f, "OS Error ({})", e),
            // CanvasWindowCreationError::SurfaceCreationFailed(e) => {
            //     write!(f, "Surface creation failed ({})", e)
            // }
            // CanvasWindowCreationError::SwapchainCreationFailed(e) => {
            //     write!(f, "Swapchain configuration failed ({})", e)
            // }
            // CanvasWindowCreationError::OutOfMemory(e) => write!(f, "Out of memory ({})", e),
        }
    }
}

impl std::error::Error for CanvasWindowCreationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CanvasWindowCreationError::OsError(e) => Some(e),
            // CanvasWindowCreationError::SurfaceCreationFailed(e) => Some(e),
            // CanvasWindowCreationError::SwapchainCreationFailed(e) => Some(e),
            // CanvasWindowCreationError::OutOfMemory(e) => Some(e),
        }
    }
}

impl From<OsError> for CanvasWindowCreationError {
    fn from(e: OsError) -> Self {
        CanvasWindowCreationError::OsError(e)
    }
}

// impl From<hal::window::InitError> for CanvasWindowCreationError {
//     fn from(e: hal::window::InitError) -> Self {
//         CanvasWindowCreationError::SurfaceCreationFailed(e)
//     }
// }
//
// impl From<hal::window::CreationError> for CanvasWindowCreationError {
//     fn from(e: hal::window::CreationError) -> Self {
//         CanvasWindowCreationError::SwapchainCreationFailed(e)
//     }
// }
//
// impl From<hal::device::OutOfMemory> for CanvasWindowCreationError {
//     fn from(e: hal::device::OutOfMemory) -> Self {
//         CanvasWindowCreationError::OutOfMemory(e)
//     }
// }

#[derive(Debug)]
pub enum CanvasWindowOperationError {
    UnsupportedOperation(NotSupportedError),
    ExternalError(ExternalError),
    // SwapchainConfigurationFailed(hal::window::CreationError),
}

impl std::fmt::Display for CanvasWindowOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CanvasWindowOperationError::UnsupportedOperation(e) => {
                write!(f, "Unsupported operation ({})", e)
            }
            CanvasWindowOperationError::ExternalError(e) => write!(f, "External error ({})", e),
            // CanvasWindowOperationError::SwapchainConfigurationFailed(e) => {
            //     write!(f, "Swapchain configuration failed ({})", e)
            // }
        }
    }
}

impl std::error::Error for CanvasWindowOperationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CanvasWindowOperationError::UnsupportedOperation(e) => Some(e),
            CanvasWindowOperationError::ExternalError(e) => Some(e),
            // CanvasWindowOperationError::SwapchainConfigurationFailed(e) => Some(e),
        }
    }
}

impl From<NotSupportedError> for CanvasWindowOperationError {
    fn from(e: NotSupportedError) -> Self {
        CanvasWindowOperationError::UnsupportedOperation(e)
    }
}

impl From<ExternalError> for CanvasWindowOperationError {
    fn from(e: ExternalError) -> Self {
        CanvasWindowOperationError::ExternalError(e)
    }
}

// impl From<hal::window::CreationError> for CanvasWindowOperationError {
//     fn from(e: hal::window::CreationError) -> Self {
//         CanvasWindowOperationError::SwapchainConfigurationFailed(e)
//     }
// }
