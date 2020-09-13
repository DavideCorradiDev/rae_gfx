use super::{
    ColorBufferFormat, DepthStencilBufferFormat, Extent3d, Instance, PresentMode, SampleCount,
    Size, Surface, SwapChain, SwapChainDescriptor, SwapChainError, SwapChainFrame, Texture,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsage, TextureView,
    TextureViewDescriptor,
};

pub type CanvasSize = Size<u32>;

#[derive(Debug)]
pub struct CanvasSwapChainRef<'a> {
    format: ColorBufferFormat,
    sample_count: SampleCount,
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

    pub fn frame(&self) -> &SwapChainFrame {
        &self.frame
    }

    pub fn multisampled_buffer(&self) -> Option<&TextureView> {
        self.multisampled_buffer
    }

    pub fn format(&self) -> ColorBufferFormat {
        self.format
    }

    pub fn sample_count(&self) -> SampleCount {
        self.sample_count
    }
}

#[derive(Debug)]
pub struct CanvasSwapChainDescriptor {
    pub size: Size<u32>,
    pub format: ColorBufferFormat,
    pub sample_count: SampleCount,
}

#[derive(Debug)]
pub struct CanvasSwapChain {
    format: ColorBufferFormat,
    sample_count: SampleCount,
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
            format: desc.format,
            sample_count: desc.sample_count,
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
            format: self.format,
            sample_count: self.sample_count,
            multisampled_buffer,
            frame,
        })
    }
}

#[derive(Debug)]
pub struct CanvasColorBufferRef<'a> {
    format: ColorBufferFormat,
    sample_count: SampleCount,
    multisampled_buffer: Option<&'a TextureView>,
    main_buffer: &'a TextureView,
}

impl<'a> CanvasColorBufferRef<'a> {
    pub fn attachment(&self) -> &TextureView {
        match self.multisampled_buffer {
            Some(v) => &v,
            None => &self.main_buffer,
        }
    }

    pub fn resolve_target(&self) -> Option<&TextureView> {
        match self.multisampled_buffer {
            Some(_) => Some(&self.main_buffer),
            None => None,
        }
    }

    pub fn main_buffer(&self) -> &TextureView {
        self.main_buffer
    }

    pub fn multisampled_buffer(&self) -> Option<&TextureView> {
        self.multisampled_buffer
    }

    pub fn format(&self) -> ColorBufferFormat {
        self.format
    }

    pub fn sample_count(&self) -> SampleCount {
        self.sample_count
    }
}

#[derive(Debug)]
pub struct CanvasColorBufferDescriptor {
    pub size: Size<u32>,
    pub format: ColorBufferFormat,
    pub sample_count: SampleCount,
}

#[derive(Debug)]
pub struct CanvasColorBuffer {
    format: ColorBufferFormat,
    sample_count: SampleCount,
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
            format: desc.format,
            sample_count: desc.sample_count,
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
            format: self.format,
            sample_count: self.sample_count,
            multisampled_buffer,
            main_buffer: &self.main_buffer,
        }
    }
}

#[derive(Debug)]
pub struct CanvasDepthStencilBufferRef<'a> {
    pub format: DepthStencilBufferFormat,
    pub sample_count: SampleCount,
    pub buffer: &'a TextureView,
}

#[derive(Debug)]
pub struct CanvasFrame<'a> {
    pub swap_chain: Option<CanvasSwapChainRef<'a>>,
    pub color_buffers: Vec<CanvasColorBufferRef<'a>>,
    pub depth_stencil_buffer: Option<CanvasDepthStencilBufferRef<'a>>,
}

pub trait Canvas {
    fn current_frame(&mut self) -> Result<CanvasFrame, SwapChainError>;
}
