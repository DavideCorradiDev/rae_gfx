use std::iter;

use rae_app::{
    application::Application,
    event::{ControlFlow, EventHandler, EventLoop},
    window,
    window::{WindowBuilder, WindowId},
};

use rae_math::{
    conversion::convert,
    geometry2::{OrthographicProjection, Projective},
};

use rae_gfx::{
    core::{
        AddressMode, Canvas, CanvasWindow, CanvasWindowDescriptor, Color, CommandSequence,
        FilterMode, Instance, InstanceCreationError, InstanceDescriptor, RenderPassOperations,
        SampleCount, Sampler, SamplerDescriptor, SwapChainError, Texture, TextureView,
        TextureViewDescriptor,
    },
    sprite,
    sprite::{MeshTemplates as SpriteMeshTemplates, Renderer as SpriteRenderer},
};

#[derive(Debug)]
struct Sprite {
    uniform_constants: sprite::UniformConstants,
    mesh: sprite::Mesh,
}

#[derive(Debug)]
struct ApplicationImpl {
    window: CanvasWindow,
    instance: Instance,
    pipeline: sprite::RenderPipeline,
    projection_transform: Projective<f32>,
    sprites: Vec<Sprite>,
}

impl ApplicationImpl {
    const SAMPLE_COUNT: SampleCount = 8;

    fn create_sprites(instance: &Instance) -> Vec<Sprite> {
        let image = image::open("examples/data/gioconda.jpg")
            .expect("Failed to load texture image")
            .into_rgba();
        let sprite_texture =
            Texture::from_image(instance, &image).create_view(&TextureViewDescriptor::default());

        vec![
            Sprite {
                uniform_constants: sprite::UniformConstants::new(
                    instance,
                    &sprite_texture,
                    &Sampler::new(&instance, &SamplerDescriptor::default()),
                ),
                mesh: sprite::Mesh::quad(
                    instance,
                    &sprite::Vertex::new([0., 0.], [0., 0.]),
                    &sprite::Vertex::new([400., 400.], [1., 1.]),
                ),
            },
            Sprite {
                uniform_constants: sprite::UniformConstants::new(
                    instance,
                    &sprite_texture,
                    &Sampler::new(
                        &instance,
                        &SamplerDescriptor {
                            mag_filter: FilterMode::Nearest,
                            min_filter: FilterMode::Linear,
                            mipmap_filter: FilterMode::Nearest,
                            ..SamplerDescriptor::default()
                        },
                    ),
                ),
                mesh: sprite::Mesh::quad(
                    instance,
                    &sprite::Vertex::new([400., 0.], [0., 0.]),
                    &sprite::Vertex::new([600., 200.], [0.5, 0.5]),
                ),
            },
            Sprite {
                uniform_constants: sprite::UniformConstants::new(
                    instance,
                    &sprite_texture,
                    &Sampler::new(
                        &instance,
                        &SamplerDescriptor {
                            mag_filter: FilterMode::Linear,
                            min_filter: FilterMode::Linear,
                            mipmap_filter: FilterMode::Linear,
                            ..SamplerDescriptor::default()
                        },
                    ),
                ),
                mesh: sprite::Mesh::quad(
                    instance,
                    &sprite::Vertex::new([600., 0.], [0.5, 0.]),
                    &sprite::Vertex::new([800., 200.], [1., 0.5]),
                ),
            },
            Sprite {
                uniform_constants: sprite::UniformConstants::new(
                    instance,
                    &sprite_texture,
                    &Sampler::new(
                        &instance,
                        &SamplerDescriptor {
                            mag_filter: FilterMode::Linear,
                            min_filter: FilterMode::Linear,
                            mipmap_filter: FilterMode::Linear,
                            ..SamplerDescriptor::default()
                        },
                    ),
                ),
                mesh: sprite::Mesh::quad(
                    instance,
                    &sprite::Vertex::new([400., 200.], [0., 0.5]),
                    &sprite::Vertex::new([600., 400.], [0.5, 1.]),
                ),
            },
            Sprite {
                uniform_constants: sprite::UniformConstants::new(
                    instance,
                    &sprite_texture,
                    &Sampler::new(
                        &instance,
                        &SamplerDescriptor {
                            mag_filter: FilterMode::Nearest,
                            min_filter: FilterMode::Linear,
                            mipmap_filter: FilterMode::Nearest,
                            ..SamplerDescriptor::default()
                        },
                    ),
                ),
                mesh: sprite::Mesh::quad(
                    instance,
                    &sprite::Vertex::new([600., 200.], [0.5, 0.5]),
                    &sprite::Vertex::new([800., 400.], [1., 1.]),
                ),
            },
            Sprite {
                uniform_constants: sprite::UniformConstants::new(
                    instance,
                    &sprite_texture,
                    &Sampler::new(
                        &instance,
                        &SamplerDescriptor {
                            address_mode_u: AddressMode::Repeat,
                            address_mode_v: AddressMode::ClampToEdge,
                            ..SamplerDescriptor::default()
                        },
                    ),
                ),
                mesh: sprite::Mesh::quad(
                    instance,
                    &sprite::Vertex::new([000., 400.], [-0.5, -0.5]),
                    &sprite::Vertex::new([400., 800.], [1.5, 1.5]),
                ),
            },
            Sprite {
                uniform_constants: sprite::UniformConstants::new(
                    instance,
                    &sprite_texture,
                    &Sampler::new(
                        &instance,
                        &SamplerDescriptor {
                            address_mode_u: AddressMode::MirrorRepeat,
                            address_mode_v: AddressMode::ClampToEdge,
                            ..SamplerDescriptor::default()
                        },
                    ),
                ),
                mesh: sprite::Mesh::quad(
                    instance,
                    &sprite::Vertex::new([400., 400.], [-0.5, -0.5]),
                    &sprite::Vertex::new([800., 800.], [1.5, 1.5]),
                ),
            },
        ]
    }
}

impl EventHandler<ApplicationError, ApplicationEvent> for ApplicationImpl {
    type Error = ApplicationError;
    type CustomEvent = ApplicationEvent;

    fn new(event_loop: &EventLoop<Self::CustomEvent>) -> Result<Self, Self::Error> {
        let window = WindowBuilder::new()
            .with_inner_size(window::Size::Physical(window::PhysicalSize {
                width: 800,
                height: 800,
            }))
            .build(event_loop)?;
        let (window, instance) = unsafe {
            let (instance, surface) = Instance::new_with_compatible_window(
                &InstanceDescriptor::high_performance(),
                &window,
            )?;
            let window = CanvasWindow::from_window_and_surface(
                &instance,
                window,
                surface,
                &CanvasWindowDescriptor {
                    sample_count: Self::SAMPLE_COUNT,
                    ..CanvasWindowDescriptor::default()
                },
            );
            (window, instance)
        };

        let pipeline = sprite::RenderPipeline::new(
            &instance,
            &sprite::RenderPipelineDescriptor {
                sample_count: Self::SAMPLE_COUNT,
                ..sprite::RenderPipelineDescriptor::default()
            },
        );

        let window_size = window.inner_size();

        // This matrix will flip the y axis, so that screen coordinates follow mouse
        // coordinates.
        let projection_transform = OrthographicProjection::new(
            0.,
            window_size.width as f32,
            window_size.height as f32,
            0.,
        )
        .to_projective();

        let sprites = Self::create_sprites(&instance);

        Ok(Self {
            window,
            instance,
            pipeline,
            projection_transform,
            sprites,
        })
    }

    fn on_resized(
        &mut self,
        wid: WindowId,
        size: window::PhysicalSize<u32>,
    ) -> Result<ControlFlow, Self::Error> {
        if wid == self.window.id() {
            self.window.update_buffers(&self.instance);
            self.projection_transform = OrthographicProjection::new(
                0.,
                1f32.max(size.width as f32),
                1f32.max(size.height as f32),
                0.,
            )
            .to_projective();
        }
        Ok(ControlFlow::Continue)
    }

    fn on_variable_update(&mut self, _dt: std::time::Duration) -> Result<ControlFlow, Self::Error> {
        let push_constants =
            sprite::PushConstants::new(&convert(self.projection_transform), Color::WHITE);

        let frame = self.window.current_frame()?;
        let mut cmd_sequence = CommandSequence::new(&self.instance);
        {
            let mut rpass = cmd_sequence.begin_render_pass(
                &frame,
                &self.pipeline.render_pass_requirements(),
                &RenderPassOperations::default(),
            );
            for sprite in &self.sprites {
                rpass.draw_sprite(
                    &self.pipeline,
                    iter::once(sprite::DrawCommandDescriptor {
                        uniform_constants: &sprite.uniform_constants,
                        draw_mesh_commands: iter::once(sprite::DrawMeshCommandDescriptor {
                            mesh: &sprite.mesh,
                            index_range: 0..sprite.mesh.index_count(),
                            push_constants: &push_constants,
                        }),
                    }),
                );
            }
        }
        cmd_sequence.submit(&self.instance);
        Ok(ControlFlow::Continue)
    }
}

type ApplicationEvent = ();

#[derive(Debug)]
enum ApplicationError {
    WindowCreationFailed(window::OsError),
    InstanceCreationFailed(InstanceCreationError),
    RenderFrameCreationFailed(SwapChainError),
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::WindowCreationFailed(e) => {
                write!(f, "Window creation failed ({})", e)
            }
            ApplicationError::InstanceCreationFailed(e) => {
                write!(f, "Instance creation failed ({})", e)
            }
            ApplicationError::RenderFrameCreationFailed(e) => {
                write!(f, "Render frame creation failed ({})", e)
            }
        }
    }
}

impl std::error::Error for ApplicationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ApplicationError::WindowCreationFailed(e) => Some(e),
            ApplicationError::InstanceCreationFailed(e) => Some(e),
            ApplicationError::RenderFrameCreationFailed(e) => Some(e),
        }
    }
}

impl From<window::OsError> for ApplicationError {
    fn from(e: window::OsError) -> Self {
        ApplicationError::WindowCreationFailed(e)
    }
}

impl From<InstanceCreationError> for ApplicationError {
    fn from(e: InstanceCreationError) -> Self {
        ApplicationError::InstanceCreationFailed(e)
    }
}

impl From<SwapChainError> for ApplicationError {
    fn from(e: SwapChainError) -> Self {
        ApplicationError::RenderFrameCreationFailed(e)
    }
}

fn main() {
    const FIXED_FRAMERATE: u64 = 30;
    const VARIABLE_FRAMERATE_CAP: u64 = 60;
    Application::<ApplicationImpl, _, _>::new(FIXED_FRAMERATE, Some(VARIABLE_FRAMERATE_CAP)).run();
}
