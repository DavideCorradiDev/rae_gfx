extern crate gfx_hal as hal;

use crate::halw;

use std::{cell::RefCell, rc::Rc};

use super::{Canvas, Instance};

pub struct ShaderConfig {
    source: Vec<u32>,
    push_constant_range: Option<std::ops::Range<u32>>,
}

pub trait PipelineConfig {
    type Vertex;
    type Constants;
    type Uniforms;
    fn vertex_shader_config() -> &'static ShaderConfig;
    fn fragment_shader_config() -> &'static ShaderConfig;
}

#[derive(Debug, PartialEq, Clone)]
pub enum PipelineCreationError {
    OutOfMemory(hal::device::OutOfMemory),
    ShaderCreationFailed(hal::device::ShaderError),
    PipelineCreationFailed(hal::pso::CreationError),
}

impl std::fmt::Display for PipelineCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineCreationError::OutOfMemory(e) => write!(f, "Out of memory ({})", e),
            PipelineCreationError::ShaderCreationFailed(e) => {
                write!(f, "Shader creation failed({})", e)
            }
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
            PipelineCreationError::ShaderCreationFailed(e) => Some(e),
            PipelineCreationError::PipelineCreationFailed(e) => Some(e),
        }
    }
}

impl From<hal::device::OutOfMemory> for PipelineCreationError {
    fn from(e: hal::device::OutOfMemory) -> Self {
        PipelineCreationError::OutOfMemory(e)
    }
}

impl From<hal::device::ShaderError> for PipelineCreationError {
    fn from(e: hal::device::ShaderError) -> Self {
        PipelineCreationError::ShaderCreationFailed(e)
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
    pipeline: halw::GraphicsPipeline,
    _p: std::marker::PhantomData<Config>,
}

impl<Config> Pipeline<Config>
where
    Config: PipelineConfig,
{
    pub fn create(
        instance: &Instance,
        canvas: Rc<RefCell<dyn Canvas>>,
    ) -> Result<Self, PipelineCreationError> {
        let layout = Self::create_layout(Rc::clone(&instance.gpu_rc()))?;
        let pipeline = Self::create_pipeline(
            Rc::clone(&instance.gpu_rc()),
            canvas.borrow().render_pass(),
            &layout,
        )?;
        Ok(Self {
            canvas,
            layout,
            pipeline,
            _p: std::marker::PhantomData,
        })
    }

    fn create_layout(
        gpu: Rc<RefCell<halw::Gpu>>,
    ) -> Result<halw::PipelineLayout, hal::device::OutOfMemory> {
        let push_constants_config = {
            let mut push_constants_config = Vec::new();
            if let Some(push_constant_range) =
                Config::vertex_shader_config().push_constant_range.clone()
            {
                push_constants_config
                    .push((hal::pso::ShaderStageFlags::VERTEX, push_constant_range));
            }
            push_constants_config
        };

        let pipeline = halw::PipelineLayout::create(gpu, &[], push_constants_config.iter())?;

        Ok(pipeline)
    }

    fn create_pipeline(
        gpu: Rc<RefCell<halw::Gpu>>,
        render_pass: &halw::RenderPass,
        layout: &halw::PipelineLayout,
    ) -> Result<halw::GraphicsPipeline, PipelineCreationError> {
        let vs_module = halw::ShaderModule::from_spirv(
            Rc::clone(&gpu),
            Config::vertex_shader_config().source.as_slice(),
        )?;
        let vs_entry = halw::EntryPoint {
            entry: "main",
            module: &vs_module,
            specialization: hal::pso::Specialization::default(),
        };

        let fs_module = halw::ShaderModule::from_spirv(
            Rc::clone(&gpu),
            Config::fragment_shader_config().source.as_slice(),
        )?;
        let fs_entry = halw::EntryPoint {
            entry: "main",
            module: &fs_module,
            specialization: hal::pso::Specialization::default(),
        };

        let shader_entries = hal::pso::GraphicsShaderSet {
            vertex: vs_entry,
            fragment: Some(fs_entry),
            geometry: None,
            hull: None,
            domain: None,
        };

        let subpass = halw::Subpass {
            index: 0,
            main_pass: render_pass,
        };

        let mut pipeline_desc = halw::GraphicsPipelineDesc::new(
            shader_entries,
            halw::Primitive::TriangleList,
            halw::Rasterizer::FILL,
            layout,
            subpass,
        );
        pipeline_desc
            .blender
            .targets
            .push(hal::pso::ColorBlendDesc {
                mask: hal::pso::ColorMask::ALL,
                blend: Some(hal::pso::BlendState::ALPHA),
            });
        pipeline_desc
            .vertex_buffers
            .push(hal::pso::VertexBufferDesc {
                binding: 0,
                stride: std::mem::size_of::<Config::Vertex>() as u32,
                rate: hal::pso::VertexInputRate::Vertex,
            });
        let pipeline = halw::GraphicsPipeline::create(gpu, &pipeline_desc, None)?;
        Ok(pipeline)
    }
}
