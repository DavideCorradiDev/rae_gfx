use std::default::Default;

use super::{
    Canvas, CanvasColorBuffer, CanvasDepthStencilBuffer, CanvasFrame, ColorBufferFormat,
    DepthStencilBufferFormat, Extent3d, Instance, SampleCount, Size, SwapChainError, Texture,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsage, TextureView,
    TextureViewDescriptor,
};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct CanvasTextureDescriptor {
    pub size: Size<u32>,
    pub sample_count: SampleCount,
    pub color_buffer_format: Option<ColorBufferFormat>,
    pub depth_stencil_buffer_format: Option<DepthStencilBufferFormat>,
}

impl Default for CanvasTextureDescriptor {
    fn default() -> Self {
        Self {
            size: Size::new(1, 1),
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
    size: Size<u32>,
    sample_count: SampleCount,
    depth_stencil_buffer: Option<DepthStencilBuffer>,
    color_buffer: Option<ColorBuffer>,
}

impl CanvasTexture {
    pub fn new(instance: &Instance, desc: &CanvasTextureDescriptor) -> Self {
        let color_buffer = match desc.color_buffer_format {
            Some(format) => Some(ColorBuffer {
                format,
                buffer: Self::create_texture_view(
                    instance,
                    &desc.size,
                    TextureFormat::from(format),
                    desc.sample_count,
                ),
            }),
            None => None,
        };
        let depth_stencil_buffer = match desc.depth_stencil_buffer_format {
            Some(format) => Some(DepthStencilBuffer {
                format,
                buffer: Self::create_texture_view(
                    instance,
                    &desc.size,
                    TextureFormat::from(format),
                    desc.sample_count,
                ),
            }),
            None => None,
        };
        assert!(
            color_buffer.is_some() || depth_stencil_buffer.is_some(),
            "No buffer defined for a CanvasTexture"
        );
        Self {
            size: desc.size,
            sample_count: desc.sample_count,
            color_buffer,
            depth_stencil_buffer,
        }
    }

    pub fn size(&self) -> &Size<u32> {
        &self.size
    }

    fn create_texture_view(
        instance: &Instance,
        size: &Size<u32>,
        format: TextureFormat,
        sample_count: SampleCount,
    ) -> TextureView {
        Texture::new(
            instance,
            &TextureDescriptor {
                size: Extent3d {
                    width: size.width(),
                    height: size.height(),
                    depth: 1,
                },
                mip_level_count: 1,
                sample_count,
                dimension: TextureDimension::D2,
                format,
                usage: TextureUsage::OUTPUT_ATTACHMENT,
                label: None,
            },
        )
        .create_view(&TextureViewDescriptor::default())
    }
}

impl Canvas for CanvasTexture {
    fn current_frame(&mut self) -> Result<CanvasFrame, SwapChainError> {
        let color_buffers = match &self.color_buffer {
            Some(color_buffer) => vec![CanvasColorBuffer {
                buffer: &color_buffer.buffer,
                format: color_buffer.format,
                sample_count: self.sample_count,
            }],
            None => Vec::new(),
        };
        let depth_stencil_buffer = match &self.depth_stencil_buffer {
            Some(depth_stencil_buffer) => Some(CanvasDepthStencilBuffer {
                buffer: &depth_stencil_buffer.buffer,
                format: depth_stencil_buffer.format,
                sample_count: self.sample_count,
            }),
            None => None,
        };
        Ok(CanvasFrame {
            swap_chain_frame: None,
            color_buffers,
            depth_stencil_buffer,
        })
    }
}
