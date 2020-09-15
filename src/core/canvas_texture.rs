use std::default::Default;

use super::{
    Canvas, CanvasBuffer, CanvasBufferDescriptor, CanvasFrame, CanvasSize, ColorBufferFormat,
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
    canvas_buffer: CanvasBuffer,
}

impl CanvasTexture {
    pub fn new(instance: &Instance, desc: &CanvasTextureDescriptor) -> Self {
        let canvas_buffer = CanvasBuffer::new(
            instance,
            &CanvasBufferDescriptor {
                size: desc.size,
                sample_count: desc.sample_count,
                swap_chain_descriptor: None,
                color_buffer_formats: match desc.color_buffer_format {
                    Some(format) => vec![format],
                    None => Vec::new(),
                },
                depth_stencil_buffer_format: desc.depth_stencil_buffer_format,
            },
        );
        Self { canvas_buffer }
    }

    pub fn color_buffer_format(&self) -> Option<ColorBufferFormat> {
        if self.canvas_buffer.color_buffers().is_empty() {
            None
        } else {
            Some(self.canvas_buffer.color_buffers()[0].format())
        }
    }

    pub fn depth_stencil_buffer_format(&self) -> Option<DepthStencilBufferFormat> {
        match &self.canvas_buffer.depth_stencil_buffer() {
            Some(v) => Some(v.format()),
            None => None,
        }
    }

    pub fn color_texture_view(&self) -> Option<&TextureView> {
        if self.canvas_buffer.color_buffers().is_empty() {
            None
        } else {
            Some(self.canvas_buffer.color_buffers()[0].texture_view())
        }
    }

    pub fn depth_stencil_texture_view(&self) -> Option<&TextureView> {
        match &self.canvas_buffer.depth_stencil_buffer() {
            Some(v) => Some(v.texture_view()),
            None => None,
        }
    }
}

impl Canvas for CanvasTexture {
    fn current_frame(&mut self) -> Result<CanvasFrame, SwapChainError> {
        self.canvas_buffer.current_frame()
    }

    fn canvas_size(&self) -> &CanvasSize {
        self.canvas_buffer.canvas_size()
    }

    fn sample_count(&self) -> SampleCount {
        self.canvas_buffer.sample_count()
    }
}
