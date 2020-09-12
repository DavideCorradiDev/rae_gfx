use num_traits::identities::Zero;
use std::cmp::PartialOrd;

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Size<T: Zero + PartialOrd> {
    width: T,
    height: T,
}

impl<T: Zero + PartialOrd> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        assert!(
            width >= T::zero() && height >= T::zero(),
            "A negative size is invalid"
        );
        Self { width, height }
    }

    pub fn width(&self) -> &T {
        &self.width
    }

    pub fn set_width(&mut self, value: T) {
        assert!(value >= T::zero(), "A negative size is invalid");
        self.width = value;
    }

    pub fn height(&self) -> &T {
        &self.height
    }

    pub fn set_height(&mut self, value: T) {
        assert!(value >= T::zero(), "A negative size is invalid");
        self.height = value;
    }
}
