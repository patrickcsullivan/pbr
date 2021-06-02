use cgmath::InnerSpace;

/// If the angle between `v1` and `v2` is less than 90 degrees then return `v1`.
/// Otherwise flip and return `v1` so that it is in the same hemisphere as `v2`.
pub fn face_forward<S: cgmath::BaseNum>(
    v1: cgmath::Vector3<S>,
    v2: cgmath::Vector3<S>,
) -> cgmath::Vector3<S> {
    if v1.dot(v2) < S::zero() {
        v1 * (S::zero() - S::one())
    } else {
        v1
    }
}
