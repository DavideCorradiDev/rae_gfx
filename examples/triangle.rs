extern crate rae_app;

use std::{cell::RefCell, rc::Rc};

use rae_app::{
    application::Application,
    event::{ControlFlow, EventHandler, EventLoop},
    window,
    window::WindowId,
};

use rae_gfx::core::{
    geometry2d_pipeline,
    pipeline::{PipelineCreationError, RenderingError},
    BeginFrameError, BufferCreationError, Canvas, CanvasWindow, CanvasWindowBuilder,
    CanvasWindowCreationError, CanvasWindowOperationError, EndFrameError, Instance,
    InstanceCreationError,
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
    pipeline: geometry2d_pipeline::Pipeline<CanvasWindow>,
    triangle: geometry2d_pipeline::VertexArray,
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
                    height: 600,
                }))
                .build(&instance, event_loop)?,
        ));
        let pipeline = geometry2d_pipeline::Pipeline::create(&instance, Rc::clone(&window))?;
        let triangle = geometry2d_pipeline::VertexArray::from_vertices(
            &instance,
            &[
                geometry2d_pipeline::Vertex { pos: [-0.5, 0.5] },
                geometry2d_pipeline::Vertex { pos: [0., -0.5] },
                geometry2d_pipeline::Vertex { pos: [0.5, 0.5] },
            ],
        )?;
        Ok(Self {
            instance,
            window,
            pipeline,
            triangle,
        })
    }

    fn on_resized(
        &mut self,
        wid: WindowId,
        _size: window::PhysicalSize<u32>,
    ) -> Result<ControlFlow, Self::Error> {
        if wid == self.window.borrow().id() {
            self.window.borrow_mut().resize_canvas_if_necessary()?;
        }
        Ok(ControlFlow::Continue)
    }

    fn on_variable_update(&mut self, _: std::time::Duration) -> Result<ControlFlow, Self::Error> {
        let push_constant = geometry2d_pipeline::PushConstant {
            color: [1., 1., 1., 1.],
        };
        self.window.borrow_mut().begin_frame()?;
        self.pipeline.render(&[(&push_constant, &self.triangle)])?;
        self.window.borrow_mut().end_frame()?;
        Ok(ControlFlow::Continue)
    }
}

fn main() {
    const FIXED_FRAMERATE: u64 = 30;
    const VARIABLE_FRAMERATE_CAP: u64 = 60;
    Application::<ApplicationImpl, _, _>::new(FIXED_FRAMERATE, Some(VARIABLE_FRAMERATE_CAP)).run();
}
