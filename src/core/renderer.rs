use crate::halw;

pub trait Renderable {
    fn stride() -> u32;
    fn render(&self, command_buffer: &mut halw::CommandBuffer);
}

#[derive(Debug, PartialEq, Clone)]
pub enum RenderingError {
    NotProcessingFrame,
}

impl std::fmt::Display for RenderingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderingError::NotProcessingFrame => write!(f, "No frame is being processed"),
        }
    }
}

impl std::error::Error for RenderingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
