use super::{Color, Operations, SwapChainError, SwapChainFrame, TextureView};

pub trait Canvas {
    fn swap_chain_frame(&mut self) -> Result<Option<SwapChainFrame>, SwapChainError>;
    fn color_buffer(&self) -> Option<&TextureView>;
    fn depth_stencil_buffer(&self) -> Option<&TextureView>;
    fn color_operations(&self) -> Option<Operations<Color>>;
    fn depth_operations(&self) -> Option<Operations<f32>>;
    fn stencil_operations(&self) -> Option<Operations<u32>>;
}
