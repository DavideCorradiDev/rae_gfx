use super::{Color, Operations, SwapChainError, SwapChainFrame, TextureView};

pub trait Canvas {
    fn get_swap_chain_frame(&mut self) -> Result<Option<SwapChainFrame>, SwapChainError>;
    fn get_color_buffer(&self) -> Option<&TextureView>;
    fn get_depth_stencil_buffer(&self) -> Option<&TextureView>;
    fn get_color_operations(&self) -> Option<Operations<Color>>;
    fn get_depth_operations(&self) -> Option<Operations<f32>>;
    fn get_stencil_operations(&self) -> Option<Operations<u32>>;
}
