use std::iter;

use super::{
    CanvasFrame, Color, CommandEncoder, CommandEncoderDescriptor, Instance, Operations, RenderPass,
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
        canvas_frame: &'a CanvasFrame,
        requirements: &RenderPassRequirements,
        operations: &RenderPassOperations,
    ) -> RenderPass<'a> {
        let color_attachments = Vec::new();
        let depth_stencil_attachment = None;
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
