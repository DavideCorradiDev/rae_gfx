use ::core::ops::Range;
use std::marker::PhantomData;

use super::{Buffer, BufferInitDescriptor, BufferUsage, Instance, RenderPass};

#[derive(Debug)]
struct TypedBuffer<T: bytemuck::Pod> {
    buffer: Buffer,
    element_count: u32,
    _p: PhantomData<T>,
}

impl<T: bytemuck::Pod> TypedBuffer<T> {
    pub fn new(instance: &Instance, vertex_list: &[T], usage: BufferUsage) -> Self {
        let buffer = Buffer::init(
            &instance,
            &BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertex_list),
                usage,
            },
        );
        let element_count = vertex_list.len() as u32;
        Self {
            buffer,
            element_count,
            _p: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct Mesh<V: bytemuck::Pod> {
    vertex_buffer: TypedBuffer<V>,
}

impl<V: bytemuck::Pod> Mesh<V> {
    pub fn new(instance: &Instance, vertex_list: &[V]) -> Self {
        let vertex_buffer = TypedBuffer::new(instance, vertex_list, BufferUsage::VERTEX);
        Self { vertex_buffer }
    }

    pub fn vertex_count(&self) -> u32 {
        self.vertex_buffer.element_count
    }
}

pub trait MeshRenderer<'a> {
    fn draw_mesh<V>(&mut self, mesh: &'a Mesh<V>)
    where
        V: bytemuck::Pod,
    {
        self.draw_mesh_range(mesh, 0..mesh.vertex_count());
    }

    fn draw_mesh_range<V>(&mut self, mesh: &'a Mesh<V>, vertex_range: Range<u32>)
    where
        V: bytemuck::Pod,
    {
        self.draw_mesh_ranges(mesh, std::iter::once(vertex_range));
    }

    fn draw_mesh_ranges<V, It>(&mut self, mesh: &'a Mesh<V>, vertex_ranges: It)
    where
        V: bytemuck::Pod,
        It: IntoIterator,
        It::Item: Into<Range<u32>>;
}

impl<'a> MeshRenderer<'a> for RenderPass<'a> {
    fn draw_mesh_ranges<V, It>(&mut self, mesh: &'a Mesh<V>, vertex_ranges: It)
    where
        V: bytemuck::Pod,
        It: IntoIterator,
        It::Item: Into<Range<u32>>,
    {
        self.set_vertex_buffer(0, mesh.vertex_buffer.buffer.slice(..));
        for range in vertex_ranges.into_iter() {
            self.draw(range.into(), 0..1);
        }
    }
}

pub type Index = u16;

#[derive(Debug)]
pub struct IndexedMesh<V: bytemuck::Pod> {
    vertex_buffer: TypedBuffer<V>,
    index_buffer: TypedBuffer<Index>,
}

impl<V: bytemuck::Pod> IndexedMesh<V> {
    pub fn new(instance: &Instance, vertex_list: &[V], index_list: &[Index]) -> Self {
        let vertex_buffer = TypedBuffer::new(instance, vertex_list, BufferUsage::VERTEX);
        let index_buffer = TypedBuffer::new(instance, index_list, BufferUsage::INDEX);
        Self {
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn vertex_count(&self) -> u32 {
        self.vertex_buffer.element_count
    }

    pub fn index_count(&self) -> u32 {
        self.index_buffer.element_count
    }
}

pub trait IndexedMeshRenderer<'a> {
    fn draw_indexed_mesh<V>(&mut self, mesh: &'a IndexedMesh<V>)
    where
        V: bytemuck::Pod,
    {
        self.draw_indexed_mesh_range(mesh, 0..mesh.index_count());
    }

    fn draw_indexed_mesh_range<V>(&mut self, mesh: &'a IndexedMesh<V>, index_range: Range<u32>)
    where
        V: bytemuck::Pod,
    {
        self.draw_indexed_mesh_ranges(mesh, std::iter::once(index_range));
    }

    fn draw_indexed_mesh_ranges<V, It>(&mut self, mesh: &'a IndexedMesh<V>, index_ranges: It)
    where
        V: bytemuck::Pod,
        It: IntoIterator,
        It::Item: Into<Range<u32>>;
}

impl<'a> IndexedMeshRenderer<'a> for RenderPass<'a> {
    fn draw_indexed_mesh_ranges<V, It>(&mut self, mesh: &'a IndexedMesh<V>, index_ranges: It)
    where
        V: bytemuck::Pod,
        It: IntoIterator,
        It::Item: Into<Range<u32>>,
    {
        self.set_index_buffer(mesh.index_buffer.buffer.slice(..));
        self.set_vertex_buffer(0, mesh.vertex_buffer.buffer.slice(..));
        for range in index_ranges.into_iter() {
            self.draw_indexed(range.into(), 0, 0..1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use galvanic_assert::{matchers::*, *};

    use crate::core::InstanceDescriptor;

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct Vertex {
        pos: [f32; 2],
    }

    unsafe impl bytemuck::Zeroable for Vertex {
        fn zeroed() -> Self {
            Self { pos: [0., 0.] }
        }
    }

    unsafe impl bytemuck::Pod for Vertex {}

    #[test]
    fn creation() {
        let instance = Instance::new(&InstanceDescriptor::default()).unwrap();
        let mesh = IndexedMesh::<Vertex>::new(
            &instance,
            &[
                Vertex { pos: [1., 2.] },
                Vertex { pos: [3., 4.] },
                Vertex { pos: [5., 6.] },
            ],
            &[0, 1, 1, 2],
        );

        assert_that!(&mesh.index_count(), eq(4));
    }
}
