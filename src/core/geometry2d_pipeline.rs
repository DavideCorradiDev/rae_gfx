extern crate gfx_hal as hal;
extern crate nalgebra;

use std::ops::Deref;

use hal::command::CommandBuffer as HalCommandBuffer;
use nalgebra::geometry::{Point2, Transform2};

use super::{pipeline, BufferCreationError, Format, ImmutableBuffer, Instance, VertexCount};
use crate::halw;

#[repr(packed)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vertex {
    pub pos: Point2<f32>,
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Point2::from([x, y]),
        }
    }
}

#[derive(Debug)]
pub struct VertexArray {
    buffer: ImmutableBuffer,
    vertex_count: VertexCount,
}

impl VertexArray {
    pub fn from_vertices(
        instance: &Instance,
        data: &[Vertex],
    ) -> Result<Self, BufferCreationError> {
        let buffer = ImmutableBuffer::from_data(instance, data)?;
        Ok(Self {
            buffer,
            vertex_count: data.len() as VertexCount,
        })
    }
}

impl pipeline::VertexArray for VertexArray {
    fn stride() -> u32 {
        std::mem::size_of::<Vertex>() as u32
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

#[repr(packed)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PushConstant {
    pub transform: [f32; 16],
    pub color: [f32; 4],
}

impl pipeline::PushConstant for PushConstant {
    fn bind(&self, pipeline_layout: &halw::PipelineLayout, cmd_buf: &mut halw::CommandBuffer) {
        unsafe {
            let pc: *const PushConstant = self;
            let pc: *const u8 = pc as *const u8;
            let data = std::slice::from_raw_parts(pc, std::mem::size_of::<PushConstant>());
            let (prefix, aligned_data, suffix) = data.align_to::<u32>();
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

#[derive(Debug, PartialEq, Clone)]
pub struct PipelineConfig {}

impl pipeline::PipelineConfig<VertexArray, PushConstant> for PipelineConfig {
    fn push_constant_layout_bindings() -> Vec<pipeline::PushConstantLayoutBinding> {
        vec![pipeline::PushConstantLayoutBinding {
            stages: pipeline::ShaderStageFlags::VERTEX,
            range: 0..std::mem::size_of::<PushConstant>() as u32,
        }]
    }

    fn vertex_buffer_descriptions() -> Vec<pipeline::VertexBufferDesc> {
        vec![pipeline::VertexBufferDesc {
            binding: 0,
            stride: std::mem::size_of::<Vertex>() as pipeline::ElemStride,
            rate: pipeline::VertexInputRate::Vertex,
            vertex_attribute_descs: vec![pipeline::VertexAttributeDesc {
                location: 0,
                format: Format::Rg32Sfloat,
                offset: 0,
            }],
        }]
    }

    fn vertex_shader_source() -> Vec<u8> {
        include_bytes!("shaders/gen/spirv/geometry2d.vert.spv").to_vec()
    }

    fn fragment_shader_source() -> Option<Vec<u8>> {
        Some(include_bytes!("shaders/gen/spirv/geometry2d.frag.spv").to_vec())
    }
}

pub type Pipeline<C> = pipeline::Pipeline<C, PipelineConfig, VertexArray, PushConstant>;
