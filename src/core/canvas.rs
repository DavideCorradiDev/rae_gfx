use super::{SwapChainError, SwapChainFrame, TextureFormat, TextureView};

pub trait Canvas {
    fn get_swap_chain_frame(&mut self) -> Result<Option<SwapChainFrame>, SwapChainError>;
    fn get_color_buffer(&self) -> Option<&TextureView>;
    fn get_depth_stencil_buffer(&self) -> Option<&TextureView>;
    fn get_color_format(&self) -> Option<TextureFormat>;
    fn get_depth_stencil_format(&self) -> Option<TextureFormat>;
}
