use rae_app::{
    event::{EventLoop, EventLoopWindowTarget},
    window,
    window::{ExternalError, NotSupportedError, OsError, Window, WindowBuilder, WindowId},
};

use super::{Instance, PresentMode, Surface, SwapChain, SwapChainDescriptor, TextureUsage};

#[derive(Debug)]
pub struct CanvasWindow {
    swap_chain: SwapChain,
    surface: Surface,
    surface_size: window::PhysicalSize<u32>,
    window: Window,
}

impl CanvasWindow {
    pub unsafe fn new<T: 'static>(
        instance: &Instance,
        event_loop: &EventLoop<T>,
    ) -> Result<Self, OsError> {
        let window = Window::new(event_loop)?;
        Ok(Self::with_window(instance, window))
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

    pub fn inner_position(&self) -> Result<window::PhysicalPosition<i32>, NotSupportedError> {
        self.window.inner_position()
    }

    pub fn outer_position(&self) -> Result<window::PhysicalPosition<i32>, NotSupportedError> {
        self.window.outer_position()
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

    pub fn set_inner_size<S>(&mut self, instance: &Instance, size: S)
    where
        S: Into<window::Size>,
    {
        self.window.set_inner_size(size);
        self.reconfigure_swap_chain(instance);
    }

    pub fn set_min_inner_size<S>(&mut self, instance: &Instance, min_size: Option<S>)
    where
        S: Into<window::Size>,
    {
        self.window.set_min_inner_size(min_size);
        self.reconfigure_swap_chain(instance);
    }

    pub fn set_max_inner_size<S>(&mut self, instance: &Instance, max_size: Option<S>)
    where
        S: Into<window::Size>,
    {
        self.window.set_max_inner_size(max_size);
        self.reconfigure_swap_chain(instance);
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

    pub fn set_cursor_position<P>(&self, position: P) -> Result<(), ExternalError>
    where
        P: Into<window::Position>,
    {
        self.window.set_cursor_position(position)
    }

    pub fn set_cursor_grab(&self, grab: bool) -> Result<(), ExternalError> {
        self.window.set_cursor_grab(grab)
    }

    pub fn set_cursor_visible(&self, visible: bool) {
        self.window.set_cursor_visible(visible)
    }

    pub fn reconfigure_swap_chain(&mut self, instance: &Instance) {
        let current_size = self.inner_size();
        if self.surface_size != current_size {
            self.swap_chain = Self::create_swap_chain(instance, &self.surface, &current_size);
            self.surface_size = current_size;
        }
    }

    unsafe fn with_window(instance: &Instance, window: Window) -> Self {
        let surface = instance.create_surface(&window);
        let surface_size = window.inner_size();
        let swap_chain = Self::create_swap_chain(instance, &surface, &surface_size);
        Self {
            swap_chain,
            surface,
            surface_size,
            window,
        }
    }

    fn create_swap_chain(
        instance: &Instance,
        surface: &Surface,
        size: &window::PhysicalSize<u32>,
    ) -> SwapChain {
        instance.create_swap_chain(
            surface,
            &SwapChainDescriptor {
                usage: TextureUsage::OUTPUT_ATTACHMENT,
                format: instance.color_format(),
                width: size.width,
                height: size.height,
                present_mode: PresentMode::Mailbox,
            },
        )
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
    ) -> Result<CanvasWindow, OsError>
    where
        T: 'static,
    {
        let window = self.builder.build(window_target)?;
        Ok(CanvasWindow::with_window(instance, window))
    }
}