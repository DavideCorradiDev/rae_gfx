use std::marker::PhantomData;

use super::{Buffer, BufferInitDescriptor, BufferUsage, Instance};

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

    pub fn vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &Buffer {
        &self.index_buffer
    }

    pub fn index_count(&self) -> u32 {
        self.index_count
    }
}
