use std::default::Default;

use rae_app::{
    event::EventLoop,
    window,
    window::{ExternalError, NotSupportedError, OsError, Window, WindowId},
};

use super::{
    Canvas, CanvasBuffer, CanvasFrame, CanvasSwapChainFrame, Extent3d, Instance, PresentMode,
    Surface, SwapChain, SwapChainDescriptor, SwapChainError, Texture, TextureDescriptor,
    TextureDimension, TextureFormat, TextureUsage, TextureView, TextureViewDescriptor,
};

// TODO: actually add the depth buffer to the canvas frame.
// TODO: resize depth buffer when resizing.
// TODO: specify texture formats in the descriptor.
// TODO: create specific enums for color formats and ds formats.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct CanvasWindowDescriptor {
    pub sample_count: u32,
}

impl Default for CanvasWindowDescriptor {
    fn default() -> Self {
        Self { sample_count: 1 }
    }
}

#[derive(Debug)]
pub struct CanvasWindow {
    surface_size: window::PhysicalSize<u32>,
    sample_count: u32,
    depth_buffer_format: TextureFormat,
    depth_buffer: TextureView,
    color_buffer_format: TextureFormat,
    backbuffer: Option<TextureView>,
    swap_chain: SwapChain,
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
        let depth_buffer_format = instance.depth_format();
        let color_buffer_format = instance.color_format();
        let depth_buffer = Self::create_depth_buffer(
            instance,
            &surface_size,
            depth_buffer_format,
            desc.sample_count,
        );
        let (swap_chain, backbuffer) = Self::create_swap_chain(
            instance,
            &surface,
            &surface_size,
            color_buffer_format,
            desc.sample_count,
        );
        Self {
            surface_size,
            sample_count: desc.sample_count,
            depth_buffer_format,
            depth_buffer,
            color_buffer_format,
            backbuffer,
            swap_chain,
            surface,
            window,
        }
    }

    pub fn update_buffers(&mut self, instance: &Instance) {
        let current_size = self.inner_size();
        if self.surface_size != current_size {
            let depth_buffer = Self::create_depth_buffer(
                instance,
                &current_size,
                self.depth_buffer_format,
                self.sample_count,
            );
            let (swap_chain, backbuffer) = Self::create_swap_chain(
                instance,
                &self.surface,
                &current_size,
                self.color_buffer_format,
                self.sample_count,
            );
            self.depth_buffer = depth_buffer;
            self.swap_chain = swap_chain;
            self.backbuffer = backbuffer;
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

    fn create_depth_buffer(
        instance: &Instance,
        size: &window::PhysicalSize<u32>,
        format: TextureFormat,
        sample_count: u32,
    ) -> TextureView {
        let texture = Texture::new(
            instance,
            &TextureDescriptor {
                size: Extent3d {
                    width: size.width,
                    height: size.height,
                    depth: 1,
                },
                mip_level_count: 1,
                sample_count,
                dimension: TextureDimension::D2,
                format,
                usage: TextureUsage::OUTPUT_ATTACHMENT,
                label: None,
            },
        );
        texture.create_view(&TextureViewDescriptor::default())
    }

    fn create_swap_chain(
        instance: &Instance,
        surface: &Surface,
        size: &window::PhysicalSize<u32>,
        format: TextureFormat,
        sample_count: u32,
    ) -> (SwapChain, Option<TextureView>) {
        let swap_chain = SwapChain::new(
            instance,
            surface,
            &SwapChainDescriptor {
                usage: TextureUsage::OUTPUT_ATTACHMENT,
                format,
                width: size.width,
                height: size.height,
                present_mode: PresentMode::Mailbox,
            },
        );
        let backbuffer = if sample_count > 1 {
            let backbuffer_texture = Texture::new(
                instance,
                &TextureDescriptor {
                    size: Extent3d {
                        width: size.width,
                        height: size.height,
                        depth: 1,
                    },
                    mip_level_count: 1,
                    sample_count,
                    dimension: TextureDimension::D2,
                    format,
                    usage: TextureUsage::OUTPUT_ATTACHMENT,
                    label: None,
                },
            );
            Some(backbuffer_texture.create_view(&TextureViewDescriptor::default()))
        } else {
            None
        };
        (swap_chain, backbuffer)
    }
}

impl Canvas for CanvasWindow {
    fn current_frame(&mut self) -> Result<CanvasFrame, SwapChainError> {
        let swap_chain_frame = self.swap_chain.get_current_frame()?;
        let backbuffer = match self.backbuffer {
            Some(ref v) => Some(v),
            None => None,
        };
        Ok(CanvasFrame {
            swap_chain_frame: Some(CanvasSwapChainFrame {
                frame: swap_chain_frame,
                backbuffer,
                format: self.color_buffer_format,
                sample_count: self.sample_count,
            }),
            color_buffers: Vec::new(),
            depth_stencil_buffer: Some(CanvasBuffer {
                buffer: &self.depth_buffer,
                format: self.depth_buffer_format,
                sample_count: self.sample_count,
            }),
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
