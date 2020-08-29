use std::default::Default;

use num_traits::Zero;

use rae_math::{conversion::ToHomogeneous3, geometry2, geometry3};

use crate::wgpu::core;

//TODO: move code to another file, rather than a generic mod.rs

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

// TODO: add static methods to create common shapes and shape outlines.
impl Mesh {
    pub fn new<'a>(
        instance: &core::Instance,
        vertex_list: &[Vertex],
        index_list: &[Index],
    ) -> Self {
        let vertex_buffer = instance.create_buffer_init(&core::BufferInitDescriptor {
            label: Some("geometry2_mesh_vertex_buffer"),
            contents: bytemuck::cast_slice(vertex_list),
            usage: core::BufferUsage::VERTEX,
        });
        let index_buffer = instance.create_buffer_init(&core::BufferInitDescriptor {
            label: Some("geometry2_mesh_index_buffer"),
            contents: bytemuck::cast_slice(index_list),
            usage: core::BufferUsage::INDEX,
        });
        let index_count = index_list.len() as u32;
        Self {
            vertex_buffer,
            index_buffer,
            index_count,
        }
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
pub struct RenderPipelineConfig {
    pub color_blend: core::BlendDescriptor,
    pub alpha_blend: core::BlendDescriptor,
    pub write_mask: core::ColorWrite,
    pub sample_count: u32,
}

impl Default for RenderPipelineConfig {
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
            sample_count: 1,
        }
    }
}

#[derive(Debug)]
pub struct RenderPipeline {
    pipeline: core::RenderPipeline,
}

impl RenderPipeline {
    pub fn new(instance: &core::Instance, config: &RenderPipelineConfig) -> Self {
        let pipeline_layout = instance.create_pipeline_layout(&core::PipelineLayoutDescriptor {
            label: Some("geometry2_pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[core::PushConstantRange {
                stages: core::ShaderStage::VERTEX,
                range: 0..std::mem::size_of::<PushConstants>() as u32,
            }],
        });
        let vs_module = instance
            .create_shader_module(core::include_spirv!("shaders/gen/spirv/geometry2.vert.spv"));
        let fs_module = instance
            .create_shader_module(core::include_spirv!("shaders/gen/spirv/geometry2.frag.spv"));
        let pipeline = instance.create_render_pipeline(&core::RenderPipelineDescriptor {
            label: Some("geometry2_render_pipeline"),
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
                format: instance.color_format(),
                color_blend: config.color_blend.clone(),
                alpha_blend: config.alpha_blend.clone(),
                write_mask: config.write_mask,
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
            sample_count: config.sample_count,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });
        Self { pipeline }
    }
}

pub trait Renderer<'a> {
    fn draw_geometry2(
        &mut self,
        pipeline: &'a RenderPipeline,
        mesh: &'a Mesh,
        constants: &'a PushConstants,
    );
    fn draw_geometry2_array(
        &mut self,
        pipeline: &'a RenderPipeline,
        meshes: &'a [(&'a Mesh, &'a PushConstants)],
    );
}

impl<'a> Renderer<'a> for core::RenderPass<'a> {
    fn draw_geometry2(
        &mut self,
        pipeline: &'a RenderPipeline,
        mesh: &'a Mesh,
        constants: &'a PushConstants,
    ) {
        self.set_pipeline(&pipeline.pipeline);
        self.set_index_buffer(mesh.index_buffer.slice(..));
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_push_constants(core::ShaderStage::VERTEX, 0, constants.as_slice());
        self.draw_indexed(0..mesh.index_count as u32, 0, 0..1);
    }

    fn draw_geometry2_array(
        &mut self,
        pipeline: &'a RenderPipeline,
        meshes: &'a [(&'a Mesh, &'a PushConstants)],
    ) {
        self.set_pipeline(&pipeline.pipeline);
        for (mesh, constants) in meshes.iter() {
            self.set_index_buffer(mesh.index_buffer.slice(..));
            self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            self.set_push_constants(core::ShaderStage::VERTEX, 0, constants.as_slice());
            self.draw_indexed(0..mesh.index_count as u32, 0, 0..1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let instance = core::Instance::new(&core::InstanceConfig::default()).unwrap();
        let _pipeline = RenderPipeline::new(&instance, &RenderPipelineConfig::default());
    }
}
