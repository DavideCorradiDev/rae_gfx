use super::{RenderFrame, SwapChainError};

pub trait Canvas {
    fn get_render_frame(&mut self) -> Result<RenderFrame, SwapChainError>;
}
