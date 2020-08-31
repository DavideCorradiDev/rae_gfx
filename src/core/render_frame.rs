use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    pin::Pin,
    ptr::NonNull,
};

use super::{
    Color, CommandEncoder, CommandEncoderDescriptor, Instance, LoadOp, Operations, RenderPass,
    RenderPassColorAttachmentDescriptor, RenderPassDescriptor, SwapChainFrame, TextureView,
};

#[derive(Debug)]
pub struct RenderFrame<'a> {
    render_pass: Option<RenderPass<'a>>,
    command_encoder: Box<CommandEncoder>,
    attachment: Box<TextureView>,
    resolve_target: Option<Box<TextureView>>,
}

impl<'a> RenderFrame<'a> {
    pub fn new(
        instance: &Instance,
        attachment: TextureView,
        resolve_target: Option<TextureView>,
    ) -> Self {
        let command_encoder =
            Box::new(instance.create_command_encoder(&CommandEncoderDescriptor::default()));
        let attachment = Box::new(attachment);
        let resolve_target = match resolve_target {
            Some(v) => Some(Box::new(v)),
            None => None,
        };

        let mut render_frame = Self {
            render_pass: None,
            command_encoder,
            attachment,
            resolve_target,
        };
        unsafe {
            let command_encoder_ptr =
                render_frame.command_encoder.deref_mut() as *mut CommandEncoder;
            let attachment_ptr = render_frame.attachment.deref() as *const TextureView;
            let resolve_target_ptr = match &render_frame.resolve_target {
                Some(v) => Some(&*(v.deref() as *const TextureView)),
                None => None,
            };
            let render_pass = (*command_encoder_ptr).begin_render_pass(&RenderPassDescriptor {
                color_attachments: &[RenderPassColorAttachmentDescriptor {
                    attachment: &*attachment_ptr,
                    resolve_target: resolve_target_ptr,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            render_frame.render_pass = Some(render_pass);
        }
        render_frame
    }
}

// TODO: specify clear operations.

// impl<'a> RenderFrame<'a> {
//     pub fn new(
//         instance: &Instance,
//         attachment: TextureView,
//         // resolve_target: Option<Box<TextureView>>,
//     ) -> Pin<Box<Self>> {
//         let mut command_encoder =
//             instance.create_command_encoder(&CommandEncoderDescriptor::default());
//
//         let mut render_frame = Box::pin(Self {
//             render_pass: None,
//             command_encoder,
//             attachment,
//         });
//
//         unsafe {
//             let render_pass = Some(render_frame.command_encoder.begin_render_pass(
//                 &RenderPassDescriptor {
//                     color_attachments: &[RenderPassColorAttachmentDescriptor {
//                         attachment: &render_frame.attachment,
//                         resolve_target: None,
//                         ops: Operations {
//                             load: LoadOp::Clear(Color::BLACK),
//                             store: true,
//                         },
//                     }],
//                     depth_stencil_attachment: None,
//                 },
//             ));
//
//             let mut_ref = Pin::as_mut(&mut render_frame);
//             Pin::get_unchecked_mut(mut_ref).render_pass = render_pass;
//         }
//
//         render_frame
//
//         // let attachment = Box::pin(attachment);
//         // let render_pass = {
//         //     command_encoder.begin_render_pass(&RenderPassDescriptor {
//         //         color_attachments: &[RenderPassColorAttachmentDescriptor {
//         //             attachment: &attachment,
//         //             resolve_target: None,
//         //             ops: Operations {
//         //                 load: LoadOp::Clear(Color::BLACK),
//         //                 store: true,
//         //             },
//         //         }],
//         //         depth_stencil_attachment: None,
//         //     })
//         // };
//
//         // unsafe {
//         //     Self {
//         //         render_pass,
//         //         command_encoder,
//         //         attachment,
//         //     }
//         // }
//     }
// }
