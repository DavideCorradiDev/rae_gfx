use ::core::{iter::IntoIterator, ops::Range};
use std::{convert::Into, default::Default};

use num_traits::Zero;

use rae_math::{conversion::ToHomogeneous3, geometry2, geometry3};

use crate::{core, core::IndexedMeshRenderer};

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Vertex {
    pub position: [f32; 2],
    pub texture_coordinates: [f32; 2],
}

impl Vertex {
    pub fn new(position: [f32; 2], texture_coordinates: [f32; 2]) -> Self {
        Self {
            position,
            texture_coordinates,
        }
    }
}

unsafe impl bytemuck::Zeroable for Vertex {
    fn zeroed() -> Self {
        Self::new([0., 0.], [0., 0.])
    }
}

unsafe impl bytemuck::Pod for Vertex {}

pub type Index = core::Index;

pub type Mesh = core::IndexedMesh<Vertex>;

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

fn bind_group_layout(instance: &core::Instance) -> core::BindGroupLayout {
    core::BindGroupLayout::new(
        instance,
        &core::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                core::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: core::ShaderStage::FRAGMENT,
                    ty: core::BindingType::SampledTexture {
                        multisampled: false,
                        component_type: core::TextureComponentType::Float,
                        dimension: core::TextureViewDimension::D2,
                    },
                    count: None,
                },
                core::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: core::ShaderStage::FRAGMENT,
                    ty: core::BindingType::Sampler { comparison: false },
                    count: None,
                },
            ],
        },
    )
}

#[derive(Debug)]
pub struct UniformConstants {
    bind_group: core::BindGroup,
}

impl UniformConstants {
    pub fn new(
        instance: &core::Instance,
        texture: &core::TextureView,
        sampler: &core::Sampler,
    ) -> Self {
        let layout = bind_group_layout(instance);
        let bind_group = core::BindGroup::new(
            instance,
            &core::BindGroupDescriptor {
                label: None,
                layout: &layout,
                entries: &[
                    core::BindGroupEntry {
                        binding: 0,
                        resource: core::BindingResource::TextureView(texture),
                    },
                    core::BindGroupEntry {
                        binding: 1,
                        resource: core::BindingResource::Sampler(sampler),
                    },
                ],
            },
        );
        Self { bind_group }
    }
}

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
    bind_group_layout: core::BindGroupLayout,
    sample_count: core::SampleCount,
    color_buffer_format: core::ColorBufferFormat,
}

impl RenderPipeline {
    pub fn new(instance: &core::Instance, desc: &RenderPipelineDescriptor) -> Self {
        let bind_group_layout = bind_group_layout(instance);
        let pipeline_layout = core::PipelineLayout::new(
            instance,
            &core::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[core::PushConstantRange {
                    stages: core::ShaderStage::VERTEX,
                    range: 0..std::mem::size_of::<PushConstants>() as u32,
                }],
            },
        );
        let vs_module = core::ShaderModule::new(
            instance,
            core::include_spirv!("shaders/gen/spirv/sprite.vert.spv"),
        );
        let fs_module = core::ShaderModule::new(
            instance,
            core::include_spirv!("shaders/gen/spirv/sprite.frag.spv"),
        );
        let pipeline = core::RenderPipeline::new(
            instance,
            &core::RenderPipelineDescriptor {
                label: None,
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
            bind_group_layout,
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
pub struct DrawCommandMeshDescriptor<'a> {
    pub mesh: &'a Mesh,
    pub index_range: Range<u32>,
    pub constants: &'a PushConstants,
}

#[derive(Debug)]
pub struct DrawCommandDescriptor<'a, It>
where
    It: IntoIterator,
    It::Item: Into<DrawCommandMeshDescriptor<'a>>,
{
    pub texture: &'a core::Texture,
    pub sample: &'a core::Sampler,
    pub meshes: It,
}

pub trait Renderer<'a> {
    fn draw_sprite<It, MeshIt>(&mut self, pipeline: &'a RenderPipeline, draw_commands: It)
    where
        It: IntoIterator,
        It::Item: Into<DrawCommandDescriptor<'a, MeshIt>>,
        MeshIt: IntoIterator,
        MeshIt::Item: Into<DrawCommandMeshDescriptor<'a>>;
}

impl<'a> Renderer<'a> for core::RenderPass<'a> {
    fn draw_sprite<It, MeshIt>(&mut self, pipeline: &'a RenderPipeline, draw_commands: It)
    where
        It: IntoIterator,
        It::Item: Into<DrawCommandDescriptor<'a, MeshIt>>,
        MeshIt: IntoIterator,
        MeshIt::Item: Into<DrawCommandMeshDescriptor<'a>>,
    {
        self.set_pipeline(&pipeline.pipeline);
        for draw_command in draw_commands.into_iter() {
            let draw_command = draw_command.into();
            for draw_mesh_command in draw_command.meshes.into_iter() {
                let draw_mesh_command = draw_mesh_command.into();
                self.set_push_constants(
                    core::ShaderStage::VERTEX,
                    0,
                    draw_mesh_command.constants.as_slice(),
                );
                self.draw_indexed_mesh(draw_mesh_command.mesh, &draw_mesh_command.index_range);
            }
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