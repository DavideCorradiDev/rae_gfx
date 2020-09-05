use super::{SwapChainError, SwapChainFrame, TextureFormat, TextureView};

#[derive(Debug)]
pub struct CanvasSwapChainFrame {
    pub frame: SwapChainFrame,
    pub backbuffer: Option<TextureView>,
    pub format: TextureFormat,
    pub sample_count: u32,
}

#[derive(Debug)]
pub struct CanvasBuffer<'a> {
    pub buffer: &'a TextureView,
    pub format: TextureFormat,
    pub sample_count: u32,
}

#[derive(Debug)]
pub struct CanvasFrame<'a> {
    pub swap_chain_frame: Option<CanvasSwapChainFrame>,
    pub color_buffers: Vec<CanvasBuffer<'a>>,
    pub depth_stencil_buffer: Option<CanvasBuffer<'a>>,
}

pub trait Canvas {
    fn current_frame(&mut self) -> Result<CanvasFrame, SwapChainError>;
}
