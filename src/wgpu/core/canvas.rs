use super::{SwapChainError, SwapChainFrame};

pub trait Canvas {
    fn get_current_frame(&mut self) -> Result<SwapChainFrame, SwapChainError>;
}
