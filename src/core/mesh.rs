extern crate gfx_hal as hal;

use super::{BufferCreationError, BufferLength, ImmutableBuffer, Instance};

pub type VertexCount = hal::VertexCount;

#[derive(Debug)]
pub struct Mesh<Vertex> {
    buffer: ImmutableBuffer,
    vertex_count: VertexCount,
    _p: std::marker::PhantomData<Vertex>,
}

impl<Vertex> Mesh<Vertex> {
    pub fn from_vertices(
        instance: &Instance,
        vertices: &[Vertex],
    ) -> Result<Self, BufferCreationError> {
        let (_, bytes, _) = unsafe { vertices.align_to::<u8>() };
        let buffer = ImmutableBuffer::from_data(instance, bytes)?;
        Ok(Self {
            buffer,
            vertex_count: vertices.len() as VertexCount,
            _p: std::marker::PhantomData,
        })
    }

    pub fn buffer(&self) -> &ImmutableBuffer {
        &self.buffer
    }

    pub fn vertex_byte_count(&self) -> BufferLength {
        std::mem::size_of::<Vertex>() as BufferLength
    }

    pub fn vertex_count(&self) -> VertexCount {
        self.vertex_count
    }
}
