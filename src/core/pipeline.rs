use crate::halw;

pub trait PipelineConfig {
    type Vertex;
    type Constants;
    type Uniforms;
    fn vertex_shader_module(&self) -> &[u32];
    fn fragment_shader_module(&self) -> &[u32];
}

pub struct Pipeline<Config: PipelineConfig> {}
