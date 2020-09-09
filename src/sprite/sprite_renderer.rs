use ::core::{iter::IntoIterator, ops::Range};
use std::{borrow::Borrow, default::Default};

use num_traits::Zero;

use rae_math::{conversion::ToHomogeneous3, geometry2, geometry3};

use crate::core;

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

pub type Index = u16;

#[derive(Debug)]
pub struct Mesh {
    vertex_buffer: core::Buffer,
    index_buffer: core::Buffer,
    index_count: u32,
}

impl Mesh {
    pub fn new<'a>(
        instance: &core::Instance,
        vertex_list: &[Vertex],
        index_list: &[Index],
    ) -> Self {
        let vertex_buffer = core::Buffer::init(
            &instance,
            &core::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertex_list),
                usage: core::BufferUsage::VERTEX,
            },
        );
        let index_buffer = core::Buffer::init(
            &instance,
            &core::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(index_list),
                usage: core::BufferUsage::INDEX,
            },
        );
        let index_count = index_list.len() as u32;
        Self {
            vertex_buffer,
            index_buffer,
            index_count,
        }
    }

    pub fn index_count(&self) -> u32 {
        self.index_count
    }
}
