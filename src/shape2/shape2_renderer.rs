use ::core::{iter::IntoIterator, ops::Range};
use std::{borrow::Borrow, default::Default};

use num_traits::Zero;

use rae_math::{conversion::ToHomogeneous3, geometry2, geometry3};

use crate::core;

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Vertex {
    pub position: geometry2::Point<f32>,
}

impl Vertex {
    pub fn new(position: [f32; 2]) -> Self {
        Self {
            position: geometry2::Point::from(position),
        }
    }
}

unsafe impl bytemuck::Zeroable for Vertex {
    fn zeroed() -> Self {
        Self::new([0., 0.])
    }
}

unsafe impl bytemuck::Pod for Vertex {}

pub type Index = u16;

#[derive(Debug)]
pub struct Mesh {
    vertex_buffer: core::Buffer,
    index_buffer: core::Buffer,
    index_count: u32,
}

impl Mesh {
    pub fn new<'a>(
        instance: &core::Instance,
        vertex_list: &[Vertex],
        index_list: &[Index],
    ) -> Self {
        let vertex_buffer = core::Buffer::init(
            &instance,
            &core::BufferInitDescriptor {
                label: Some("shape2_mesh_vertex_buffer"),
                contents: bytemuck::cast_slice(vertex_list),
                usage: core::BufferUsage::VERTEX,
            },
        );
        let index_buffer = core::Buffer::init(
            &instance,
            &core::BufferInitDescriptor {
                label: Some("shape2_mesh_index_buffer"),
                contents: bytemuck::cast_slice(index_list),
                usage: core::BufferUsage::INDEX,
            },
        );
        let index_count = index_list.len() as u32;
        Self {
            vertex_buffer,
            index_buffer,
            index_count,
        }
    }

    pub fn index_count(&self) -> u32 {
        self.index_count
    }
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct PushConstants {
    transform: geometry3::HomogeneousMatrix<f32>,
    color: [f32; 4],
}

impl PushConstants {
    pub fn new(transform: &geometry2::Transform<f32>, color: core::Color) -> Self {
        Self {
            transform: transform.to_homogeneous3(),
            color: [
                color.r as f32,
                color.g as f32,
                color.b as f32,
                color.a as f32,
            ],
        }
    }

    fn as_slice(&self) -> &[u32] {
        let pc: *const PushConstants = self;
        let pc: *const u8 = pc as *const u8;
        let data = unsafe { std::slice::from_raw_parts(pc, std::mem::size_of::<PushConstants>()) };
        bytemuck::cast_slice(&data)
    }
}

unsafe impl bytemuck::Zeroable for PushConstants {
    fn zeroed() -> Self {
        Self {
            transform: geometry3::HomogeneousMatrix::zero(),
            color: [0., 0., 0., 0.],
        }
    }
}

unsafe impl bytemuck::Pod for PushConstants {}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct RenderPipelineDescriptor {
    pub color_blend: core::BlendDescriptor,
    pub alpha_blend: core::BlendDescriptor,
    pub write_mask: core::ColorWrite,
    pub color_buffer_format: core::ColorBufferFormat,
    pub sample_count: core::SampleCount,
}

impl Default for RenderPipelineDescriptor {
    fn default() -> Self {
        Self {
            color_blend: core::BlendDescriptor {
                src_factor: core::BlendFactor::SrcAlpha,
                dst_factor: core::BlendFactor::OneMinusSrcAlpha,
                operation: core::BlendOperation::Add,
            },
            alpha_blend: core::BlendDescriptor {
                src_factor: core::BlendFactor::One,
                dst_factor: core::BlendFactor::One,
                operation: core::BlendOperation::Max,
            },
            write_mask: core::ColorWrite::ALL,
            color_buffer_format: core::ColorBufferFormat::default(),
            sample_count: 1,
        }
    }
}

#[derive(Debug)]
pub struct RenderPipeline {
    pipeline: core::RenderPipeline,
    sample_count: core::SampleCount,
    color_buffer_format: core::ColorBufferFormat,
}

impl RenderPipeline {
    pub fn new(instance: &core::Instance, desc: &RenderPipelineDescriptor) -> Self {
        let pipeline_layout = core::PipelineLayout::new(
            &instance,
            &core::PipelineLayoutDescriptor {
                label: Some("shape2_pipeline_layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[core::PushConstantRange {
                    stages: core::ShaderStage::VERTEX,
                    range: 0..std::mem::size_of::<PushConstants>() as u32,
                }],
            },
        );
        let vs_module = core::ShaderModule::new(
            &instance,
            core::include_spirv!("shaders/gen/spirv/shape2.vert.spv"),
        );
        let fs_module = core::ShaderModule::new(
            &instance,
            core::include_spirv!("shaders/gen/spirv/shape2.frag.spv"),
        );
        let pipeline = core::RenderPipeline::new(
            &instance,
            &core::RenderPipelineDescriptor {
                label: Some("shape2_render_pipeline"),
                layout: Some(&pipeline_layout),
                vertex_stage: core::ProgrammableStageDescriptor {
                    module: &vs_module,
                    entry_point: "main",
                },
                fragment_stage: Some(core::ProgrammableStageDescriptor {
                    module: &fs_module,
                    entry_point: "main",
                }),
                rasterization_state: Some(core::RasterizationStateDescriptor {
                    front_face: core::FrontFace::Ccw,
                    cull_mode: core::CullMode::Back,
                    ..Default::default()
                }),
                primitive_topology: core::PrimitiveTopology::TriangleList,
                color_states: &[core::ColorStateDescriptor {
                    format: core::TextureFormat::from(desc.color_buffer_format),
                    color_blend: desc.color_blend.clone(),
                    alpha_blend: desc.alpha_blend.clone(),
                    write_mask: desc.write_mask,
                }],
                depth_stencil_state: None,
                vertex_state: core::VertexStateDescriptor {
                    index_format: core::IndexFormat::Uint16,
                    vertex_buffers: &[core::VertexBufferDescriptor {
                        stride: std::mem::size_of::<Vertex>() as core::BufferAddress,
                        step_mode: core::InputStepMode::Vertex,
                        attributes: &[core::VertexAttributeDescriptor {
                            format: core::VertexFormat::Float2,
                            offset: 0,
                            shader_location: 0,
                        }],
                    }],
                },
                sample_count: desc.sample_count,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            },
        );
        Self {
            pipeline,
            sample_count: desc.sample_count,
            color_buffer_format: desc.color_buffer_format,
        }
    }

    pub fn render_pass_requirements(&self) -> core::RenderPassRequirements {
        core::RenderPassRequirements {
            sample_count: self.sample_count,
            color_buffer_formats: vec![self.color_buffer_format],
            depth_stencil_buffer_format: None,
        }
    }
}

#[derive(Debug)]
pub struct DrawMesh<'a> {
    pub mesh: &'a Mesh,
    pub index_range: Range<u32>,
    pub constants: &'a PushConstants,
}

pub trait Renderer<'a> {
    fn draw_shape2<It>(&mut self, pipeline: &'a RenderPipeline, draw_mesh_commands: It)
    where
        It: IntoIterator,
        It::Item: Borrow<DrawMesh<'a>>;
}

impl<'a> Renderer<'a> for core::RenderPass<'a> {
    fn draw_shape2<It>(&mut self, pipeline: &'a RenderPipeline, draw_mesh_commands: It)
    where
        It: IntoIterator,
        It::Item: Borrow<DrawMesh<'a>>,
    {
        self.set_pipeline(&pipeline.pipeline);
        for draw_mesh in draw_mesh_commands.into_iter() {
            let draw_mesh = draw_mesh.borrow();
            self.set_index_buffer(draw_mesh.mesh.index_buffer.slice(..));
            self.set_vertex_buffer(0, draw_mesh.mesh.vertex_buffer.slice(..));
            self.set_push_constants(core::ShaderStage::VERTEX, 0, draw_mesh.constants.as_slice());
            self.draw_indexed(draw_mesh.index_range.clone(), 0, 0..1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let instance = core::Instance::new(&core::InstanceDescriptor::default()).unwrap();
        let _pipeline = RenderPipeline::new(&instance, &RenderPipelineDescriptor::default());
    }
}
