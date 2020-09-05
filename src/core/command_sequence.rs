use std::iter;

use super::{
    Color, CommandEncoder, CommandEncoderDescriptor, Instance, Operations, RenderFrame, RenderPass,
    TextureFormat,
};

#[derive(Debug, PartialEq, Clone)]
pub struct RenderPassRequirements {
    pub sample_count: u32,
    pub color_buffer_formats: Vec<TextureFormat>,
    pub depth_stencil_buffer_format: Option<TextureFormat>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RenderPassOperations {
    pub color_operations: Vec<Operations<Color>>,
    pub depth_operations: Option<Operations<f32>>,
    pub stencil_operations: Option<Operations<u32>>,
}

#[derive(Debug)]
pub struct CommandSequence {
    encoder: CommandEncoder,
}

impl CommandSequence {
    pub fn new(instance: &Instance) -> Self {
        let encoder = CommandEncoder::new(&instance, &CommandEncoderDescriptor::default());
        Self { encoder }
    }

    pub fn begin_render_pass<'a>(
        &'a mut self,
        render_frame: &'a RenderFrame,
        requirements: &RenderPassRequirements,
        operations: &RenderPassOperations,
    ) -> RenderPass<'a> {
        self.encoder
            .begin_render_pass(&render_frame.render_pass_descriptor())
    }

    pub fn submit(self, instance: &Instance) {
        instance.submit(iter::once(self.encoder.finish()))
    }
}
