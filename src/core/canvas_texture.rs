use std::default::Default;

use super::{
    ColorBufferFormat, DepthStencilBufferFormat, Extent3d, Instance, SampleCount, Size, Texture,
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
            sample_count: desc.sample_count,
            color_buffer,
            depth_stencil_buffer,
        }
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
