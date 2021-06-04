use crate::bounding_box;
use crate::interaction;
use crate::ray;
use crate::transform::Transform;

/// Describes the geometric properties of a primitive and provides a ray
/// intersection function.
trait Shape<'a> {
    /// Returns a reference to the matrix that transforms the shape from object
    /// space to world space.
    fn transform() -> &'a cgmath::Matrix4<f32>;

    /// Returns a reference to the inverse of the object-to-world transform
    /// matrix.
    fn inverse_transform() -> &'a cgmath::Matrix4<f32>;

    /// Returns a flag that indicates whether the shape's object-to-world
    /// transform matrix swaps the handedness of the object coordinate system.
    fn transform_swaps_handedness() -> bool;

    /// Returns a flag that indicates whether the shape's normals should be
    /// flipped from their original directions in order to point to the outside
    /// of the shape.
    fn reverse_orientation() -> bool;

    /// Returns an axis-aligned bounding box in the shape's object space.
    fn object_bound() -> bounding_box::Bounds3<f32>;

    /// Returns an axis-aligned bounding box in world space.
    fn world_bound() -> bounding_box::Bounds3<f32>;

    /// Returns information about the first ray-shape intersection, if any, in
    /// the (0, `ray.t_max`) parametric range along the ray.
    ///
    /// `ray` is in world space, and the returned surface interaction is in
    /// world space.
    fn ray_intersection(
        &self,
        ray: &ray::Ray,
        test_alpha_texture: bool,
    ) -> Option<(f32, interaction::SurfaceInteraction)>;

    /// Returns a boolean indicating whether the ray intersects the shape.
    fn does_ray_intersect(&self, ray: &ray::Ray, test_alpha_texture: bool) -> bool;

    /// Returns the surface area of the shape.
    fn surface_area(&self) -> f32;
}

// TODO: Remove and replace uses of GenericShape with Shape trait objects.
pub struct GenericShape {
    /// Flag that indicates whether the shape's normals should be flipped from
    /// their original directions so that they point to the outside of the
    /// shape.
    pub reverse_orientation: bool,

    /// Flag that indicates whether the shape's `transform` swaps the handedness
    /// of the object coordinate system for the shape.
    pub transform_swaps_handedness: bool,
}

// This naiive world space bounding box calculation can be used by default, but
// shapes that can easily calculate a tighter world space bounding box should
// implement a custom function.
fn naiive_world_bound(
    bounds: &bounding_box::Bounds3<f32>,
    transform: &cgmath::Matrix4<f32>,
) -> bounding_box::Bounds3<f32> {
    transform.transform(*bounds)
}
