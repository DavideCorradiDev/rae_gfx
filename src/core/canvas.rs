use super::{Color, Operations, RenderFrame, SwapChainError, SwapChainFrame, TextureView};

pub trait Canvas {
    fn swap_chain_frame(&mut self) -> Result<Option<SwapChainFrame>, SwapChainError>;
    fn color_buffer(&self) -> Option<&TextureView>;
    fn depth_stencil_buffer(&self) -> Option<&TextureView>;
    fn color_operations(&self) -> Option<Operations<Color>>;
    fn depth_operations(&self) -> Option<Operations<f32>>;
    fn stencil_operations(&self) -> Option<Operations<u32>>;

    fn get_render_frame(&mut self) -> Result<RenderFrame, SwapChainError> {
        let swap_chain_frame = self.swap_chain_frame()?;
        Ok(RenderFrame::from_parts(
            swap_chain_frame,
            self.color_buffer(),
            self.color_operations(),
            self.depth_stencil_buffer(),
            self.depth_operations(),
            self.stencil_operations(),
        ))
    }
}
