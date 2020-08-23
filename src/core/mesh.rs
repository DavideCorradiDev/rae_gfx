extern crate gfx_hal as hal;

use super::{BufferCreationError, ImmutableBuffer, Instance};

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
        let buffer = ImmutableBuffer::from_data(instance, vertices)?;
        Ok(Self {
            buffer,
            vertex_count: vertices.len() as VertexCount,
            _p: std::marker::PhantomData,
        })
    }

    pub fn buffer(&self) -> &ImmutableBuffer {
        &self.buffer
    }

    pub fn vertex_count(&self) -> VertexCount {
        self.vertex_count
    }
}

#[cfg(test)]
mod tests {
    extern crate galvanic_assert;

    use galvanic_assert::{matchers::*, *};

    use super::*;

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct MyVertex {
        _pos: [f32; 3],
        _color: [f32; 4],
    }

    #[test]
    fn mesh_creation() {
        let instance = Instance::create().unwrap();
        let mesh = Mesh::from_vertices(
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
        expect_that!(&mesh.vertex_count(), eq(2));
    }
}
