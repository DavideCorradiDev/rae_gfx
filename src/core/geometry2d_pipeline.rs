extern crate gfx_hal as hal;

use std::ops::Deref;

use hal::command::CommandBuffer as HalCommandBuffer;

use super::{pipeline, BufferCreationError, ImmutableBuffer, Instance, VertexCount};
use crate::halw;

#[derive(Debug, PartialEq, Clone)]
pub struct Vertex {
    pub pos: [f32; 2],
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

#[derive(Debug, PartialEq, Clone)]
pub struct PushConstant {
    color: [f32; 4],
}

impl pipeline::PushConstant for PushConstant {
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

#[derive(Debug, PartialEq, Clone)]
pub struct PipelineConfig {}

impl pipeline::PipelineConfig<VertexArray, PushConstant> for PipelineConfig {
    fn push_constant_layout_bindings() -> Vec<pipeline::PushConstantLayoutBinding> {
        vec![pipeline::PushConstantLayoutBinding {
            stages: pipeline::ShaderStageFlags::VERTEX,
            range: 0..16,
        }]
    }

    fn vertex_buffer_descriptions() -> Vec<pipeline::VertexBufferDesc> {
        vec![pipeline::VertexBufferDesc {
            binding: 0,
            stride: 8,
            rate: pipeline::VertexInputRate::Vertex,
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
