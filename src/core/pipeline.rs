extern crate gfx_hal as hal;

use crate::halw;

use std::{cell::RefCell, rc::Rc};

use super::{Canvas, Instance};

pub trait PipelineConfig {
    type Vertex;
    type Constants;
    type Uniforms;
    fn vertex_shader_module(&self) -> &[u32];
    fn fragment_shader_module(&self) -> &[u32];
}

#[derive(Debug, PartialEq, Clone)]
pub enum PipelineCreationError {
    OutOfMemory(hal::device::OutOfMemory),
    PipelineCreationFailed(hal::pso::CreationError),
}

impl std::fmt::Display for PipelineCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineCreationError::OutOfMemory(e) => write!(f, "Out of memory ({})", e),
            PipelineCreationError::PipelineCreationFailed(e) => {
                write!(f, "Pipeline creation failed ({})", e)
            }
        }
    }
}

impl std::error::Error for PipelineCreationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PipelineCreationError::OutOfMemory(e) => Some(e),
            PipelineCreationError::PipelineCreationFailed(e) => Some(e),
        }
    }
}

impl From<hal::device::OutOfMemory> for PipelineCreationError {
    fn from(e: hal::device::OutOfMemory) -> Self {
        PipelineCreationError::OutOfMemory(e)
    }
}
impl From<hal::pso::CreationError> for PipelineCreationError {
    fn from(e: hal::pso::CreationError) -> Self {
        PipelineCreationError::PipelineCreationFailed(e)
    }
}

pub struct Pipeline<Config: PipelineConfig> {
    canvas: Rc<RefCell<dyn Canvas>>,
    layout: halw::PipelineLayout,
    // pipeline: halw::GraphicsPipeline,
    _p: std::marker::PhantomData<Config>,
}

impl<Config> Pipeline<Config>
where
    Config: PipelineConfig,
{
    fn create(
        instance: &Instance,
        canvas: Rc<RefCell<dyn Canvas>>,
    ) -> Result<Self, PipelineCreationError> {
        let layout = Self::create_layout(Rc::clone(&instance.gpu_rc()))?;
        // let pipeline = Self::create_pipeline(Rc::clone(&instance.gpu_rc()),
        // canvas.render_pass(),
        // &layout)?;
        Ok(Self {
            canvas,
            layout,
            // pipeline,
            _p: std::marker::PhantomData,
        })
    }

    fn create_layout(
        gpu: Rc<RefCell<halw::Gpu>>,
    ) -> Result<halw::PipelineLayout, hal::device::OutOfMemory> {
        let pipeline = halw::PipelineLayout::create(gpu, &[], &[])?;
        Ok(pipeline)
    }

    // fn create_pipeline(
    //     gpu: Rc<RefCell<halw::Gpu>>
    //     render_pass: &halw::RenderPass,
    //     layout: &halw::PipelineLayout
    // ) -> Result<halw::GraphicsPipeline, hal::pso::CreationError>{

    // }
}
