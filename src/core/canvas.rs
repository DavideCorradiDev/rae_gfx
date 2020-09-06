use super::{SampleCount, SwapChainError, SwapChainFrame, TextureFormat, TextureView};

#[derive(Debug)]
pub struct CanvasSwapChainFrame<'a> {
    pub frame: SwapChainFrame,
    pub backbuffer: Option<&'a TextureView>,
    pub format: TextureFormat,
    pub sample_count: SampleCount,
}

#[derive(Debug)]
pub struct CanvasBuffer<'a> {
    pub buffer: &'a TextureView,
    pub format: TextureFormat,
    pub sample_count: SampleCount,
}

#[derive(Debug)]
pub struct CanvasFrame<'a> {
    pub swap_chain_frame: Option<CanvasSwapChainFrame<'a>>,
    pub color_buffers: Vec<CanvasBuffer<'a>>,
    pub depth_stencil_buffer: Option<CanvasBuffer<'a>>,
}

pub trait Canvas {
    fn current_frame(&mut self) -> Result<CanvasFrame, SwapChainError>;
}
