use std::default::Default;

use super::{
    Canvas, CanvasBuffer, CanvasBufferDescriptor, CanvasColorBufferFormat,
    CanvasDepthStencilBufferFormat, CanvasFrame, CanvasSize, Instance, SampleCount, Size,
    SwapChainError, TextureView,
};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct CanvasTextureDescriptor {
    pub size: Size<u32>,
    pub sample_count: SampleCount,
    pub color_buffer_format: Option<CanvasColorBufferFormat>,
    pub depth_stencil_buffer_format: Option<CanvasDepthStencilBufferFormat>,
}

impl Default for CanvasTextureDescriptor {
    fn default() -> Self {
        Self {
            size: Size::new(1, 1),
            sample_count: 1,
            color_buffer_format: Some(CanvasColorBufferFormat::default()),
            depth_stencil_buffer_format: None,
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

    pub fn color_buffer_format(&self) -> Option<CanvasColorBufferFormat> {
        if self.canvas_buffer.color_buffers().is_empty() {
            None
        } else {
            Some(self.canvas_buffer.color_buffers()[0].format())
        }
    }

    pub fn depth_stencil_buffer_format(&self) -> Option<CanvasDepthStencilBufferFormat> {
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

#[cfg(test)]
mod tests {
    use super::*;

    use galvanic_assert::{matchers::*, *};

    use crate::core::InstanceDescriptor;

    #[test]
    fn default_parameters() {
        let instance = Instance::new(&InstanceDescriptor::default()).unwrap();
        let mut texture = CanvasTexture::new(&instance, &CanvasTextureDescriptor::default());

        expect_that!(texture.canvas_size(), eq(CanvasSize::new(1, 1)));
        expect_that!(&texture.sample_count(), eq(1));
        expect_that!(
            &texture.color_buffer_format(),
            eq(Some(CanvasColorBufferFormat::default()))
        );
        expect_that!(&texture.depth_stencil_buffer_format(), eq(None));

        let frame = texture.current_frame().unwrap();
        expect_that!(frame.swap_chain().is_none());
        expect_that!(&frame.color_buffers().len(), eq(1));
        expect_that!(frame.depth_stencil_buffer().is_none());

        let color_buffer_ref = &frame.color_buffers()[0];
        expect_that!(&color_buffer_ref.sample_count(), eq(1));
        expect_that!(
            &color_buffer_ref.format(),
            eq(CanvasColorBufferFormat::default())
        );
    }
}
