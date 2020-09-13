use std::default::Default;

use rae_app::{
    event::EventLoop,
    window,
    window::{ExternalError, NotSupportedError, OsError, Window, WindowId},
};

use super::{
    Canvas, CanvasDepthStencilBuffer, CanvasDepthStencilBufferDescriptor, CanvasFrame,
    CanvasSwapChain, CanvasSwapChainDescriptor, ColorBufferFormat, DepthStencilBufferFormat,
    Instance, SampleCount, Size, Surface, SwapChainError,
};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct CanvasWindowDescriptor {
    pub sample_count: SampleCount,
    pub color_buffer_format: ColorBufferFormat,
    pub depth_stencil_buffer_format: Option<DepthStencilBufferFormat>,
}

impl Default for CanvasWindowDescriptor {
    fn default() -> Self {
        Self {
            sample_count: 1,
            color_buffer_format: ColorBufferFormat::default(),
            depth_stencil_buffer_format: Some(DepthStencilBufferFormat::Depth32Float),
        }
    }
}

#[derive(Debug)]
pub struct CanvasWindow {
    surface_size: window::PhysicalSize<u32>,
    sample_count: SampleCount,
    depth_stencil_buffer: Option<CanvasDepthStencilBuffer>,
    swap_chain: CanvasSwapChain,
    surface: Surface,
    window: Window,
}

impl CanvasWindow {
    // Unsafe: surface creation.
    pub unsafe fn new<T: 'static>(
        instance: &Instance,
        event_loop: &EventLoop<T>,
        desc: &CanvasWindowDescriptor,
    ) -> Result<Self, OsError> {
        let window = Window::new(event_loop)?;
        Ok(Self::from_window(instance, window, desc))
    }

    // Unsafe: surface creation.
    pub unsafe fn from_window(
        instance: &Instance,
        window: Window,
        desc: &CanvasWindowDescriptor,
    ) -> Self {
        let surface = Surface::new(&instance, &window);
        Self::from_window_and_surface(instance, window, surface, desc)
    }

    // Unsafe: surface must correspond to the window.
    pub unsafe fn from_window_and_surface(
        instance: &Instance,
        window: Window,
        surface: Surface,
        desc: &CanvasWindowDescriptor,
    ) -> Self {
        let surface_size = window.inner_size();
        let (swap_chain, depth_stencil_buffer) = Self::create_buffers(
            instance,
            &surface,
            &surface_size,
            desc.color_buffer_format,
            desc.depth_stencil_buffer_format,
            desc.sample_count,
        );
        Self {
            surface_size,
            sample_count: desc.sample_count,
            depth_stencil_buffer,
            swap_chain,
            surface,
            window,
        }
    }

    pub fn sample_count(&self) -> SampleCount {
        self.sample_count
    }

    pub fn color_buffer_format(&self) -> ColorBufferFormat {
        self.swap_chain.format()
    }

    pub fn depth_stencil_buffer_format(&self) -> Option<DepthStencilBufferFormat> {
        match &self.depth_stencil_buffer {
            Some(v) => Some(v.format()),
            None => None,
        }
    }

    pub fn update_buffers(&mut self, instance: &Instance) {
        let current_size = self.inner_size();
        if self.surface_size != current_size {
            let (swap_chain, depth_stencil_buffer) = Self::create_buffers(
                instance,
                &self.surface,
                &current_size,
                self.color_buffer_format(),
                self.depth_stencil_buffer_format(),
                self.sample_count,
            );
            self.swap_chain = swap_chain;
            self.depth_stencil_buffer = depth_stencil_buffer;
            self.surface_size = current_size;
        }
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
        self.update_buffers(instance);
    }

    pub fn set_min_inner_size<S>(&mut self, instance: &Instance, min_size: Option<S>)
    where
        S: Into<window::Size>,
    {
        self.window.set_min_inner_size(min_size);
        self.update_buffers(instance);
    }

    pub fn set_max_inner_size<S>(&mut self, instance: &Instance, max_size: Option<S>)
    where
        S: Into<window::Size>,
    {
        self.window.set_max_inner_size(max_size);
        self.update_buffers(instance);
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

    fn create_buffers(
        instance: &Instance,
        surface: &Surface,
        size: &window::PhysicalSize<u32>,
        color_format: ColorBufferFormat,
        depth_stencil_format: Option<DepthStencilBufferFormat>,
        sample_count: SampleCount,
    ) -> (CanvasSwapChain, Option<CanvasDepthStencilBuffer>) {
        let swap_chain = CanvasSwapChain::new(
            instance,
            surface,
            &CanvasSwapChainDescriptor {
                size: Size::new(size.width, size.height),
                format: color_format,
                sample_count,
            },
        );
        let depth_stencil_buffer = match depth_stencil_format {
            Some(format) => Some(CanvasDepthStencilBuffer::new(
                instance,
                &CanvasDepthStencilBufferDescriptor {
                    size: Size::new(size.width, size.height),
                    format,
                    sample_count,
                },
            )),
            None => None,
        };
        (swap_chain, depth_stencil_buffer)
    }
}

impl Canvas for CanvasWindow {
    fn current_frame(&mut self) -> Result<CanvasFrame, SwapChainError> {
        let swap_chain = Some(self.swap_chain.reference()?);
        let depth_stencil_buffer = match &self.depth_stencil_buffer {
            Some(depth_stencil_buffer) => Some(depth_stencil_buffer.reference()),
            None => None,
        };
        Ok(CanvasFrame {
            swap_chain,
            color_buffers: Vec::new(),
            depth_stencil_buffer,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use galvanic_assert::{matchers::*, *};

    use rae_app::{event::EventLoopAnyThread, window::WindowBuilder};

    use crate::core::InstanceDescriptor;

    #[test]
    fn from_window() {
        let instance = Instance::new(&InstanceDescriptor::default()).unwrap();
        let event_loop = EventLoop::<()>::new_any_thread();
        let window = WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)
            .unwrap();
        let _canvas_window = unsafe {
            CanvasWindow::from_window(&instance, window, &CanvasWindowDescriptor::default())
        };
    }

    #[test]
    fn from_window_and_surface() {
        let event_loop = EventLoop::<()>::new_any_thread();
        let window = WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)
            .unwrap();
        let (instance, surface) = unsafe {
            Instance::new_with_compatible_window(&InstanceDescriptor::default(), &window).unwrap()
        };
        let _canvas_window = unsafe {
            CanvasWindow::from_window_and_surface(
                &instance,
                window,
                surface,
                &CanvasWindowDescriptor::default(),
            )
        };
    }

    #[test]
    fn multiple_windows_with_generic_instance() {
        let instance = Instance::new(&InstanceDescriptor::default()).unwrap();
        let event_loop = EventLoop::<()>::new_any_thread();
        let window1 = unsafe {
            CanvasWindow::from_window(
                &instance,
                WindowBuilder::new()
                    .with_visible(false)
                    .build(&event_loop)
                    .unwrap(),
                &CanvasWindowDescriptor::default(),
            )
        };
        let window2 = unsafe {
            CanvasWindow::from_window(
                &instance,
                WindowBuilder::new()
                    .with_visible(false)
                    .build(&event_loop)
                    .unwrap(),
                &CanvasWindowDescriptor::default(),
            )
        };
        expect_that!(&window1.id(), not(eq(window2.id())));
    }

    #[test]
    fn multiple_windows_with_compatible_instance() {
        let event_loop = EventLoop::<()>::new_any_thread();
        let window1 = WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)
            .unwrap();
        let (instance, surface) = unsafe {
            Instance::new_with_compatible_window(&InstanceDescriptor::default(), &window1).unwrap()
        };
        let window1 = unsafe {
            CanvasWindow::from_window_and_surface(
                &instance,
                window1,
                surface,
                &CanvasWindowDescriptor::default(),
            )
        };
        let window2 = unsafe {
            CanvasWindow::from_window(
                &instance,
                WindowBuilder::new()
                    .with_visible(false)
                    .build(&event_loop)
                    .unwrap(),
                &CanvasWindowDescriptor::default(),
            )
        };
        expect_that!(&window1.id(), not(eq(window2.id())));
    }
}
