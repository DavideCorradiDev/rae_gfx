use std::default::Default;

use super::{
    Canvas, CanvasColorBuffer, CanvasColorBufferDescriptor, CanvasDepthStencilBuffer,
    CanvasDepthStencilBufferDescriptor, CanvasFrameRef, ColorBufferFormat,
    DepthStencilBufferFormat, Instance, SampleCount, Size, SwapChainError, TextureView,
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
pub struct CanvasTexture {
    size: Size<u32>,
    sample_count: SampleCount,
    depth_stencil_buffer: Option<CanvasDepthStencilBuffer>,
    color_buffer: Option<CanvasColorBuffer>,
}

impl CanvasTexture {
    pub fn new(instance: &Instance, desc: &CanvasTextureDescriptor) -> Self {
        let (color_buffer, depth_stencil_buffer) = Self::create_buffers(
            instance,
            &desc.size,
            desc.color_buffer_format,
            desc.depth_stencil_buffer_format,
            desc.sample_count,
        );
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
            Some(v) => Some(v.texture_view()),
            None => None,
        }
    }

    pub fn color_buffer_format(&self) -> Option<ColorBufferFormat> {
        match &self.color_buffer {
            Some(v) => Some(v.format()),
            None => None,
        }
    }

    pub fn depth_stencil_buffer(&self) -> Option<&TextureView> {
        match &self.depth_stencil_buffer {
            Some(v) => Some(v.texture_view()),
            None => None,
        }
    }

    pub fn depth_stencil_buffer_format(&self) -> Option<DepthStencilBufferFormat> {
        match &self.depth_stencil_buffer {
            Some(v) => Some(v.format()),
            None => None,
        }
    }

    fn create_buffers(
        instance: &Instance,
        size: &Size<u32>,
        color_format: Option<ColorBufferFormat>,
        depth_stencil_format: Option<DepthStencilBufferFormat>,
        sample_count: SampleCount,
    ) -> (Option<CanvasColorBuffer>, Option<CanvasDepthStencilBuffer>) {
        let color_buffer = match color_format {
            Some(format) => Some(CanvasColorBuffer::new(
                instance,
                &CanvasColorBufferDescriptor {
                    size: *size,
                    format,
                    sample_count,
                },
            )),
            None => None,
        };
        let depth_stencil_buffer = match depth_stencil_format {
            Some(format) => Some(CanvasDepthStencilBuffer::new(
                instance,
                &CanvasDepthStencilBufferDescriptor {
                    size: *size,
                    format,
                    sample_count,
                },
            )),
            None => None,
        };
        (color_buffer, depth_stencil_buffer)
    }
}

impl Canvas for CanvasTexture {
    fn current_frame(&mut self) -> Result<CanvasFrameRef, SwapChainError> {
        let color_buffers = match &self.color_buffer {
            Some(color_buffer) => vec![color_buffer.reference()],
            None => Vec::new(),
        };
        let depth_stencil_buffer = match &self.depth_stencil_buffer {
            Some(depth_stencil_buffer) => Some(depth_stencil_buffer.reference()),
            None => None,
        };
        Ok(CanvasFrameRef {
            swap_chain: None,
            color_buffers,
            depth_stencil_buffer,
        })
    }
}
