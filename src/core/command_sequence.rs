use std::{default::Default, iter};

use super::{
    CanvasFrame, ColorBufferFormat, ColorOperations, CommandEncoder, CommandEncoderDescriptor,
    DepthOperations, DepthStencilBufferFormat, Instance, Operations, RenderPass,
    RenderPassColorAttachmentDescriptor, RenderPassDepthStencilAttachmentDescriptor,
    RenderPassDescriptor, SampleCount, StencilOperations,
};

#[derive(Debug, PartialEq, Clone)]
pub struct RenderPassRequirements {
    pub sample_count: SampleCount,
    pub color_buffer_formats: Vec<ColorBufferFormat>,
    pub depth_stencil_buffer_format: Option<DepthStencilBufferFormat>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RenderPassOperations {
    pub swap_chain_frame_operations: Option<ColorOperations>,
    pub color_operations: Vec<ColorOperations>,
    pub depth_operations: Option<DepthOperations>,
    pub stencil_operations: Option<StencilOperations>,
}

impl Default for RenderPassOperations {
    fn default() -> Self {
        RenderPassOperations {
            swap_chain_frame_operations: None,
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
        let mut color_attachments = Vec::new();
        let mut required_color_buffer_count = requirements.color_buffer_formats.len();
        let available_color_buffer_count = canvas_frame.color_buffers.len()
            + match &canvas_frame.swap_chain_frame {
                Some(_) => 1,
                None => 0,
            };
        assert!(
            required_color_buffer_count <= available_color_buffer_count,
            "Failed to begin render pass ({} color buffers were required by the pipeline but only \
             {} were available in the canvas frame)",
            required_color_buffer_count,
            available_color_buffer_count
        );

        // Main swapchain attachment.
        if required_color_buffer_count > 0 {
            if let Some(swap_chain_frame) = &canvas_frame.swap_chain_frame {
                let frame_view = &swap_chain_frame.frame.output.view;
                let (attachment, resolve_target) = match swap_chain_frame.multisampled_buffer {
                    Some(ms_buffer) => (ms_buffer, Some(frame_view)),
                    None => (frame_view, None),
                };
                color_attachments.push(RenderPassColorAttachmentDescriptor {
                    attachment,
                    resolve_target,
                    ops: operations.swap_chain_frame_operations.unwrap_or_default(),
                });
                required_color_buffer_count = required_color_buffer_count - 1;
            }
        }

        // Other color attachments.
        for i in 0..required_color_buffer_count {
            let color_buffer = canvas_frame
                .color_buffers
                .get(i)
                .expect("Not enough color buffers");
            let (attachment, resolve_target) = match color_buffer.multisampled_buffer {
                Some(ms_buffer) => (ms_buffer, Some(color_buffer.main_buffer)),
                None => (color_buffer.main_buffer, None),
            };
            let ops = match operations.color_operations.get(i) {
                Some(v) => *v,
                None => Operations::default(),
            };
            color_attachments.push(RenderPassColorAttachmentDescriptor {
                attachment,
                resolve_target,
                ops,
            })
        }

        // Define depth stencil attachments.
        let depth_stencil_attachment = match requirements.depth_stencil_buffer_format {
            Some(_) => {
                let (attachment, resolve_target) = match &canvas_frame.depth_stencil_buffer {
                    Some(ds_buffer) => match ds_buffer.multisampled_buffer {
                        Some(ms_buffer) => (ms_buffer, Some(ds_buffer.main_buffer)),
                        None => (ds_buffer.main_buffer, None),
                    },
                    None => panic!(
                        "Failed to begin render pass (A depth stencil buffer was required by the \
                         pipeline but none was available in the canvas frame)",
                    ),
                };
                Some(RenderPassDepthStencilAttachmentDescriptor {
                    attachment,
                    depth_ops: operations.depth_operations,
                    stencil_ops: operations.stencil_operations,
                })
            }
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
