use std::{default::Default, iter};

use super::{
    CanvasColorBufferFormat, CanvasDepthStencilBufferFormat, CanvasFrame, ColorOperations,
    CommandEncoder, CommandEncoderDescriptor, DepthOperations, Instance, Operations, RenderPass,
    RenderPassColorAttachmentDescriptor, RenderPassDepthStencilAttachmentDescriptor,
    RenderPassDescriptor, SampleCount, StencilOperations,
};

#[derive(Debug, PartialEq, Clone)]
pub struct RenderPassRequirements {
    pub sample_count: SampleCount,
    pub color_buffer_formats: Vec<CanvasColorBufferFormat>,
    pub depth_stencil_buffer_format: Option<CanvasDepthStencilBufferFormat>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RenderPassOperations {
    pub color_operations: Vec<ColorOperations>,
    pub depth_operations: Option<DepthOperations>,
    pub stencil_operations: Option<StencilOperations>,
}

impl Default for RenderPassOperations {
    fn default() -> Self {
        RenderPassOperations {
            color_operations: Vec::new(),
            depth_operations: None,
            stencil_operations: None,
        }
    }
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
        canvas_frame: &'a CanvasFrame,
        requirements: &RenderPassRequirements,
        operations: &RenderPassOperations,
    ) -> RenderPass<'a> {
        // Define color attachments.
        let has_swap_chain = canvas_frame.swap_chain().is_some();
        let required_color_buffer_count = requirements.color_buffer_formats.len();
        let available_color_buffer_count =
            canvas_frame.color_buffers().len() + if has_swap_chain { 1 } else { 0 };
        assert!(
            required_color_buffer_count <= available_color_buffer_count,
            "Failed to begin render pass ({} color buffers were required by the pipeline but only \
             {} were available in the canvas frame)",
            required_color_buffer_count,
            available_color_buffer_count
        );
        let mut color_attachments = Vec::with_capacity(required_color_buffer_count);

        for i in 0..required_color_buffer_count {
            let required_format = requirements.color_buffer_formats[i];
            let ops = match operations.color_operations.get(i) {
                Some(v) => *v,
                None => Operations::default(),
            };
            if i == 0 && has_swap_chain {
                let swap_chain = canvas_frame.swap_chain().unwrap();
                assert!(
                    required_format == swap_chain.format(),
                    "Incompatible swap chain format"
                );
                color_attachments.push(RenderPassColorAttachmentDescriptor {
                    attachment: swap_chain.attachment(),
                    resolve_target: swap_chain.resolve_target(),
                    ops,
                });
            } else {
                let buffer_index = i - if has_swap_chain { 1 } else { 0 };
                let color_buffer = canvas_frame
                    .color_buffers()
                    .get(buffer_index)
                    .expect("Not enough color buffers");
                assert!(
                    required_format == color_buffer.format(),
                    "Incompatible color buffer format"
                );
                color_attachments.push(RenderPassColorAttachmentDescriptor {
                    attachment: color_buffer.attachment(),
                    resolve_target: color_buffer.resolve_target(),
                    ops,
                })
            }
        }

        // Define depth stencil attachments.
        let depth_stencil_attachment = match requirements.depth_stencil_buffer_format {
            Some(required_format) => match canvas_frame.depth_stencil_buffer() {
                Some(ds_buffer) => {
                    assert!(
                        required_format == ds_buffer.format(),
                        "Incompatible depth stencil buffer format"
                    );
                    Some(RenderPassDepthStencilAttachmentDescriptor {
                        attachment: ds_buffer.attachment(),
                        depth_ops: operations.depth_operations,
                        stencil_ops: operations.stencil_operations,
                    })
                }
                None => panic!("Unavailable depth stencil buffer"),
            },
            None => None,
        };

        // Begin the render pass.
        let render_pass_desc = RenderPassDescriptor {
            color_attachments: color_attachments.as_slice(),
            depth_stencil_attachment,
        };
        self.encoder.begin_render_pass(&render_pass_desc)
    }

    pub fn submit(self, instance: &Instance) {
        instance.submit(iter::once(self.encoder.finish()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use galvanic_assert::{matchers::*, *};

    use crate::core::{CanvasBuffer, CanvasBufferDescriptor, CanvasSize, InstanceDescriptor};

    #[test]
    fn creation() {
        let instance = Instance::new(&InstanceDescriptor::default()).unwrap();
        let _cmd_seq = CommandSequence::new(&instance);
    }

    #[test]
    fn render_pass() {
        let instance = Instance::new(&InstanceDescriptor::default()).unwrap();
        let mut cmd_seq = CommandSequence::new(&instance);
        let mut buffer = CanvasBuffer::new(
            &instance,
            &CanvasBufferDescriptor {
                size: CanvasSize::new(12, 20),
                sample_count: 2,
                swap_chain_descriptor: None,
                color_buffer_formats: vec![CanvasColorBufferFormat::default()],
                depth_stencil_buffer_format: Some(CanvasDepthStencilBufferFormat::Depth32Float),
            },
        );

        {
            let frame = buffer.current_frame().unwrap();
            let _rpass = cmd_seq.begin_render_pass(
                &frame,
                &RenderPassRequirements {
                    sample_count: 2,
                    color_buffer_formats: vec![CanvasColorBufferFormat::default()],
                    depth_stencil_buffer_format: Some(CanvasDepthStencilBufferFormat::Depth32Float),
                },
                &RenderPassOperations::default(),
            );
        }

        {
            let frame = buffer.current_frame().unwrap();
            let _rpass = cmd_seq.begin_render_pass(
                &frame,
                &RenderPassRequirements {
                    sample_count: 2,
                    color_buffer_formats: vec![CanvasColorBufferFormat::default()],
                    depth_stencil_buffer_format: Some(CanvasDepthStencilBufferFormat::Depth32Float),
                },
                &RenderPassOperations::default(),
            );
        }

        cmd_seq.submit(&instance);
    }
}
