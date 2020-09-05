use std::{default::Default, slice};

use super::{
    Color, Operations, RenderPassColorAttachmentDescriptor,
    RenderPassDepthStencilAttachmentDescriptor, RenderPassDescriptor, SwapChainFrame,
    SwapChainTexture, TextureFormat, TextureView,
};

#[derive(Debug)]
pub struct RenderFrame<'a, 'b> {
    render_pass_desc: RenderPassDescriptor<'a, 'b>,
    color_attachment_descs: Vec<RenderPassColorAttachmentDescriptor<'a>>,
    swap_chain_texture: Option<Box<SwapChainTexture>>,
}

impl<'a, 'b> RenderFrame<'a, 'b> {
    pub fn from_parts(
        swap_chain_frame: Option<SwapChainFrame>,
        color_buffer: Option<&'a TextureView>,
        color_ops: Option<Operations<Color>>,
        depth_stencil_buffer: Option<&'a TextureView>,
        depth_ops: Option<Operations<f32>>,
        stencil_ops: Option<Operations<u32>>,
    ) -> Self {
        let swap_chain_texture = match swap_chain_frame {
            Some(v) => Some(Box::new(v.output)),
            None => None,
        };

        // Safe to store the address of the swap_chain_texture because it is boxed and isn't exposed
        // by the public API.
        let color_attachment_refs = unsafe {
            match color_buffer {
                Some(cv) => match &swap_chain_texture {
                    Some(sct) => Some((cv, Some(&*(&sct.view as *const TextureView)))),
                    None => Some((cv, None)),
                },
                None => match &swap_chain_texture {
                    Some(sct) => Some((&*(&sct.view as *const TextureView), None)),
                    None => None,
                },
            }
        };

        let color_attachment_descs = match color_attachment_refs {
            Some(color_attachment_refs) => {
                let ops = match color_ops {
                    Some(co) => co,
                    None => Operations::<Color>::default(),
                };
                vec![RenderPassColorAttachmentDescriptor {
                    attachment: color_attachment_refs.0,
                    resolve_target: color_attachment_refs.1,
                    ops,
                }]
            }
            None => Vec::new(),
        };

        // Safe to store the address of the color attachments because it is in a vec and isn't
        // exposed by the public API.
        let color_attachment_descs_ref = unsafe {
            let len = color_attachment_descs.len();
            let ptr = color_attachment_descs.as_ptr();
            slice::from_raw_parts(ptr, len)
        };

        let depth_stencil_attachment = match depth_stencil_buffer {
            Some(dsb) => Some(RenderPassDepthStencilAttachmentDescriptor {
                attachment: dsb,
                depth_ops: depth_ops,
                stencil_ops: stencil_ops,
            }),
            None => None,
        };

        let render_pass_desc = RenderPassDescriptor {
            color_attachments: color_attachment_descs_ref,
            depth_stencil_attachment,
        };

        Self {
            render_pass_desc,
            color_attachment_descs,
            swap_chain_texture,
        }
    }

    pub fn render_pass_descriptor(&self) -> &RenderPassDescriptor {
        &self.render_pass_desc
    }
}
