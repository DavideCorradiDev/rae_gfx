use super::{
    ColorBufferFormat, DepthStencilBufferFormat, SampleCount, SwapChainError, SwapChainFrame,
    TextureView,
};

#[derive(Debug)]
pub struct CanvasSwapChainFrame<'a> {
    pub frame: SwapChainFrame,
    pub multisampled_buffer: Option<&'a TextureView>,
    pub format: ColorBufferFormat,
    pub sample_count: SampleCount,
}

#[derive(Debug)]
pub struct CanvasColorBuffer<'a> {
    pub main_buffer: &'a TextureView,
    pub multisampled_buffer: Option<&'a TextureView>,
    pub format: ColorBufferFormat,
    pub sample_count: SampleCount,
}

#[derive(Debug)]
pub struct CanvasDepthStencilBuffer<'a> {
    pub main_buffer: &'a TextureView,
    pub multisampled_buffer: Option<&'a TextureView>,
    pub format: DepthStencilBufferFormat,
    pub sample_count: SampleCount,
}

#[derive(Debug)]
pub struct CanvasFrame<'a> {
    pub swap_chain_frame: Option<CanvasSwapChainFrame<'a>>,
    pub color_buffers: Vec<CanvasColorBuffer<'a>>,
    pub depth_stencil_buffer: Option<CanvasDepthStencilBuffer<'a>>,
}

pub trait Canvas {
    fn current_frame(&mut self) -> Result<CanvasFrame, SwapChainError>;
}
