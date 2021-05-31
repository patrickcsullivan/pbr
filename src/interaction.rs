use crate::medium;

// trait Interaction {}

pub struct Interaction {
    /// Point in space where the interaction occurs.
    point: cgmath::Point3<f32>,

    /// Conservative bound on the error present in `point`. There is generally
    /// some floating-point error present in `point` when the point is computed
    /// from ray intersection, but it is (0, 0, 0) for points in participating
    /// media.
    point_error_bound: cgmath::Vector3<f32>,

    /// If the interaction occurs on a surface, this is the surface normal at
    /// the interaction point.
    normal: Option<cgmath::Vector3<f32>>,

    /// The direction of a negative/outgoing ray. Only interactions that lie
    /// along a ray will have a negative ray direction. Interaction that do not
    /// exist on a ray, such as those found by randomly sampling points on a
    /// surface, won't have a negative ray direction.
    neg_ray_direction: Option<cgmath::Vector3<f32>>,

    /// Instance in time when the interaction occurs.
    time: std::time::Instant,

    /// The scattering media at the interaction point.
    medium_interface: Option<medium::MediumInterface>,
}

impl Interaction {
    pub fn new(
        point: cgmath::Point3<f32>,
        point_error_bound: cgmath::Vector3<f32>,
        normal: Option<cgmath::Vector3<f32>>,
        neg_ray_direction: Option<cgmath::Vector3<f32>>,
        time: std::time::Instant,
        medium_interface: Option<medium::MediumInterface>,
    ) -> Self {
        Self {
            point,
            point_error_bound,
            normal,
            neg_ray_direction,
            time,
            medium_interface,
        }
    }
}

/// Represents the geometry of a particular point on a surface. This point is
/// often found by intersecting a ray against the surface.
pub struct SurfaceInteraction {}
