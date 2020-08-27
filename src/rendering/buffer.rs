extern crate gfx_hal as hal;

use std::{cell::RefCell, ops::Deref, rc::Rc};

use hal::{
    adapter::PhysicalDevice as HalPhysicalDevice, command::CommandBuffer as HalCommandBuffer,
    device::Device as HalDevice, queue::CommandQueue as HalCommandQueue,
};

use crate::{core::Instance, halw};

pub type BufferLength = u64;
pub use hal::{VertexCount, VertexOffset};

#[derive(Debug)]
pub struct ImmutableBuffer {
    memory: halw::Memory,
    buffer: halw::Buffer,
    buffer_len: BufferLength,
}

impl ImmutableBuffer {
    pub fn from_data<T>(instance: &Instance, data: &[T]) -> Result<Self, BufferCreationError> {
        let (prefix, raw_data, suffix) = unsafe { data.align_to::<u8>() };
        assert!(
            prefix.len() == 0 && suffix.len() == 0,
            "Failed to align buffer data"
        );
        Self::from_raw_data(instance, raw_data)
    }

    pub fn from_raw_data(instance: &Instance, data: &[u8]) -> Result<Self, BufferCreationError> {
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
        Ok(Self {
            memory,
            buffer,
            buffer_len,
        })
    }

    pub fn len(&self) -> BufferLength {
        self.buffer_len
    }

    pub fn memory(&self) -> &halw::Memory {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut halw::Memory {
        &mut self.memory
    }

    pub fn buffer(&self) -> &halw::Buffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut halw::Buffer {
        &mut self.buffer
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

#[cfg(test)]
mod tests {
    use galvanic_assert::{matchers::*, *};

    use super::*;

    #[test]
    fn immutable_buffer_from_raw_data() {
        let instance = Instance::create().unwrap();
        let buffer = ImmutableBuffer::from_raw_data(&instance, &[1, 2, 3, 4, 5, 6, 7]).unwrap();
        expect_that!(&buffer.len(), eq(7));
    }

    #[test]
    fn immutable_buffer_from_i32_data() {
        let instance = Instance::create().unwrap();
        let buffer =
            ImmutableBuffer::from_data(&instance, &[1i32, 2i32, 3i32, 4i32, 5i32, 6i32, 7i32])
                .unwrap();
        expect_that!(&buffer.len(), eq(28));
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct MyVertex {
        _pos: [f32; 3],
        _color: [f32; 4],
    }

    #[test]
    fn immutable_buffer_from_struct_data() {
        let instance = Instance::create().unwrap();
        let buffer = ImmutableBuffer::from_data(
            &instance,
            &[
                MyVertex {
                    _pos: [1., 2., 3.],
                    _color: [4., 5., 6., 7.],
                },
                MyVertex {
                    _pos: [-1., 2., -3.],
                    _color: [4., 5., 6., 7.],
                },
            ],
        )
        .unwrap();
        expect_that!(&buffer.len(), eq(56));
    }
}
