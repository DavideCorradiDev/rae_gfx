use std::{default::Default, iter};

use super::{
    CanvasFrame, Color, CommandEncoder, CommandEncoderDescriptor, Instance, Operations, RenderPass,
    RenderPassColorAttachmentDescriptor, RenderPassDepthStencilAttachmentDescriptor,
    RenderPassDescriptor, TextureFormat,
};

#[derive(Debug, PartialEq, Clone)]
pub struct RenderPassRequirements {
    pub sample_count: u32,
    pub color_buffer_formats: Vec<TextureFormat>,
    pub depth_stencil_buffer_format: Option<TextureFormat>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RenderPassOperations {
    pub swap_chain_frame_operations: Option<Operations<Color>>,
    pub color_operations: Vec<Operations<Color>>,
    pub depth_operations: Option<Operations<f32>>,
    pub stencil_operations: Option<Operations<u32>>,
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

// TODO: improve error handling
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
        let mut required_color_buffer_count = requirements.color_buffer_formats.len();

        // Define color attachments.
        let mut color_attachments = Vec::new();

        // Main swapchain attachment.
        if required_color_buffer_count > 0 {
            if let Some(swap_chain_frame) = &canvas_frame.swap_chain_frame {
                let frame_view = &swap_chain_frame.frame.output.view;
                let (attachment, resolve_target) = match &swap_chain_frame.backbuffer {
                    Some(backbuffer) => (backbuffer, Some(frame_view)),
                    None => (frame_view, None),
                };
                color_attachments.push(RenderPassColorAttachmentDescriptor {
                    attachment,
                    resolve_target,
                    ops: operations.swap_chain_frame_operations.unwrap_or_default(),
                });
            }
            required_color_buffer_count = required_color_buffer_count - 1;
        }

        // Other color attachments.
        for i in 0..required_color_buffer_count {
            let color_buffer = canvas_frame
                .color_buffers
                .get(i)
                .expect("Not enough color buffers");
            let ops = match operations.color_operations.get(i) {
                Some(v) => *v,
                None => Operations::default(),
            };
            color_attachments.push(RenderPassColorAttachmentDescriptor {
                attachment: color_buffer.buffer,
                resolve_target: None,
                ops,
            })
        }

        // Define depth stencil attachments.
        let depth_stencil_attachment = match requirements.depth_stencil_buffer_format {
            Some(_) => {
                let attachment = match &canvas_frame.depth_stencil_buffer {
                    Some(v) => v.buffer,
                    None => panic!("No depth stencil buffer"),
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

// let swap_chain_texture = match swap_chain_frame {
//     Some(v) => Some(Box::new(v.output)),
//     None => None,
// };

// // Safe to store the address of the swap_chain_texture because it is boxed and isn't exposed
// // by the public API.
// let color_attachment_refs = unsafe {
//     match color_buffer {
//         Some(cv) => match &swap_chain_texture {
//             Some(sct) => Some((cv, Some(&*(&sct.view as *const TextureView)))),
//             None => Some((cv, None)),
//         },
//         None => match &swap_chain_texture {
//             Some(sct) => Some((&*(&sct.view as *const TextureView), None)),
//             None => None,
//         },
//     }
// };

// let color_attachment_descs = match color_attachment_refs {
//     Some(color_attachment_refs) => {
//         let ops = match color_ops {
//             Some(co) => co,
//             None => Operations::<Color>::default(),
//         };
//         vec![RenderPassColorAttachmentDescriptor {
//             attachment: color_attachment_refs.0,
//             resolve_target: color_attachment_refs.1,
//             ops,
//         }]
//     }
//     None => Vec::new(),
// };

// // Safe to store the address of the color attachments because it is in a vec and isn't
// // exposed by the public API.
// let color_attachment_descs_ref = unsafe {
//     let len = color_attachment_descs.len();
//     let ptr = color_attachment_descs.as_ptr();
//     slice::from_raw_parts(ptr, len)
// };

// let depth_stencil_attachment = match depth_stencil_buffer {
//     Some(dsb) => Some(RenderPassDepthStencilAttachmentDescriptor {
//         attachment: dsb,
//         depth_ops: depth_ops,
//         stencil_ops: stencil_ops,
//     }),
//     None => None,
// };

// let render_pass_desc = RenderPassDescriptor {
//     color_attachments: color_attachment_descs_ref,
//     depth_stencil_attachment,
// };

// Self {
//     render_pass_desc,
//     color_attachment_descs,
//     swap_chain_texture,
// }
