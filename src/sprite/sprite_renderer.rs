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
