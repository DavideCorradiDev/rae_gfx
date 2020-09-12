use std::default::Default;

use super::{ColorBufferFormat, DepthStencilBufferFormat, Instance, SampleCount, TextureView};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct CanvasTextureDescriptor {
    pub sample_count: SampleCount,
    pub color_buffer_format: Option<ColorBufferFormat>,
    pub depth_stencil_buffer_format: Option<DepthStencilBufferFormat>,
}

impl Default for CanvasTextureDescriptor {
    fn default() -> Self {
        Self {
            sample_count: 1,
            color_buffer_format: Some(ColorBufferFormat::default()),
            depth_stencil_buffer_format: Some(DepthStencilBufferFormat::Depth32Float),
        }
    }
}

#[derive(Debug)]
struct ColorBuffer {
    format: ColorBufferFormat,
    buffer: TextureView,
}

#[derive(Debug)]
struct DepthStencilBuffer {
    format: DepthStencilBufferFormat,
    buffer: TextureView,
}

#[derive(Debug)]
pub struct CanvasTexture {
    sample_count: SampleCount,
    depth_stencil_buffer: Option<DepthStencilBuffer>,
    color_buffer: Option<ColorBuffer>,
}

// impl CanvasTexture {
//     pub fn new(instance: &Instance)
// }
