extern crate gfx_hal as hal;

use core::ops::Range;
use std::{cell::RefCell, rc::Rc};

use hal::command::CommandBuffer as HalCommandBuffer;

use super::{Canvas, Format, Instance};
use crate::halw;

pub use hal::pso::{
    BufferIndex, DescriptorArrayIndex, DescriptorBinding, DescriptorSetLayoutBinding, ElemOffset,
    ElemStride, InstanceRate, Location, ShaderStageFlags, VertexInputRate,
};

pub trait VertexArray {
    fn stride() -> u32;
    fn render(&self, command_buffer: &mut halw::CommandBuffer);
}

pub trait PushConstant {
    fn bind(&self, pipeline_layout: &halw::PipelineLayout, cmd_buf: &mut halw::CommandBuffer);
}

#[derive(Debug, PartialEq, Clone)]
pub struct PushConstantLayoutBinding {
    pub stages: ShaderStageFlags,
    pub range: Range<u32>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VertexAttributeDesc {
    pub location: Location,
    pub format: Format,
    pub offset: ElemOffset,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VertexBufferDesc {
    pub binding: BufferIndex,
    pub stride: ElemStride,
    pub rate: VertexInputRate,
    pub vertex_attribute_descs: Vec<VertexAttributeDesc>,
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

#[derive(Debug)]
pub struct Pipeline<C, Config, VA, PC>
where
    C: Canvas,
    Config: PipelineConfig<VA, PC>,
    VA: VertexArray,
    PC: PushConstant,
{
    canvas: Rc<RefCell<C>>,
    layout: halw::PipelineLayout,
    pipeline: halw::GraphicsPipeline,
    _p1: std::marker::PhantomData<Config>,
    _p2: std::marker::PhantomData<VA>,
    _p3: std::marker::PhantomData<PC>,
}

impl<C, Config, VA, PC> Pipeline<C, Config, VA, PC>
where
    C: Canvas,
    Config: PipelineConfig<VA, PC>,
    VA: VertexArray,
    PC: PushConstant,
{
    pub fn create(
        instance: &Instance,
        canvas: Rc<RefCell<C>>,
    ) -> Result<Self, PipelineCreationError> {
        let layout = Self::create_layout(Rc::clone(&instance.gpu_rc()))?;
        let pipeline = Self::create_pipeline(
            Rc::clone(&instance.gpu_rc()),
            canvas.borrow().render_pass(),
            &layout,
        )?;
        Ok(Self {
            canvas,
            layout: layout,
            pipeline,
            _p1: std::marker::PhantomData,
            _p2: std::marker::PhantomData,
            _p3: std::marker::PhantomData,
        })
    }

    pub fn render(&mut self, elements: &[(&PC, &VA)]) -> Result<(), RenderingError> {
        let mut canvas = self.canvas.borrow_mut();
        let cmd_buf = match canvas.current_command_buffer_mut() {
            Some(v) => v,
            None => return Err(RenderingError::NotProcessingFrame),
        };

        unsafe {
            cmd_buf.bind_graphics_pipeline(&self.pipeline);
            for element in elements {
                element.0.bind(&self.layout, cmd_buf);
                element.1.render(cmd_buf);
            }
        }

        Ok(())
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
        for vbd in Config::vertex_buffer_descriptions() {
            pipeline_desc
                .vertex_buffers
                .push(hal::pso::VertexBufferDesc {
                    binding: vbd.binding,
                    stride: vbd.stride,
                    rate: vbd.rate,
                });
            for vad in vbd.vertex_attribute_descs {
                pipeline_desc.attributes.push(hal::pso::AttributeDesc {
                    binding: vbd.binding,
                    location: vad.location,
                    element: hal::pso::Element {
                        format: vad.format,
                        offset: vad.offset,
                    },
                });
            }
        }
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
    extern crate galvanic_assert;
    extern crate rae_app;

    use std::ops::Deref;

    use galvanic_assert::{matchers::*, *};
    use rae_app::{event, event::EventLoopAnyThread};

    use super::*;
    use crate::core::{
        BufferCreationError, CanvasWindow, CanvasWindowBuilder, ImmutableBuffer, VertexCount,
    };

    struct TestVertex {
        _pos: [f32; 2],
    }

    struct TestVertexArray {
        buffer: ImmutableBuffer,
        vertex_count: VertexCount,
    }

    impl TestVertexArray {
        pub fn new(instance: &Instance, data: &[TestVertex]) -> Result<Self, BufferCreationError> {
            let buffer = ImmutableBuffer::from_data(instance, data)?;
            Ok(Self {
                buffer,
                vertex_count: data.len() as VertexCount,
            })
        }
    }

    impl VertexArray for TestVertexArray {
        fn stride() -> u32 {
            std::mem::size_of::<TestVertex>() as u32
        }

        fn render(&self, cmd_buf: &mut halw::CommandBuffer) {
            unsafe {
                cmd_buf.bind_vertex_buffers(
                    0,
                    std::iter::once((
                        self.buffer.buffer().deref(),
                        hal::buffer::SubRange {
                            offset: 0,
                            size: Some(self.buffer.len()),
                        },
                    )),
                );
                cmd_buf.draw(0..self.vertex_count, 0..1);
            }
        }
    }

    struct TestPushConstant {
        color: [f32; 4],
    }

    impl PushConstant for TestPushConstant {
        fn bind(&self, pipeline_layout: &halw::PipelineLayout, cmd_buf: &mut halw::CommandBuffer) {
            unsafe {
                let (prefix, aligned_data, suffix) = self.color.align_to::<u32>();
                assert!(prefix.len() == 0 && suffix.len() == 0);
                cmd_buf.push_graphics_constants(
                    pipeline_layout,
                    hal::pso::ShaderStageFlags::VERTEX,
                    0,
                    &aligned_data,
                );
            }
        }
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
                vertex_attribute_descs: vec![VertexAttributeDesc {
                    location: 0,
                    format: Format::Rg32Sfloat,
                    offset: 0,
                }],
            }]
        }

        fn vertex_shader_source() -> Vec<u8> {
            include_bytes!("shaders/gen/spirv/test.vert.spv").to_vec()
        }

        fn fragment_shader_source() -> Option<Vec<u8>> {
            Some(include_bytes!("shaders/gen/spirv/test.frag.spv").to_vec())
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
        let _pipeline =
            Pipeline::<CanvasWindow, TestPipelineConfig, _, _>::create(&instance, canvas).unwrap();
    }

    #[test]
    fn render() {
        let instance = Instance::create().unwrap();
        let event_loop = event::EventLoop::<()>::new_any_thread();
        let canvas = Rc::new(RefCell::new(
            CanvasWindowBuilder::new()
                .with_visible(false)
                .build(&instance, &event_loop)
                .unwrap(),
        ));
        let mut pipeline = Pipeline::<CanvasWindow, TestPipelineConfig, _, _>::create(
            &instance,
            Rc::clone(&canvas),
        )
        .unwrap();

        let va1 = TestVertexArray::new(
            &instance,
            &[
                TestVertex { _pos: [0., 1.] },
                TestVertex { _pos: [1., 0.] },
                TestVertex { _pos: [0., 0.] },
            ],
        )
        .unwrap();

        let va2 = TestVertexArray::new(
            &instance,
            &[
                TestVertex { _pos: [0., 1.] },
                TestVertex { _pos: [1., 0.] },
                TestVertex { _pos: [0., 0.] },
            ],
        )
        .unwrap();

        let white = TestPushConstant {
            color: [1., 1., 1., 1.],
        };

        let red = TestPushConstant {
            color: [1., 0., 0., 1.],
        };

        assert_that!(
            &pipeline.render(&[]),
            eq(Err(RenderingError::NotProcessingFrame))
        );

        canvas.borrow_mut().begin_frame().unwrap();
        pipeline
            .render(&[
                (&white, &va1),
                (&red, &va1),
                (
                    &TestPushConstant {
                        color: [0., 1., 0., 1.],
                    },
                    &va2,
                ),
            ])
            .unwrap();
        pipeline.render(&[]).unwrap();
        pipeline.render(&[(&white, &va1)]).unwrap();
        canvas.borrow_mut().end_frame().unwrap();

        assert_that!(
            &pipeline.render(&[]),
            eq(Err(RenderingError::NotProcessingFrame))
        );
    }
}
