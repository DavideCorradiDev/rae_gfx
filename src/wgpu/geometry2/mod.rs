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
    pub fn new(device: &core::Device, config: &RenderPipelineConfig) -> Self {
        let pipeline_layout = device.create_pipeline_layout(&core::PipelineLayoutDescriptor {
            // TODO: define proper push constant / uniform layouts.
            label: Some("geometry2_pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let vs_module = device
            .create_shader_module(core::include_spirv!("shaders/gen/spirv/geometry2.vert.spv"));
        let fs_module = device
            .create_shader_module(core::include_spirv!("shaders/gen/spirv/geometry2.frag.spv"));
        let pipeline = device.create_render_pipeline(&core::RenderPipelineDescriptor {
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
                format: device.color_format(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let device = core::Device::new(&core::DeviceConfig::default(), None).unwrap();
        let _pipeline = RenderPipeline::new(&device, &RenderPipelineConfig::default());
    }
}
