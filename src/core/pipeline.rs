extern crate gfx_hal as hal;

use core::ops::Range;
use std::{cell::RefCell, rc::Rc};

use super::{Canvas, Instance};
use crate::halw;

pub use hal::pso::{
    DescriptorArrayIndex, DescriptorBinding, DescriptorSetLayoutBinding, InstanceRate,
    ShaderStageFlags, VertexBufferDesc, VertexInputRate,
};

#[derive(Debug, PartialEq, Clone)]
pub struct PushConstantLayoutBinding {
    stages: ShaderStageFlags,
    range: Range<u32>,
}

pub trait VertexArray {
    fn stride() -> u32;
    fn render(&self, command_buffer: &mut halw::CommandBuffer);
}

pub trait PushConstant {
    fn bind(&self, command_buffer: &mut halw::CommandBuffer);
}

pub trait PipelineConfig<VA, PC>
where
    VA: VertexArray,
    PC: PushConstant,
{
    fn push_constant_layout_bindings() -> Vec<PushConstantLayoutBinding>;
    fn vertex_buffer_descriptions() -> Vec<VertexBufferDesc>;
    fn vertex_shader_source() -> Vec<u8>;
    fn fragment_shader_source() -> Option<Vec<u8>>;
}

pub struct Pipeline<Config, VA, PC>
where
    Config: PipelineConfig<VA, PC>,
    VA: VertexArray,
    PC: PushConstant,
{
    _canvas: Rc<RefCell<dyn Canvas>>,
    _layout: halw::PipelineLayout,
    _pipeline: halw::GraphicsPipeline,
    _p1: std::marker::PhantomData<Config>,
    _p2: std::marker::PhantomData<VA>,
    _p3: std::marker::PhantomData<PC>,
}

impl<Config, VA, PC> Pipeline<Config, VA, PC>
where
    Config: PipelineConfig<VA, PC>,
    VA: VertexArray,
    PC: PushConstant,
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
            _canvas: canvas,
            _layout: layout,
            _pipeline: pipeline,
            _p1: std::marker::PhantomData,
            _p2: std::marker::PhantomData,
            _p3: std::marker::PhantomData,
        })
    }

    fn create_layout(
        gpu: Rc<RefCell<halw::Gpu>>,
    ) -> Result<halw::PipelineLayout, hal::device::OutOfMemory> {
        let push_constants = {
            let mut push_constants = Vec::new();
            for pc_layout_binding in Config::push_constant_layout_bindings().iter() {
                push_constants.push((pc_layout_binding.stages, pc_layout_binding.range.clone()));
            }
            push_constants
        };

        let pipeline = halw::PipelineLayout::create(gpu, &[], push_constants.iter())?;

        Ok(pipeline)
    }

    fn create_pipeline(
        gpu: Rc<RefCell<halw::Gpu>>,
        render_pass: &halw::RenderPass,
        layout: &halw::PipelineLayout,
    ) -> Result<halw::GraphicsPipeline, PipelineCreationError> {
        let vs_module = halw::ShaderModule::from_spirv(
            Rc::clone(&gpu),
            Config::vertex_shader_source().as_slice(),
        )?;
        let vs_entry_point = halw::EntryPoint {
            entry: "main",
            module: &vs_module,
            specialization: hal::pso::Specialization::default(),
        };

        let fs_module = match Config::fragment_shader_source() {
            Some(v) => {
                let module = halw::ShaderModule::from_spirv(Rc::clone(&gpu), v.as_slice())?;
                Some(module)
            }
            None => None,
        };
        let fs_entry_point = match &fs_module {
            Some(v) => {
                let entry_point = halw::EntryPoint {
                    entry: "main",
                    module: &v,
                    specialization: hal::pso::Specialization::default(),
                };
                Some(entry_point)
            }
            None => None,
        };

        let shader_entries = hal::pso::GraphicsShaderSet {
            vertex: vs_entry_point,
            fragment: fs_entry_point,
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
        pipeline_desc.vertex_buffers = Config::vertex_buffer_descriptions();
        let pipeline = halw::GraphicsPipeline::create(gpu, &pipeline_desc, None)?;
        Ok(pipeline)
    }
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

#[derive(Debug, PartialEq, Clone)]
pub enum RenderingError {
    NotProcessingFrame,
}

impl std::fmt::Display for RenderingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderingError::NotProcessingFrame => write!(f, "No frame is being processed"),
        }
    }
}

impl std::error::Error for RenderingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[cfg(test)]
mod test {
    extern crate rae_app;

    use rae_app::{event, event::EventLoopAnyThread};

    use super::*;
    use crate::core::CanvasWindowBuilder;

    struct TestVertexArray {}

    impl VertexArray for TestVertexArray {
        fn stride() -> u32 {
            8u32
        }

        fn render(&self, _: &mut halw::CommandBuffer) {}
    }

    struct TestPushConstant {}

    impl PushConstant for TestPushConstant {
        fn bind(&self, _: &mut halw::CommandBuffer) {}
    }

    struct TestPipelineConfig {}

    impl PipelineConfig<TestVertexArray, TestPushConstant> for TestPipelineConfig {
        fn push_constant_layout_bindings() -> Vec<PushConstantLayoutBinding> {
            vec![PushConstantLayoutBinding {
                stages: ShaderStageFlags::VERTEX,
                range: 0..16,
            }]
        }

        fn vertex_buffer_descriptions() -> Vec<VertexBufferDesc> {
            vec![VertexBufferDesc {
                binding: 0,
                stride: 8,
                rate: VertexInputRate::Vertex,
            }]
        }

        fn vertex_shader_source() -> Vec<u8> {
            include_bytes!("../shaders/gen/spirv/mesh2d.vert.spv").to_vec()
        }

        fn fragment_shader_source() -> Option<Vec<u8>> {
            Some(include_bytes!("../shaders/gen/spirv/mesh2d.frag.spv").to_vec())
        }
    }

    #[test]
    fn create_pipeline() {
        let instance = Instance::create().unwrap();
        let event_loop = event::EventLoop::<()>::new_any_thread();
        let canvas = Rc::new(RefCell::new(
            CanvasWindowBuilder::new()
                .with_visible(false)
                .build(&instance, &event_loop)
                .unwrap(),
        ));
        let _pipeline = Pipeline::<TestPipelineConfig, _, _>::create(&instance, canvas).unwrap();
    }
}
