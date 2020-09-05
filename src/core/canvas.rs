use super::{SwapChainError, SwapChainFrame, TextureFormat, TextureView};

#[derive(Debug)]
pub struct CanvasFrame<'a> {
    pub swap_chain_frame: Option<SwapChainFrame>,
    pub color_buffers: Vec<&'a TextureView>,
    pub color_buffer_formats: Vec<TextureFormat>,
    pub depth_stencil_buffer: Option<&'a TextureView>,
    pub depth_stencil_buffer_format: Option<&'a TextureView>,
}

pub trait Canvas {
    fn current_frame(&mut self) -> Result<CanvasFrame, SwapChainError>;
}
