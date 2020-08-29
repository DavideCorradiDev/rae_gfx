use rae_math::geometry2;

// TODO serialization for Vertex
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vertex {
    pub position: geometry2::Point<f32>,
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: geometry2::Point::from([x, y]),
        }
    }
}
