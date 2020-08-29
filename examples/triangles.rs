use rae_app::{
    application::Application,
    event::{mouse, ControlFlow, DeviceId, EventHandler, EventLoop},
    window,
    window::{WindowBuilder, WindowId},
};

use rae_gfx::wgpu::{
    core::{
        Canvas, CanvasWindow, Color, CommandEncoderDescriptor, Instance, InstanceConfig,
        InstanceCreationError, LoadOp, Operations, RenderPassColorAttachmentDescriptor,
        RenderPassDescriptor,
    },
    geometry2,
    geometry2::Renderer as Geometry2Renderer,
};

#[derive(Debug)]
struct ApplicationImpl {
    window: CanvasWindow,
    instance: Instance,
    pipeline: geometry2::RenderPipeline,
    triangle_mesh: geometry2::Mesh,
}

impl EventHandler<ApplicationError, ApplicationEvent> for ApplicationImpl {
    type Error = ApplicationError;
    type CustomEvent = ApplicationEvent;

    fn new(event_loop: &EventLoop<Self::CustomEvent>) -> Result<Self, Self::Error> {
        let window = WindowBuilder::new()
            .with_inner_size(window::Size::Physical(window::PhysicalSize {
                width: 800,
                height: 800,
            }))
            .build(event_loop)?;
        let (window, instance) = unsafe {
            let (instance, surface) =
                Instance::new_with_compatible_window(&InstanceConfig::high_performance(), &window)?;
            let window = CanvasWindow::from_window_and_surface(&instance, window, surface);
            (window, instance)
        };

        let pipeline =
            geometry2::RenderPipeline::new(&instance, &geometry2::RenderPipelineConfig::default());

        let triangle_mesh = geometry2::Mesh::new(
            &instance,
            &[
                geometry2::Vertex::new(-0.5, -0.5),
                geometry2::Vertex::new(0.5, -0.5),
                geometry2::Vertex::new(0., 0.5),
            ],
            &[0, 1, 2],
        );

        Ok(Self {
            window,
            instance,
            pipeline,
            triangle_mesh,
        })
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
        // TODO: error handling
        let frame = self.window.get_current_frame().unwrap();
        let mut encoder = self
            .instance
            .create_command_encoder(&CommandEncoderDescriptor::default());
        {
            // TODO: simplify this a little bit by implementing a trait for encoder taking a Canvas?
            let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                color_attachments: &[RenderPassColorAttachmentDescriptor {
                    attachment: &frame.output.view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            rpass.draw_geometry2(&self.pipeline, &self.triangle_mesh);
        }
        self.instance.submit(Some(encoder.finish()));
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

fn main() {
    const FIXED_FRAMERATE: u64 = 30;
    const VARIABLE_FRAMERATE_CAP: u64 = 60;
    Application::<ApplicationImpl, _, _>::new(FIXED_FRAMERATE, Some(VARIABLE_FRAMERATE_CAP)).run();
}
