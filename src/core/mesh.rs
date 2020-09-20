use ::core::ops::Range;
use std::marker::PhantomData;

use super::{Buffer, BufferInitDescriptor, BufferUsage, Instance, RenderPass};

pub type Index = u16;

#[derive(Debug)]
pub struct IndexedMesh<V: bytemuck::Pod> {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
    _p: PhantomData<V>,
}

impl<V: bytemuck::Pod> IndexedMesh<V> {
    pub fn new(instance: &Instance, vertex_list: &[V], index_list: &[Index]) -> Self {
        let vertex_buffer = Buffer::init(
            &instance,
            &BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertex_list),
                usage: BufferUsage::VERTEX,
            },
        );
        let index_buffer = Buffer::init(
            &instance,
            &BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(index_list),
                usage: BufferUsage::INDEX,
            },
        );
        let index_count = index_list.len() as u32;
        Self {
            vertex_buffer,
            index_buffer,
            index_count,
            _p: PhantomData,
        }
    }

    pub fn index_count(&self) -> u32 {
        self.index_count
    }
}

pub trait IndexedMeshRenderer<'a> {
    fn draw_indexed_mesh<V: bytemuck::Pod>(
        &mut self,
        mesh: &'a IndexedMesh<V>,
        index_range: &Range<u32>,
    );
}

impl<'a> IndexedMeshRenderer<'a> for RenderPass<'a> {
    fn draw_indexed_mesh<V: bytemuck::Pod>(
        &mut self,
        mesh: &'a IndexedMesh<V>,
        index_range: &Range<u32>,
    ) {
        self.set_index_buffer(mesh.index_buffer.slice(..));
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.draw_indexed(index_range.clone(), 0, 0..1);
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
