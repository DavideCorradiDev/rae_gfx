extern crate gfx_hal as hal;

use crate::halw;

use std::{cell::RefCell, ops::Deref, rc::Rc};

use super::{Canvas, Instance};

pub struct ShaderConfig {
    source: Vec<u32>,
    push_constant_range: Option<std::ops::Range<u32>>,
}

pub type BufferLength = u64;
pub type VertexCount = hal::VertexCount;

pub trait Mesh {
    type Vertex;
    fn buffer(&self) -> &halw::Buffer;
    fn buffer_len(&self) -> BufferLength;
    fn vertex_byte_count(&self) -> BufferLength;
    fn vertex_count(&self) -> VertexCount;
}

pub trait PipelineConfig {
    type Mesh: Mesh;
    type Constants;
    fn vertex_shader_config() -> &'static ShaderConfig;
    fn fragment_shader_config() -> &'static ShaderConfig;
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

impl From<hal::pso::CreationError> for PipelineCreationError {
    fn from(e: hal::pso::CreationError) -> Self {
        PipelineCreationError::PipelineCreationFailed(e)
    }
}

pub struct Pipeline<Config: PipelineConfig> {
    canvas: Rc<RefCell<dyn Canvas>>,
    _layout: halw::PipelineLayout,
    pipeline: halw::GraphicsPipeline,
    _p: std::marker::PhantomData<Config>,
}

impl<Config> Pipeline<Config>
where
    Config: PipelineConfig,
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
            canvas,
            _layout: layout,
            pipeline,
            _p: std::marker::PhantomData,
        })
    }

    pub fn render(
        &mut self,
        meshes: &[(Config::Mesh, Config::Constants)],
    ) -> Result<(), RenderingError> {
        use hal::command::CommandBuffer as HalCommandBuffer;

        let mut canvas = self.canvas.borrow_mut();
        let cmd_buf = match canvas.current_command_buffer() {
            Some(cmd_buf) => cmd_buf,
            None => return Err(RenderingError::NotProcessingFrame),
        };

        unsafe {
            cmd_buf.bind_graphics_pipeline(&self.pipeline);
            for mesh in meshes {
                // Add push constant handling here.
                cmd_buf.bind_vertex_buffers(
                    0,
                    std::iter::once((
                        mesh.0.buffer().deref(),
                        hal::buffer::SubRange {
                            offset: 0,
                            size: Some(mesh.0.buffer_len()),
                        },
                    )),
                );
                cmd_buf.draw(0..mesh.0.vertex_count(), 0..1);
            }
        }

        Ok(())
    }

    fn create_layout(
        gpu: Rc<RefCell<halw::Gpu>>,
    ) -> Result<halw::PipelineLayout, hal::device::OutOfMemory> {
        let push_constants_config = {
            let mut push_constants_config = Vec::new();
            if let Some(push_constant_range) =
                Config::vertex_shader_config().push_constant_range.clone()
            {
                push_constants_config
                    .push((hal::pso::ShaderStageFlags::VERTEX, push_constant_range));
            }
            push_constants_config
        };

        let pipeline = halw::PipelineLayout::create(gpu, &[], push_constants_config.iter())?;

        Ok(pipeline)
    }

    fn create_pipeline(
        gpu: Rc<RefCell<halw::Gpu>>,
        render_pass: &halw::RenderPass,
        layout: &halw::PipelineLayout,
    ) -> Result<halw::GraphicsPipeline, PipelineCreationError> {
        let vs_module = halw::ShaderModule::from_spirv(
            Rc::clone(&gpu),
            Config::vertex_shader_config().source.as_slice(),
        )?;
        let vs_entry = halw::EntryPoint {
            entry: "main",
            module: &vs_module,
            specialization: hal::pso::Specialization::default(),
        };

        let fs_module = halw::ShaderModule::from_spirv(
            Rc::clone(&gpu),
            Config::fragment_shader_config().source.as_slice(),
        )?;
        let fs_entry = halw::EntryPoint {
            entry: "main",
            module: &fs_module,
            specialization: hal::pso::Specialization::default(),
        };

        let shader_entries = hal::pso::GraphicsShaderSet {
            vertex: vs_entry,
            fragment: Some(fs_entry),
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
        pipeline_desc
            .vertex_buffers
            .push(hal::pso::VertexBufferDesc {
                binding: 0,
                stride: std::mem::size_of::<<Config::Mesh as Mesh>::Vertex>() as u32,
                rate: hal::pso::VertexInputRate::Vertex,
            });
        let pipeline = halw::GraphicsPipeline::create(gpu, &pipeline_desc, None)?;
        Ok(pipeline)
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[derive(Debug, PartialEq, Copy, Clone)]
//     struct MyVertex {
//         pos: [f32; 3],
//         color: [f32; 4],
//     }
//
//     #[derive(Debug, PartialEq, Copy, Clone)]
//     struct MyMesh {
//         vertices: Vec<MyVertex>,
//
//     }
//
//     impl Mesh for MyMesh {
//         type Vertex = MyVertex;
//
//         fn buffer(&self) -> &halw::Buffer
//         {
//
//         }
//
//         fn buffer_len(&self) -> BufferLength {
//             self.vertex_count() as u64 * self.vertex_byte_count()
//         }
//
//         fn vertex_byte_count(&self) -> BufferLength {
//             std::mem::size_of::<Self::Vertex>() as BufferLength
//         }
//
//         fn vertex_count(&self) -> VertexCount {
//             self.vertices.len() as VertexCount
//         }
//     }
// }
//
