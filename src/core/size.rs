#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

impl<T> From<rae_math::matrix::Vector2<T>> for Size<T> {
    fn from(vec: rae_math::matrix::Vector2<T>) -> Self {
        Self {
            width: vec.x(),
            height: vec.y(),
        }
    }
}
