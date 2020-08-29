use std::default::Default;

use rae_math::geometry2;

use crate::wgpu::core;

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Vertex {
    pub position: geometry2::Point<f32>,
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: geometry2::Point::from([x, y]),
        }
    }
}

unsafe impl bytemuck::Zeroable for Vertex {
    fn zeroed() -> Self {
        Self::new(0., 0.)
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
            // TODO: define proper push constant / uniform layouts.
            label: Some("geometry2_pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
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
            // TODO: define depth-stencil??
            color_states: &[core::ColorStateDescriptor {
                format: instance.color_format(),
                color_blend: config.color_blend.clone(),
                alpha_blend: config.alpha_blend.clone(),
                write_mask: config.write_mask,
            }],
            depth_stencil_state: None,
            // TODO: define proper vertex buffer state
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
    fn draw_geometry2(&mut self, pipeline: &'a RenderPipeline, mesh: &'a Mesh);
}

impl<'a> Renderer<'a> for core::RenderPass<'a> {
    fn draw_geometry2(&mut self, pipeline: &'a RenderPipeline, mesh: &'a Mesh) {
        self.set_pipeline(&pipeline.pipeline);
        self.set_index_buffer(mesh.index_buffer.slice(..));
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.draw_indexed(0..mesh.index_count as u32, 0, 0..1);
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
