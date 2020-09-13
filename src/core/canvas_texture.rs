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
    multisampled_buffer: Option<TextureView>,
    main_buffer: TextureView,
}

#[derive(Debug)]
struct DepthStencilBuffer {
    format: DepthStencilBufferFormat,
    multisampled_buffer: Option<TextureView>,
    main_buffer: TextureView,
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
            Some(format) => {
                let (main_buffer, multisampled_buffer) = Self::create_texture_views(
                    instance,
                    &desc.size,
                    TextureFormat::from(format),
                    desc.sample_count,
                );
                Some(ColorBuffer {
                    format,
                    multisampled_buffer,
                    main_buffer,
                })
            }
            None => None,
        };
        let depth_stencil_buffer = match desc.depth_stencil_buffer_format {
            Some(format) => {
                let (main_buffer, multisampled_buffer) = Self::create_texture_views(
                    instance,
                    &desc.size,
                    TextureFormat::from(format),
                    desc.sample_count,
                );
                Some(DepthStencilBuffer {
                    format,
                    multisampled_buffer,
                    main_buffer,
                })
            }
            None => None,
        };
        assert!(
            color_buffer.is_some() || depth_stencil_buffer.is_some(),
            "No main_buffer defined for a CanvasTexture"
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

    pub fn sample_count(&self) -> SampleCount {
        self.sample_count
    }

    pub fn color_buffer(&self) -> Option<&TextureView> {
        match &self.color_buffer {
            Some(v) => Some(&v.main_buffer),
            None => None,
        }
    }

    pub fn color_buffer_format(&self) -> Option<ColorBufferFormat> {
        match &self.color_buffer {
            Some(v) => Some(v.format),
            None => None,
        }
    }

    pub fn depth_stencil_buffer(&self) -> Option<&TextureView> {
        match &self.color_buffer {
            Some(v) => Some(&v.main_buffer),
            None => None,
        }
    }

    pub fn depth_stencil_buffer_format(&self) -> Option<DepthStencilBufferFormat> {
        match &self.depth_stencil_buffer {
            Some(v) => Some(v.format),
            None => None,
        }
    }

    fn create_texture_views(
        instance: &Instance,
        size: &Size<u32>,
        format: TextureFormat,
        sample_count: SampleCount,
    ) -> (TextureView, Option<TextureView>) {
        let mut tex_desc = TextureDescriptor {
            size: Extent3d {
                width: size.width(),
                height: size.height(),
                depth: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsage::SAMPLED | TextureUsage::OUTPUT_ATTACHMENT,
            label: None,
        };

        let tex_view =
            Texture::new(instance, &tex_desc).create_view(&TextureViewDescriptor::default());

        let multisampled_tex_view = if sample_count > 0 {
            tex_desc.sample_count = sample_count;
            Some(Texture::new(instance, &tex_desc).create_view(&TextureViewDescriptor::default()))
        } else {
            None
        };

        (tex_view, multisampled_tex_view)
    }
}

// TODO: remove code duplication in the varios Color/Depth Buffer structs.
impl Canvas for CanvasTexture {
    fn current_frame(&mut self) -> Result<CanvasFrame, SwapChainError> {
        let color_buffers = match &self.color_buffer {
            Some(color_buffer) => {
                let multisampled_buffer = match color_buffer.multisampled_buffer {
                    Some(ref v) => Some(v),
                    None => None,
                };
                vec![CanvasColorBuffer {
                    main_buffer: &color_buffer.main_buffer,
                    multisampled_buffer,
                    format: color_buffer.format,
                    sample_count: self.sample_count,
                }]
            }
            None => Vec::new(),
        };
        let depth_stencil_buffer = match &self.depth_stencil_buffer {
            Some(depth_stencil_buffer) => {
                let multisampled_buffer = match depth_stencil_buffer.multisampled_buffer {
                    Some(ref v) => Some(v),
                    None => None,
                };
                Some(CanvasDepthStencilBuffer {
                    main_buffer: &depth_stencil_buffer.main_buffer,
                    multisampled_buffer,
                    format: depth_stencil_buffer.format,
                    sample_count: self.sample_count,
                })
            }
            None => None,
        };
        Ok(CanvasFrame {
            swap_chain_frame: None,
            color_buffers,
            depth_stencil_buffer,
        })
    }
}
