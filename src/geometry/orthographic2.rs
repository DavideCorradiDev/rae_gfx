extern crate nalgebra;
use nalgebra::base::Matrix3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Orthographic2<N: nalgebra::RealField> {
    matrix: Matrix3<N>,
}

impl<N: nalgebra::RealField> Orthographic2<N> {
    pub fn new(left: N, right: N, bottom: N, top: N) -> Self {
        let matrix = Matrix3::<N>::identity();
        let mut res = Self::from_matrix_unchecked(matrix);
        res.set_left_and_right(left, right);
        res.set_bottom_and_top(bottom, top);
        res
    }

    pub fn from_matrix_unchecked(matrix: Matrix3<N>) -> Self {
        Self { matrix }
    }

    pub fn set_left_and_right(&mut self, left: N, right: N) {
        assert!(
            left != right,
            "The left corner must not be equal to the right corner."
        );
        self.matrix[(0, 0)] = nalgebra::convert::<_, N>(2.0) / (right - left);
        self.matrix[(0, 2)] = -(right + left) / (right - left);
    }

    pub fn set_bottom_and_top(&mut self, bottom: N, top: N) {
        assert!(
            bottom != top,
            "The top corner must not be equal to the bottom corner."
        );
        self.matrix[(1, 1)] = nalgebra::convert::<_, N>(2.0) / (top - bottom);
        self.matrix[(1, 2)] = -(top + bottom) / (top - bottom);
    }

    pub fn to_homogeneous(&self) -> Matrix3<N> {
        self.matrix
    }
}
