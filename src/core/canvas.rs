use super::{
    ColorBufferFormat, DepthStencilBufferFormat, Extent3d, Instance, PresentMode, SampleCount,
    Size, Surface, SwapChain, SwapChainDescriptor, SwapChainError, SwapChainFrame, Texture,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsage, TextureView,
    TextureViewDescriptor,
};

pub type CanvasSize = Size<u32>;

#[derive(Debug)]
pub struct CanvasSwapChainRef<'a> {
    sample_count: SampleCount,
    format: ColorBufferFormat,
    multisampled_buffer: Option<&'a TextureView>,
    frame: SwapChainFrame,
}

impl<'a> CanvasSwapChainRef<'a> {
    pub fn attachment(&self) -> &TextureView {
        match self.multisampled_buffer {
            Some(v) => &v,
            None => &self.frame.output.view,
        }
    }

    pub fn resolve_target(&self) -> Option<&TextureView> {
        match self.multisampled_buffer {
            Some(_) => Some(&self.frame.output.view),
            None => None,
        }
    }

    pub fn format(&self) -> ColorBufferFormat {
        self.format
    }

    pub fn sample_count(&self) -> SampleCount {
        self.sample_count
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CanvasSwapChainDescriptor {
    pub size: CanvasSize,
    pub sample_count: SampleCount,
    pub format: ColorBufferFormat,
}

#[derive(Debug)]
pub struct CanvasSwapChain {
    sample_count: SampleCount,
    format: ColorBufferFormat,
    multisampled_buffer: Option<TextureView>,
    swap_chain: SwapChain,
}

impl CanvasSwapChain {
    pub fn new(instance: &Instance, surface: &Surface, desc: &CanvasSwapChainDescriptor) -> Self {
        let usage = TextureUsage::OUTPUT_ATTACHMENT;
        let texture_format = TextureFormat::from(desc.format);
        let width = desc.size.width();
        let height = desc.size.height();
        let swap_chain = SwapChain::new(
            instance,
            surface,
            &SwapChainDescriptor {
                usage,
                format: texture_format,
                width,
                height,
                present_mode: PresentMode::Mailbox,
            },
        );
        let multisampled_buffer = if desc.sample_count > 1 {
            let multisampling_buffer_texture = Texture::new(
                instance,
                &TextureDescriptor {
                    size: Extent3d {
                        width,
                        height,
                        depth: 1,
                    },
                    mip_level_count: 1,
                    sample_count: desc.sample_count,
                    dimension: TextureDimension::D2,
                    format: texture_format,
                    usage,
                    label: None,
                },
            );
            Some(multisampling_buffer_texture.create_view(&TextureViewDescriptor::default()))
        } else {
            None
        };
        Self {
            sample_count: desc.sample_count,
            format: desc.format,
            multisampled_buffer,
            swap_chain,
        }
    }

    pub fn format(&self) -> ColorBufferFormat {
        self.format
    }

    pub fn sample_count(&self) -> SampleCount {
        self.sample_count
    }

    pub fn reference(&mut self) -> Result<CanvasSwapChainRef, SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?;
        let multisampled_buffer = match self.multisampled_buffer {
            Some(ref v) => Some(v),
            None => None,
        };
        Ok(CanvasSwapChainRef {
            sample_count: self.sample_count,
            format: self.format,
            multisampled_buffer,
            frame,
        })
    }
}

#[derive(Debug)]
pub struct CanvasColorBufferRef<'a> {
    sample_count: SampleCount,
    format: ColorBufferFormat,
    multisampled_buffer: Option<&'a TextureView>,
    main_buffer: &'a TextureView,
}

impl<'a> CanvasColorBufferRef<'a> {
    pub fn attachment(&self) -> &TextureView {
        match self.multisampled_buffer {
            Some(v) => v,
            None => self.main_buffer,
        }
    }

    pub fn resolve_target(&self) -> Option<&TextureView> {
        match self.multisampled_buffer {
            Some(_) => Some(self.main_buffer),
            None => None,
        }
    }

    pub fn format(&self) -> ColorBufferFormat {
        self.format
    }

    pub fn sample_count(&self) -> SampleCount {
        self.sample_count
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CanvasColorBufferDescriptor {
    pub size: CanvasSize,
    pub sample_count: SampleCount,
    pub format: ColorBufferFormat,
}

#[derive(Debug)]
pub struct CanvasColorBuffer {
    sample_count: SampleCount,
    format: ColorBufferFormat,
    multisampled_buffer: Option<TextureView>,
    main_buffer: TextureView,
}

impl CanvasColorBuffer {
    pub fn new(instance: &Instance, desc: &CanvasColorBufferDescriptor) -> Self {
        let mut tex_desc = TextureDescriptor {
            size: Extent3d {
                width: desc.size.width(),
                height: desc.size.height(),
                depth: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::from(desc.format),
            usage: TextureUsage::SAMPLED | TextureUsage::OUTPUT_ATTACHMENT,
            label: None,
        };

        let main_buffer =
            Texture::new(instance, &tex_desc).create_view(&TextureViewDescriptor::default());

        let multisampled_buffer = if desc.sample_count > 1 {
            tex_desc.sample_count = desc.sample_count;
            Some(Texture::new(instance, &tex_desc).create_view(&TextureViewDescriptor::default()))
        } else {
            None
        };

        Self {
            sample_count: desc.sample_count,
            format: desc.format,
            multisampled_buffer,
            main_buffer,
        }
    }

    pub fn texture_view(&self) -> &TextureView {
        &self.main_buffer
    }

    pub fn format(&self) -> ColorBufferFormat {
        self.format
    }

    pub fn sample_count(&self) -> SampleCount {
        self.sample_count
    }

    pub fn reference(&self) -> CanvasColorBufferRef {
        let multisampled_buffer = match self.multisampled_buffer {
            Some(ref v) => Some(v),
            None => None,
        };
        CanvasColorBufferRef {
            sample_count: self.sample_count,
            format: self.format,
            multisampled_buffer,
            main_buffer: &self.main_buffer,
        }
    }
}

#[derive(Debug)]
pub struct CanvasDepthStencilBufferRef<'a> {
    sample_count: SampleCount,
    format: DepthStencilBufferFormat,
    buffer: &'a TextureView,
}

impl<'a> CanvasDepthStencilBufferRef<'a> {
    pub fn attachment(&self) -> &TextureView {
        self.buffer
    }

    pub fn format(&self) -> DepthStencilBufferFormat {
        self.format
    }

    pub fn sample_count(&self) -> SampleCount {
        self.sample_count
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CanvasDepthStencilBufferDescriptor {
    pub size: CanvasSize,
    pub sample_count: SampleCount,
    pub format: DepthStencilBufferFormat,
}

#[derive(Debug)]
pub struct CanvasDepthStencilBuffer {
    sample_count: SampleCount,
    format: DepthStencilBufferFormat,
    buffer: TextureView,
}

impl CanvasDepthStencilBuffer {
    pub fn new(instance: &Instance, desc: &CanvasDepthStencilBufferDescriptor) -> Self {
        let buffer = Texture::new(
            instance,
            &TextureDescriptor {
                size: Extent3d {
                    width: desc.size.width(),
                    height: desc.size.height(),
                    depth: 1,
                },
                mip_level_count: 1,
                sample_count: desc.sample_count,
                dimension: TextureDimension::D2,
                format: TextureFormat::from(desc.format),
                usage: TextureUsage::SAMPLED | TextureUsage::OUTPUT_ATTACHMENT,
                label: None,
            },
        )
        .create_view(&TextureViewDescriptor::default());
        Self {
            format: desc.format,
            sample_count: desc.sample_count,
            buffer,
        }
    }

    pub fn texture_view(&self) -> &TextureView {
        &self.buffer
    }

    pub fn format(&self) -> DepthStencilBufferFormat {
        self.format
    }

    pub fn sample_count(&self) -> SampleCount {
        self.sample_count
    }

    pub fn reference(&self) -> CanvasDepthStencilBufferRef {
        CanvasDepthStencilBufferRef {
            sample_count: self.sample_count,
            format: self.format,
            buffer: &self.buffer,
        }
    }
}

#[derive(Debug)]
pub struct CanvasFrame<'a> {
    pub swap_chain: Option<CanvasSwapChainRef<'a>>,
    pub color_buffers: Vec<CanvasColorBufferRef<'a>>,
    pub depth_stencil_buffer: Option<CanvasDepthStencilBufferRef<'a>>,
}

#[derive(Debug, Clone)]
pub struct CanvasBufferSwapChainDescriptor<'a> {
    pub surface: &'a Surface,
    pub format: ColorBufferFormat,
}

#[derive(Debug, Clone)]
pub struct CanvasBufferDescriptor<'a> {
    pub size: CanvasSize,
    pub sample_count: SampleCount,
    pub swap_chain_descriptor: Option<CanvasBufferSwapChainDescriptor<'a>>,
    pub color_buffer_formats: Vec<ColorBufferFormat>,
    pub depth_stencil_buffer_format: Option<DepthStencilBufferFormat>,
}

#[derive(Debug)]
pub struct CanvasBuffer {
    size: CanvasSize,
    sample_count: SampleCount,
    swap_chain: Option<CanvasSwapChain>,
    color_buffers: Vec<CanvasColorBuffer>,
    depth_stencil_buffer: Option<CanvasDepthStencilBuffer>,
}

impl CanvasBuffer {
    pub fn new(instance: &Instance, desc: &CanvasBufferDescriptor) -> Self {
        let swap_chain = match &desc.swap_chain_descriptor {
            Some(sc_desc) => Some(CanvasSwapChain::new(
                instance,
                sc_desc.surface,
                &CanvasSwapChainDescriptor {
                    size: desc.size,
                    sample_count: desc.sample_count,
                    format: sc_desc.format,
                },
            )),
            None => None,
        };

        let mut color_buffers = Vec::with_capacity(desc.color_buffer_formats.len());
        for format in desc.color_buffer_formats.iter() {
            color_buffers.push(CanvasColorBuffer::new(
                instance,
                &CanvasColorBufferDescriptor {
                    size: desc.size,
                    sample_count: desc.sample_count,
                    format: *format,
                },
            ));
        }

        let depth_stencil_buffer = match &desc.depth_stencil_buffer_format {
            Some(format) => Some(CanvasDepthStencilBuffer::new(
                instance,
                &CanvasDepthStencilBufferDescriptor {
                    size: desc.size,
                    sample_count: desc.sample_count,
                    format: *format,
                },
            )),
            None => None,
        };

        assert!(
            swap_chain.is_some() || !color_buffers.is_empty() || depth_stencil_buffer.is_some(),
            "No buffer defined for a canvas buffer"
        );

        Self {
            size: desc.size,
            sample_count: desc.sample_count,
            swap_chain,
            color_buffers,
            depth_stencil_buffer,
        }
    }

    pub fn canvas_size(&self) -> &CanvasSize {
        &self.size
    }

    pub fn sample_count(&self) -> SampleCount {
        self.sample_count
    }

    pub fn swap_chain(&self) -> Option<&CanvasSwapChain> {
        self.swap_chain.as_ref()
    }

    pub fn color_buffers(&self) -> &Vec<CanvasColorBuffer> {
        &self.color_buffers
    }

    pub fn depth_stencil_buffer(&self) -> Option<&CanvasDepthStencilBuffer> {
        self.depth_stencil_buffer.as_ref()
    }

    pub fn current_frame(&mut self) -> Result<CanvasFrame, SwapChainError> {
        let swap_chain = match &mut self.swap_chain {
            Some(swap_chain) => Some(swap_chain.reference()?),
            None => None,
        };

        let mut color_buffers = Vec::with_capacity(self.color_buffers.len());
        for color_buffer in self.color_buffers.iter() {
            color_buffers.push(color_buffer.reference());
        }

        let depth_stencil_buffer = match &self.depth_stencil_buffer {
            Some(depth_stencil_buffer) => Some(depth_stencil_buffer.reference()),
            None => None,
        };

        Ok(CanvasFrame {
            swap_chain,
            color_buffers,
            depth_stencil_buffer,
        })
    }
}

pub trait Canvas {
    fn current_frame(&mut self) -> Result<CanvasFrame, SwapChainError>;
    fn canvas_size(&self) -> &CanvasSize;
    fn sample_count(&self) -> SampleCount;
}
