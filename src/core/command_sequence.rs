use std::iter;

use super::{
    Canvas, CommandEncoder, CommandEncoderDescriptor, Instance, RenderFrame, SwapChainError,
};

#[derive(Debug)]
pub struct CommandSequence {
    encoder: CommandEncoder,
}

impl CommandSequence {
    pub fn new(instance: &Instance) -> Self {
        let encoder = instance.create_command_encoder(&CommandEncoderDescriptor::default());
        Self { encoder }
    }

    pub fn begin_render_frame<'a, CanvasType: Canvas>(
        &'a mut self,
        canvas: &'a mut CanvasType,
    ) -> Result<RenderFrame<'a>, SwapChainError> {
        let swap_chain_frame = canvas.swap_chain_frame()?;
        Ok(RenderFrame::from_parts(
            &mut self.encoder,
            swap_chain_frame,
            canvas.color_buffer(),
            canvas.color_operations(),
            canvas.depth_stencil_buffer(),
            canvas.depth_operations(),
            canvas.stencil_operations(),
        ))
    }

    pub fn submit(self, instance: &Instance) {
        instance.submit(iter::once(self.encoder.finish()))
    }
}
