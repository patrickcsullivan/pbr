use cgmath::SquareMatrix;

/// A trait representing an affine transformation that can be applied to data
/// structures containing points or vectors.
pub trait Transform<T> {
    fn transform(&self, t: &T) -> T;
}

/// A trait that allows an affine transformation to indicate whether it swaps
/// the coordinate system handedness.
pub trait SwapHandedness {
    /// Returns a Boolean indicating whether the transform swaps the coordinate
    /// system handedness.
    fn swaps_handedness(&self) -> bool;
}

impl SwapHandedness for cgmath::Matrix4<f32> {
    fn swaps_handedness(&self) -> bool {
        let m3 = cgmath::Matrix3::new(
            self[0][0], self[0][1], self[0][2], self[1][0], self[1][1], self[1][2], self[2][0],
            self[2][1], self[2][2],
        );
        m3.determinant() < 0.0
    }
}
