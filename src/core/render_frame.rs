use std::default::Default;

use super::{
    BufferSlice, Color, CommandEncoder, Operations, RenderPass,
    RenderPassColorAttachmentDescriptor, RenderPassDepthStencilAttachmentDescriptor,
    RenderPassDescriptor, RenderPipeline, ShaderStage, SwapChainFrame, SwapChainTexture,
    TextureView,
};

#[derive(Debug)]
pub struct RenderFrame<'a> {
    render_pass: RenderPass<'a>,
    swap_chain_texture: Option<Box<SwapChainTexture>>,
}

impl<'a> RenderFrame<'a> {
    pub fn from_parts(
        command_encoder: &'a mut CommandEncoder,
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

        // The render pass will hold a reference to the command encoder and the texture views.
        // Since it will store them inside a Box, on the heap, their addresses should be stable.
        // Additional borrowing of the resources is prevented by keeping these resources hidden
        // inside the struct.
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

        let color_attachments = match color_attachment_refs {
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

        let depth_stencil_attachment = match depth_stencil_buffer {
            Some(dsb) => Some(RenderPassDepthStencilAttachmentDescriptor {
                attachment: dsb,
                depth_ops: depth_ops,
                stencil_ops: stencil_ops,
            }),
            None => None,
        };

        let render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
            color_attachments: color_attachments.as_slice(),
            depth_stencil_attachment,
        });

        Self {
            render_pass,
            swap_chain_texture,
        }
    }

    pub fn set_pipeline(&mut self, pipeline: &'a RenderPipeline) {
        self.render_pass.set_pipeline(pipeline);
    }

    pub fn set_index_buffer(&mut self, buffer_slice: BufferSlice<'a>) {
        self.render_pass.set_index_buffer(buffer_slice)
    }

    pub fn set_vertex_buffer(&mut self, slot: u32, buffer_slice: BufferSlice<'a>) {
        self.render_pass.set_vertex_buffer(slot, buffer_slice)
    }

    pub fn set_push_constants(&mut self, stages: ShaderStage, offset: u32, data: &[u32]) {
        self.render_pass.set_push_constants(stages, offset, data)
    }

    pub fn draw_indexed(
        &mut self,
        indices: core::ops::Range<u32>,
        base_vertex: i32,
        instances: core::ops::Range<u32>,
    ) {
        self.render_pass
            .draw_indexed(indices, base_vertex, instances)
    }
}
