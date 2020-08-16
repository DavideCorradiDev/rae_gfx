use crate::halw;

use std::{cell::RefCell, rc::Rc};

use super::Canvas;

pub trait PipelineConfig {
    type Vertex;
    type Constants;
    type Uniforms;
    fn vertex_shader_module(&self) -> &[u32];
    fn fragment_shader_module(&self) -> &[u32];
}

pub struct Pipeline<Config: PipelineConfig> {
    canvas: Rc<RefCell<dyn Canvas>>,
    layout: halw::PipelineLayout,
    pipeline: halw::GraphicsPipeline,
    _p: std::marker::PhantomData<Config>,
}
