use std::{cell::RefCell, rc::Rc};

use rand::Rng;

use rae_app::{
    application::Application,
    event::{mouse, ControlFlow, DeviceId, EventHandler, EventLoop},
    window,
    window::WindowId,
};

use rae_math::{
    conversion::convert,
    geometry2::{OrthographicProjection, Point, Projective, Similarity, Translation, UnitComplex},
};

use rae_gfx::{
    canvas::{
        BeginFrameError, Canvas, CanvasWindow, CanvasWindowBuilder, CanvasWindowCreationError,
        CanvasWindowOperationError, EndFrameError,
    },
    core::{Instance, InstanceCreationError},
    geometry2_rendering,
    rendering::{BufferCreationError, PipelineCreationError, RenderingError},
};

type ApplicationEvent = ();

#[derive(Debug)]
enum ApplicationError {
    InstanceCreationFailed(InstanceCreationError),
    WindowCreationFailed(CanvasWindowCreationError),
    WindowOperationFailed(CanvasWindowOperationError),
    PipelineCreationFailed(PipelineCreationError),
    BufferCreationFailed(BufferCreationError),
    BeginFrameFailed(BeginFrameError),
    EndFrameFailed(EndFrameError),
    RenderingFailed(RenderingError),
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
            ApplicationError::WindowOperationFailed(e) => {
                write!(f, "Render frame start failed ({})", e)
            }
            ApplicationError::PipelineCreationFailed(e) => {
                write!(f, "Pipeline creation failed ({})", e)
            }
            ApplicationError::BufferCreationFailed(e) => {
                write!(f, "Buffer creation failed ({})", e)
            }
            ApplicationError::BeginFrameFailed(e) => write!(f, "Render frame start failed ({})", e),
            ApplicationError::EndFrameFailed(e) => write!(f, "Render frame end failed ({})", e),
            ApplicationError::RenderingFailed(e) => write!(f, "Rendering failed ({})", e),
        }
    }
}

impl std::error::Error for ApplicationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ApplicationError::InstanceCreationFailed(e) => Some(e),
            ApplicationError::WindowCreationFailed(e) => Some(e),
            ApplicationError::WindowOperationFailed(e) => Some(e),
            ApplicationError::PipelineCreationFailed(e) => Some(e),
            ApplicationError::BufferCreationFailed(e) => Some(e),
            ApplicationError::BeginFrameFailed(e) => Some(e),
            ApplicationError::EndFrameFailed(e) => Some(e),
            ApplicationError::RenderingFailed(e) => Some(e),
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

impl From<CanvasWindowOperationError> for ApplicationError {
    fn from(e: CanvasWindowOperationError) -> Self {
        ApplicationError::WindowOperationFailed(e)
    }
}

impl From<PipelineCreationError> for ApplicationError {
    fn from(e: PipelineCreationError) -> Self {
        ApplicationError::PipelineCreationFailed(e)
    }
}

impl From<BufferCreationError> for ApplicationError {
    fn from(e: BufferCreationError) -> Self {
        ApplicationError::BufferCreationFailed(e)
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

impl From<RenderingError> for ApplicationError {
    fn from(e: RenderingError) -> Self {
        ApplicationError::RenderingFailed(e)
    }
}

#[derive(Debug)]
struct ApplicationImpl {
    instance: Instance,
    window: Rc<RefCell<CanvasWindow>>,
    pipeline: geometry2_rendering::Pipeline<CanvasWindow>,
    triangle: geometry2_rendering::VertexArray,
    projection_transform: Projective<f32>,
    current_position: Point<f32>,
    current_angle: f32,
    current_scaling: f32,
    current_color: [f32; 4],
    saved_triangle_constants: Vec<geometry2_rendering::PushConstant>,
}

impl ApplicationImpl {
    pub fn generate_push_constant(&self) -> geometry2_rendering::PushConstant {
        let object_transform = Similarity::<f32>::from_parts(
            Translation::new(self.current_position.x, self.current_position.y),
            UnitComplex::new(self.current_angle),
            self.current_scaling,
        );
        geometry2_rendering::PushConstant::new(
            convert(self.projection_transform * object_transform),
            self.current_color,
        )
    }
}

impl EventHandler<ApplicationError, ApplicationEvent> for ApplicationImpl {
    type Error = ApplicationError;
    type CustomEvent = ApplicationEvent;

    fn new(event_loop: &EventLoop<Self::CustomEvent>) -> Result<Self, Self::Error> {
        let instance = Instance::create()?;
        let window = Rc::new(RefCell::new(
            CanvasWindowBuilder::new()
                .with_title("Triangle Example")
                .with_inner_size(window::Size::Physical(window::PhysicalSize {
                    width: 800,
                    height: 800,
                }))
                .build(&instance, event_loop)?,
        ));
        window.borrow().set_cursor_visible(false);
        let pipeline = geometry2_rendering::Pipeline::create(&instance, Rc::clone(&window))?;
        let triangle = geometry2_rendering::VertexArray::from_vertices(
            &instance,
            &[
                geometry2_rendering::Vertex::new(-50., 50.),
                geometry2_rendering::Vertex::new(0.0, -50.),
                geometry2_rendering::Vertex::new(50., 50.),
            ],
        )?;
        let window_size = window.borrow().inner_size();
        let projection_transform = OrthographicProjection::new(
            0.,
            window_size.width as f32,
            0.,
            window_size.height as f32,
        )
        .to_projective();
        let current_position = Point::new(
            window_size.width as f32 / 2.,
            window_size.height as f32 / 2.,
        );
        Ok(Self {
            instance,
            window,
            pipeline,
            triangle,
            projection_transform,
            current_position,
            current_angle: 0.,
            current_scaling: 1.,
            current_color: [1., 1., 1., 0.75],
            saved_triangle_constants: Vec::new(),
        })
    }

    fn on_resized(
        &mut self,
        wid: WindowId,
        size: window::PhysicalSize<u32>,
    ) -> Result<ControlFlow, Self::Error> {
        if wid == self.window.borrow().id() {
            self.window.borrow_mut().resize_canvas_if_necessary()?;
            self.projection_transform =
                OrthographicProjection::new(0., size.width as f32, 0., size.height as f32)
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
        if wid == self.window.borrow().id() {
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
        if wid == self.window.borrow().id() {
            if button == mouse::Button::Left {
                self.saved_triangle_constants
                    .push(self.generate_push_constant());
                let mut rng = rand::thread_rng();
                self.current_scaling = rng.gen_range(0.25, 4.);
                self.current_color[0] = rng.gen_range(0., 1.);
                self.current_color[1] = rng.gen_range(0., 1.);
                self.current_color[2] = rng.gen_range(0., 1.);
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
            elements.push((triangle_constant, &self.triangle));
        }
        let current_triangle_constant = self.generate_push_constant();
        elements.push((&current_triangle_constant, &self.triangle));

        self.window.borrow_mut().begin_frame()?;
        self.pipeline.render(elements.as_slice())?;
        self.window.borrow_mut().end_frame()?;
        Ok(ControlFlow::Continue)
    }
}

fn main() {
    const FIXED_FRAMERATE: u64 = 30;
    const VARIABLE_FRAMERATE_CAP: u64 = 60;
    Application::<ApplicationImpl, _, _>::new(FIXED_FRAMERATE, Some(VARIABLE_FRAMERATE_CAP)).run();
}
