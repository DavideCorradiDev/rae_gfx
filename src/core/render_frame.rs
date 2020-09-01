use std::{default::Default, iter, ops::DerefMut};

use super::{
    BufferSlice, Canvas, Color, CommandEncoder, CommandEncoderDescriptor, Instance, Operations,
    RenderPass, RenderPassColorAttachmentDescriptor, RenderPassDepthStencilAttachmentDescriptor,
    RenderPassDescriptor, RenderPipeline, ShaderStage, SwapChainError, SwapChainFrame,
    SwapChainTexture, TextureView,
};

#[derive(Debug)]
pub struct RenderFrame<'a> {
    render_pass: Option<RenderPass<'a>>,
    command_encoder: Box<CommandEncoder>,
    swap_chain_texture: Option<Box<SwapChainTexture>>,
}

impl<'a> RenderFrame<'a> {
    pub fn from_canvas<CanvasType: Canvas>(
        instance: &Instance,
        canvas: &'a mut CanvasType,
    ) -> Result<Self, SwapChainError> {
        let swap_chain_frame = canvas.get_swap_chain_frame()?;
        Ok(Self::from_buffers(
            instance,
            swap_chain_frame,
            canvas.get_color_buffer(),
            canvas.get_depth_stencil_buffer(),
            canvas.get_color_operations(),
            canvas.get_depth_operations(),
            canvas.get_stencil_operations(),
        ))
    }

    pub fn from_buffers(
        instance: &Instance,
        swap_chain_frame: Option<SwapChainFrame>,
        color_buffer: Option<&'a TextureView>,
        depth_stencil_buffer: Option<&'a TextureView>,
        color_ops: Option<Operations<Color>>,
        depth_ops: Option<Operations<f32>>,
        stencil_ops: Option<Operations<u32>>,
    ) -> Self {
        let mut command_encoder =
            Box::new(instance.create_command_encoder(&CommandEncoderDescriptor::default()));

        let swap_chain_texture = match swap_chain_frame {
            Some(v) => Some(Box::new(v.output)),
            None => None,
        };

        // The render pass will hold a reference to the command encoder and the texture views.
        // Since it will store them inside a Box, on the heap, their addresses should be stable.
        // Additional borrowing of the resources is prevented by keeping these resources hidden
        // inside the struct.
        let command_encoder_ref =
            unsafe { &mut *(command_encoder.deref_mut() as *mut CommandEncoder) };

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

        let render_pass = Some(
            command_encoder_ref.begin_render_pass(&RenderPassDescriptor {
                color_attachments: color_attachments.as_slice(),
                depth_stencil_attachment,
            }),
        );

        Self {
            render_pass,
            command_encoder,
            swap_chain_texture,
        }
    }

    pub fn submit(mut self, instance: &Instance) {
        // The render pass must be dropped before the command encoder is finished, because it will
        // add some commands to it during the drop call.
        self.render_pass = None;
        instance.submit(iter::once(self.command_encoder.finish()))
    }

    fn render_pass_mut(&mut self) -> &mut RenderPass<'a> {
        match &mut self.render_pass {
            Some(v) => v,
            None => panic!("Invalid render frame access"),
        }
    }

    pub fn set_pipeline(&mut self, pipeline: &'a RenderPipeline) {
        self.render_pass_mut().set_pipeline(pipeline);
    }

    pub fn set_index_buffer(&mut self, buffer_slice: BufferSlice<'a>) {
        self.render_pass_mut().set_index_buffer(buffer_slice)
    }

    pub fn set_vertex_buffer(&mut self, slot: u32, buffer_slice: BufferSlice<'a>) {
        self.render_pass_mut().set_vertex_buffer(slot, buffer_slice)
    }

    pub fn set_push_constants(&mut self, stages: ShaderStage, offset: u32, data: &[u32]) {
        self.render_pass_mut()
            .set_push_constants(stages, offset, data)
    }

    pub fn draw_indexed(
        &mut self,
        indices: core::ops::Range<u32>,
        base_vertex: i32,
        instances: core::ops::Range<u32>,
    ) {
        self.render_pass_mut()
            .draw_indexed(indices, base_vertex, instances)
    }
}
