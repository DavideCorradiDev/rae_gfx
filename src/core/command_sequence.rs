use std::iter;

use super::{CommandEncoder, CommandEncoderDescriptor, Instance, RenderFrame, RenderPass};

#[derive(Debug)]
pub struct CommandSequence {
    encoder: CommandEncoder,
}

impl CommandSequence {
    pub fn new(instance: &Instance) -> Self {
        let encoder = CommandEncoder::new(&instance, &CommandEncoderDescriptor::default());
        Self { encoder }
    }

    pub fn begin_render_pass<'a>(&'a mut self, render_frame: &'a RenderFrame) -> RenderPass<'a> {
        self.encoder
            .begin_render_pass(&render_frame.render_pass_descriptor())
    }

    pub fn submit(self, instance: &Instance) {
        instance.submit(iter::once(self.encoder.finish()))
    }
}
