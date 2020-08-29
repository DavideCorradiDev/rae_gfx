use rand::Rng;

use rae_app::{
    application::Application,
    event::{mouse, ControlFlow, DeviceId, EventHandler, EventLoop},
    window,
    window::{WindowBuilder, WindowId},
};

use rae_math::{
    conversion::convert,
    geometry2::{OrthographicProjection, Point, Projective, Similarity, Translation, UnitComplex},
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
    saved_triangle_constants: Vec<geometry2::PushConstants>,
    projection_transform: Projective<f32>,
    current_position: Point<f32>,
    current_angle: f32,
    current_scaling: f32,
    current_color: Color,
}

impl ApplicationImpl {
    pub fn generate_push_constant(&self) -> geometry2::PushConstants {
        let object_transform = Similarity::<f32>::from_parts(
            Translation::new(self.current_position.x, self.current_position.y),
            UnitComplex::new(self.current_angle),
            self.current_scaling,
        );
        geometry2::PushConstants::new(
            &convert(self.projection_transform * object_transform),
            self.current_color,
        )
    }
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
                geometry2::Vertex::new([-50., 50.]),
                geometry2::Vertex::new([50., 50.]),
                geometry2::Vertex::new([0., -50.]),
            ],
            &[0, 1, 2],
        );

        let window_size = window.inner_size();

        // This matrix will flip the y axis, so that screen coordinates follow mouse coordinates.
        let projection_transform = OrthographicProjection::new(
            0.,
            window_size.width as f32,
            window_size.height as f32,
            0.,
        )
        .to_projective();

        let current_position = Point::from([
            window_size.width as f32 / 2.,
            window_size.height as f32 / 2.,
        ]);

        Ok(Self {
            window,
            instance,
            pipeline,
            triangle_mesh,
            saved_triangle_constants: Vec::new(),
            projection_transform,
            current_position,
            current_angle: 0.,
            current_scaling: 1.,
            current_color: Color {
                r: 1.,
                g: 1.,
                b: 1.,
                a: 0.75,
            },
        })
    }

    fn on_resized(
        &mut self,
        wid: WindowId,
        size: window::PhysicalSize<u32>,
    ) -> Result<ControlFlow, Self::Error> {
        if wid == self.window.id() {
            self.window.reconfigure_swap_chain(&self.instance);
            self.projection_transform = OrthographicProjection::new(
                0.,
                1f32.max(size.width as f32),
                1f32.max(size.height as f32),
                0.,
            )
            .to_projective();
        }
        Ok(ControlFlow::Continue)
    }

    fn on_cursor_moved(
        &mut self,
        wid: WindowId,
        _device_id: DeviceId,
        position: window::PhysicalPosition<f64>,
    ) -> Result<ControlFlow, Self::Error> {
        if wid == self.window.id() {
            self.current_position.x = position.x as f32;
            self.current_position.y = position.y as f32;
        }
        Ok(ControlFlow::Continue)
    }

    fn on_mouse_button_released(
        &mut self,
        wid: WindowId,
        _device_id: DeviceId,
        button: mouse::Button,
    ) -> Result<ControlFlow, Self::Error> {
        if wid == self.window.id() {
            if button == mouse::Button::Left {
                self.saved_triangle_constants
                    .push(self.generate_push_constant());
                let mut rng = rand::thread_rng();
                self.current_scaling = rng.gen_range(0.25, 4.);
                self.current_color.r = rng.gen_range(0., 1.);
                self.current_color.g = rng.gen_range(0., 1.);
                self.current_color.b = rng.gen_range(0., 1.);
            }
        }
        Ok(ControlFlow::Continue)
    }

    fn on_variable_update(&mut self, dt: std::time::Duration) -> Result<ControlFlow, Self::Error> {
        const ANGULAR_SPEED: f32 = std::f32::consts::PI * 0.25;
        self.current_angle = self.current_angle + ANGULAR_SPEED * dt.as_secs_f32();
        while self.current_angle >= std::f32::consts::PI * 2. {
            self.current_angle = self.current_angle - std::f32::consts::PI * 2.;
        }

        let mut elements = Vec::new();
        for triangle_constant in &self.saved_triangle_constants {
            elements.push((&self.triangle_mesh, triangle_constant));
        }

        let current_triangle_constant = self.generate_push_constant();
        elements.push((&self.triangle_mesh, &current_triangle_constant));

        // TODO: missing error handling, remove the unwrap call...
        // It seems that frame should be retrieved and be alive the whole time, or bad stuff will happen...
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
                        load: LoadOp::Clear(Color::BLACK),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            rpass.draw_geometry2_array(&self.pipeline, &elements);
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
