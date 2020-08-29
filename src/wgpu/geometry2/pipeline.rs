use crate::wgpu::core;

#[derive(Debug)]
pub struct RenderPipeline {
    pipeline: core::RenderPipeline,
}

impl RenderPipeline {
    pub fn new(device: &core::Device) -> Self {
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
            // TODO: define alpha blending
            // TODO: define depth-stencil??
            color_states: &[core::ColorStateDescriptor {
                format: device.color_format(),
                color_blend: core::BlendDescriptor::REPLACE,
                alpha_blend: core::BlendDescriptor::REPLACE,
                write_mask: core::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            // TODO: define proper vertex buffer state
            vertex_state: core::VertexStateDescriptor {
                index_format: core::IndexFormat::Uint16,
                vertex_buffers: &[core::VertexBufferDescriptor {
                    stride: 2 as core::BufferAddress,
                    step_mode: core::InputStepMode::Vertex,
                    attributes: &[core::VertexAttributeDescriptor {
                        format: core::VertexFormat::Float2,
                        offset: 0,
                        shader_location: 0,
                    }],
                }],
            },
            // TODO: configurable multisampling.
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });
        Self { pipeline }
    }
}
