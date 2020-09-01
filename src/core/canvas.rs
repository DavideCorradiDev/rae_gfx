use super::{SwapChainError, SwapChainFrame, TextureView};

pub trait Canvas {
    fn get_swap_chain_frame(&mut self) -> Result<Option<SwapChainFrame>, SwapChainError>;
    fn get_framebuffer(&self) -> Option<&TextureView>;
}
