extern crate gfx_hal as hal;

use std::{cell::RefCell, ops::Deref, rc::Rc};

use hal::{
    adapter::PhysicalDevice as HalPhysicalDevice, command::CommandBuffer as HalCommandBuffer,
    device::Device as HalDevice, queue::CommandQueue as HalCommandQueue,
};

use super::{Canvas, Instance};
use crate::halw;

pub type BufferLength = u64;
pub type VertexCount = hal::VertexCount;

pub trait Mesh {
    type Vertex;
    fn buffer(&self) -> &halw::Buffer;
    fn buffer_len(&self) -> BufferLength;
    fn vertex_byte_count(&self) -> BufferLength;
    fn vertex_count(&self) -> VertexCount;
}

#[derive(Debug, PartialEq, Clone)]
pub enum BufferCreationError {
    CreationFailed(hal::buffer::CreationError),
    MemoryAllocationFailed(hal::device::AllocationError),
    MemoryBindingFailed(hal::device::BindError),
    MemoryMappingFailed(hal::device::MapError),
    OutOfMemory(hal::device::OutOfMemory),
    NoValidMemoryType,
}

impl std::fmt::Display for BufferCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BufferCreationError::CreationFailed(e) => {
                write!(f, "Failed to create the buffer ({})", e)
            }
            BufferCreationError::MemoryAllocationFailed(e) => {
                write!(f, "Failed to allocate memory ({})", e)
            }
            BufferCreationError::MemoryBindingFailed(e) => {
                write!(f, "Failed to bind memory to the buffer ({})", e)
            }
            BufferCreationError::MemoryMappingFailed(e) => {
                write!(f, "Failed to bind CPU to GPU memory ({})", e)
            }
            BufferCreationError::OutOfMemory(e) => write!(f, "Out of memory ({})", e),
            BufferCreationError::NoValidMemoryType => {
                write!(f, "Failed to select a suitable memory type")
            }
        }
    }
}

impl std::error::Error for BufferCreationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BufferCreationError::CreationFailed(e) => Some(e),
            BufferCreationError::MemoryAllocationFailed(e) => Some(e),
            BufferCreationError::MemoryBindingFailed(e) => Some(e),
            BufferCreationError::MemoryMappingFailed(e) => Some(e),
            BufferCreationError::OutOfMemory(e) => Some(e),
            BufferCreationError::NoValidMemoryType => None,
        }
    }
}

impl From<hal::buffer::CreationError> for BufferCreationError {
    fn from(e: hal::buffer::CreationError) -> Self {
        BufferCreationError::CreationFailed(e)
    }
}

impl From<hal::device::AllocationError> for BufferCreationError {
    fn from(e: hal::device::AllocationError) -> Self {
        BufferCreationError::MemoryAllocationFailed(e)
    }
}

impl From<hal::device::BindError> for BufferCreationError {
    fn from(e: hal::device::BindError) -> Self {
        BufferCreationError::MemoryBindingFailed(e)
    }
}

impl From<hal::device::MapError> for BufferCreationError {
    fn from(e: hal::device::MapError) -> Self {
        BufferCreationError::MemoryMappingFailed(e)
    }
}

impl From<hal::device::OutOfMemory> for BufferCreationError {
    fn from(e: hal::device::OutOfMemory) -> Self {
        BufferCreationError::OutOfMemory(e)
    }
}

pub struct ImmutableBuffer {
    memory: halw::Memory,
    buffer: halw::Buffer,
}

impl ImmutableBuffer {
    pub fn from_data(instance: &Instance, data: &[u8]) -> Result<Self, BufferCreationError> {
        use hal::{buffer::Usage, memory::Properties};

        let buffer_len = data.len() as u64;
        let (mut staging_memory, staging_buffer) = Self::create_buffer(
            Rc::clone(&instance.adapter_rc()),
            Rc::clone(&instance.gpu_rc()),
            buffer_len,
            Usage::TRANSFER_SRC,
            Properties::CPU_VISIBLE | Properties::COHERENT,
        )?;
        Self::copy_memory_into_buffer(Rc::clone(&instance.gpu_rc()), data, &mut staging_memory)?;
        let (memory, mut buffer) = Self::create_buffer(
            Rc::clone(&instance.adapter_rc()),
            Rc::clone(&instance.gpu_rc()),
            buffer_len,
            Usage::TRANSFER_DST | Usage::VERTEX,
            Properties::DEVICE_LOCAL,
        )?;
        Self::copy_buffer_into_buffer(
            Rc::clone(&instance.gpu_rc()),
            &staging_buffer,
            &mut buffer,
            buffer_len,
        )?;
        Ok(Self { memory, buffer })
    }

    fn create_buffer(
        adapter: Rc<RefCell<halw::Adapter>>,
        gpu: Rc<RefCell<halw::Gpu>>,
        size: u64,
        usage: hal::buffer::Usage,
        mem_properties: hal::memory::Properties,
    ) -> Result<(halw::Memory, halw::Buffer), BufferCreationError> {
        let mem_types = adapter
            .borrow()
            .physical_device
            .memory_properties()
            .memory_types;
        let mut buffer = halw::Buffer::create(Rc::clone(&gpu), size, usage)?;
        let upload_type = match mem_types.iter().enumerate().position(|(id, mem_type)| {
            (buffer.requirements().type_mask & (1 << id)) != 0
                && (mem_type.properties & mem_properties) == mem_properties
        }) {
            Some(v) => v.into(),
            None => return Err(BufferCreationError::NoValidMemoryType),
        };
        let memory =
            halw::Memory::allocate(Rc::clone(&gpu), upload_type, buffer.requirements().size)?;
        unsafe {
            gpu.borrow()
                .device
                .bind_buffer_memory(&memory, 0, &mut buffer)?;
        }
        Ok((memory, buffer))
    }

    fn copy_memory_into_buffer(
        gpu: Rc<RefCell<halw::Gpu>>,
        data: &[u8],
        mem: &mut halw::Memory,
    ) -> Result<(), BufferCreationError> {
        unsafe {
            let mapping = gpu.borrow().device.map_memory(
                &mem,
                hal::memory::Segment {
                    offset: 0,
                    size: Some(data.len() as u64),
                },
            )?;
            std::ptr::copy_nonoverlapping(data.as_ptr(), mapping, data.len());
            gpu.borrow()
                .device
                .flush_mapped_memory_ranges(std::iter::once((
                    (*mem).deref(),
                    hal::memory::Segment {
                        offset: 0,
                        size: Some(data.len() as u64),
                    },
                )))?;
            gpu.borrow().device.unmap_memory(&mem);
        }
        Ok(())
    }

    fn copy_buffer_into_buffer(
        gpu: Rc<RefCell<halw::Gpu>>,
        src: &halw::Buffer,
        dst: &mut halw::Buffer,
        size: u64,
    ) -> Result<(), BufferCreationError> {
        let cmd_pool = Rc::new(RefCell::new(halw::CommandPool::create(
            Rc::clone(&gpu),
            gpu.borrow().queue_groups[0].family,
            hal::pool::CommandPoolCreateFlags::empty(),
        )?));
        let mut cmd_buf = halw::CommandBuffer::allocate_one(cmd_pool, hal::command::Level::Primary);
        let semaphore = halw::Semaphore::create(Rc::clone(&gpu))?;

        unsafe {
            cmd_buf.begin_primary(hal::command::CommandBufferFlags::ONE_TIME_SUBMIT);
            cmd_buf.copy_buffer(
                src,
                dst,
                std::iter::once(hal::command::BufferCopy {
                    src: 0,
                    dst: 0,
                    size,
                }),
            );
            cmd_buf.finish();

            let submission = hal::queue::Submission {
                command_buffers: std::iter::once(&*cmd_buf),
                wait_semaphores: None,
                signal_semaphores: std::iter::once(&*semaphore),
            };

            let queue = &mut gpu.borrow_mut().queue_groups[0].queues[0];
            queue.submit(submission, None);
            queue.wait_idle()?;
        }
        Ok(())
    }
}

pub struct ShaderConfig {
    source: Vec<u32>,
    push_constant_range: Option<std::ops::Range<u32>>,
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
