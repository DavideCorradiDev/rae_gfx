use std::{
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
};

use super::{
    BufferSlice, Canvas, Color, CommandEncoder, CommandEncoderDescriptor, Instance, LoadOp,
    Operations, RenderPass, RenderPassColorAttachmentDescriptor, RenderPassDescriptor,
    RenderPipeline, ShaderStage, SwapChainError, SwapChainTexture, TextureView,
};

#[derive(Debug)]
pub struct RenderFrame<'a> {
    render_pass: ManuallyDrop<RenderPass<'a>>,
    command_encoder: Box<CommandEncoder>,
    swap_chain_texture: Option<Box<SwapChainTexture>>,
    texture_view: Option<Box<TextureView>>,
}

// TODO: specify clear to color operations.

impl<'a> RenderFrame<'a> {
    pub fn from_canvas<CanvasType: Canvas>(
        instance: &Instance,
        canvas: &mut CanvasType,
    ) -> Result<Self, SwapChainError> {
        let frame = canvas.get_current_frame()?;
        Ok(Self::from_texture_views(instance, Some(frame.output), None))
    }

    pub fn from_texture_views(
        instance: &Instance,
        swap_chain_texture: Option<SwapChainTexture>,
        texture_view: Option<TextureView>,
    ) -> Self {
        let mut command_encoder =
            Box::new(instance.create_command_encoder(&CommandEncoderDescriptor::default()));

        let swap_chain_texture = match swap_chain_texture {
            Some(v) => Some(Box::new(v)),
            None => None,
        };
        let texture_view = match texture_view {
            Some(v) => Some(Box::new(v)),
            None => None,
        };

        // The render pass will hold a reference to the command encoder and the texture views.
        // Since it will store them inside a Box, on the heap, their addresses should be stable.
        // Additional borrowing of the resources is prevented by keeping these resources hidden inside the struct.
        let render_pass = ManuallyDrop::new(unsafe {
            let command_encoder_ref = &mut *(command_encoder.deref_mut() as *mut CommandEncoder);

            let (attachment_ref, resolve_target_ref) = match &texture_view {
                Some(tv) => match &swap_chain_texture {
                    Some(sct) => (
                        &*(tv.deref() as *const TextureView),
                        Some(&*(&sct.view as *const TextureView)),
                    ),
                    None => (&*(tv.deref() as *const TextureView), None),
                },
                None => match &swap_chain_texture {
                    Some(sct) => (&*(&sct.view as *const TextureView), None),
                    None => panic!("No main attachment specified when creating a render frame."),
                },
            };

            command_encoder_ref.begin_render_pass(&RenderPassDescriptor {
                color_attachments: &[RenderPassColorAttachmentDescriptor {
                    attachment: attachment_ref,
                    resolve_target: resolve_target_ref,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            })
        });

        Self {
            render_pass,
            command_encoder,
            swap_chain_texture,
            texture_view,
        }
    }

    pub fn submit(mut self, instance: &Instance) {
        // The render pass must be dropped before the command encoder is finished, because it will add some commands to it during the drop call.
        unsafe { ManuallyDrop::drop(&mut self.render_pass) };
        instance.submit(std::iter::once(self.command_encoder.finish()))
    }

    pub fn render_pass(&self) -> &RenderPass {
        &self.render_pass
    }

    pub fn render_pass_mut(&mut self) -> &'a mut RenderPass {
        &mut self.render_pass
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
