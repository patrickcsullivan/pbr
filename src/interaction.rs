use crate::medium;
use crate::shape;
use crate::transform;
use crate::vector;
use crate::vector::face_forward;
use cgmath::InnerSpace;
use cgmath::Transform;

/// Represents the geometry at a particular point on a surface. This point is
/// often found by intersecting a ray against the surface.
///
/// This struct stores the partial derivatives of position with respect to UV
/// coordinates and the partial derivatives of the surface normal with respect
/// to UV coordinates. These partial derivatives implicitly assume that any
/// shape's surface can be defined in terms of a function over U and V. All
/// shapes we can support in the application have at least a local description
/// of this sort.
pub struct SurfaceInteraction<'a> {
    /// The point in space where the interaction occurs.
    point: cgmath::Point3<f32>,

    /// A conservative bound on the error in the value returned by `point()`.
    /// There is generally some floating-point error present when an
    /// interactions point in space is computed from ray intersection, but the
    /// error is (0, 0, 0) for points in participating media.
    point_error_bound: cgmath::Vector3<f32>,

    /// If the nteraction occurs on a surface, this is the surface normal at the
    /// interaction point.
    neg_ray_direction: Option<cgmath::Vector3<f32>>,

    /// The direction of the negative/outgoing ray if the interaction lies along
    /// a ray. Interactions that do not exist on a ray, such as those found by
    /// randomly sampling points on a surface, won't have a negative ray
    /// direction.
    time: std::time::Instant,

    /// The scattering media at the interaction point.
    medium_interface: Option<medium::MediumInterface>,

    /// The shape that the point lies on.
    shape: &'a shape::GenericShape,

    /// The surface normal at the interaction point.
    normal: cgmath::Vector3<f32>,

    /// The UV coordinates of the point on some surface.
    uv: cgmath::Point2<f32>,

    /// The partial derivative of the position with respect to U.
    dpdu: cgmath::Vector3<f32>,

    /// The partial derivative of the position with respect to V.
    dpdv: cgmath::Vector3<f32>,

    /// The partial derivative of the surface normal with respect to U.
    dndu: cgmath::Vector3<f32>,

    /// The partial derivative of the surface normal with respect to V.
    dndv: cgmath::Vector3<f32>,

    /// Contains a second instance of the surface normal and partial
    /// derivatives. These values are initialized to match the originals, but
    /// they may be perturbed (by bump mapping, for example) before they are
    /// used in shading later on.
    shading_geometry: ShadingGeometry,
}

/// Represents geometry that may be used for shading. Contains a normal and
/// partial derivatives that may be perturbed from their original values (by
/// bump mapping, for example).
pub struct ShadingGeometry {
    normal: cgmath::Vector3<f32>,
    dpdu: cgmath::Vector3<f32>,
    dpdv: cgmath::Vector3<f32>,
    dndu: cgmath::Vector3<f32>,
    dndv: cgmath::Vector3<f32>,
}

impl<'a> SurfaceInteraction<'a> {
    pub fn new(
        point: cgmath::Point3<f32>,
        point_error_bound: cgmath::Vector3<f32>,
        neg_ray_direction: Option<cgmath::Vector3<f32>>,
        time: std::time::Instant,

        shape: &'a shape::GenericShape,
        uv: cgmath::Point2<f32>,
        dpdu: cgmath::Vector3<f32>,
        dpdv: cgmath::Vector3<f32>,
        dndu: cgmath::Vector3<f32>,
        dndv: cgmath::Vector3<f32>,
    ) -> Self {
        // Flip the normal if EITHER normals are reveresed or the shape's
        // transform swaps handedness. (See p. 118 for detailed explanation.)
        let normal = if (shape.reverse_orientation && !shape.transform_swaps_handedness)
            || (!shape.reverse_orientation && shape.transform_swaps_handedness)
        {
            dpdu.cross(dpdv).normalize()
        } else {
            dpdu.cross(dpdv).map(|f| -1.0 * f).normalize()
        };

        Self {
            // Generic interaction fields.
            point,
            point_error_bound,
            neg_ray_direction,
            time,
            medium_interface: None,
            // Surface interaction fields.
            shape,
            normal,
            uv,
            dpdu,
            dpdv,
            dndu,
            dndv,
            shading_geometry: ShadingGeometry {
                normal,
                dpdu,
                dpdv,
                dndu,
                dndv,
            },
        }
    }

    pub fn set_shading_geometry(
        &mut self,
        dpdu: cgmath::Vector3<f32>,
        dpdv: cgmath::Vector3<f32>,
        dndu: cgmath::Vector3<f32>,
        dndv: cgmath::Vector3<f32>,
        orientation_is_authoritative: bool,
    ) {
        self.shading_geometry.dpdu = dpdu;
        self.shading_geometry.dpdv = dpdv;
        self.shading_geometry.dndu = dndu;
        self.shading_geometry.dndv = dndv;
        // TODO: Why do we need to handle potential flip in constructor but not
        // here or in `transform`? Maybe in the constructor we can assume the
        // surface normal is authoritative.
        self.shading_geometry.normal = dpdu.cross(dpdv).normalize();

        if orientation_is_authoritative {
            // The shading geometry normal is authoritative, so flip the surface
            // normal, if necessary, so that it is in the same hemisphere as the
            // shading geometry normal.
            self.normal = vector::face_forward(self.normal, self.shading_geometry.normal);
        } else {
            // The surface normal is authoritative, so flip the shading geometry
            // normal, if necessary, so that it is in the same hemisphere as the
            // surface normal.
            self.shading_geometry.normal =
                vector::face_forward(self.shading_geometry.normal, self.normal);
        }
    }
}

impl<'a> transform::Transform<SurfaceInteraction<'a>> for cgmath::Matrix4<f32> {
    fn transform(&self, interaction: SurfaceInteraction<'a>) -> SurfaceInteraction<'a> {
        let mut transformed = SurfaceInteraction {
            point: interaction.point, // FIXME: Handle point transformation in Section 3.9.
            point_error_bound: interaction.point_error_bound, // FIXME: Handle error transformation in Section 3.9.
            neg_ray_direction: interaction
                .neg_ray_direction
                .map(|d| self.transform_vector(d).normalize()),
            time: interaction.time,
            medium_interface: interaction.medium_interface,
            // Surface interaction fields.
            shape: interaction.shape,
            normal: self.transform_vector(interaction.normal).normalize(),
            uv: interaction.uv,
            dpdu: self.transform_vector(interaction.dpdu),
            dpdv: self.transform_vector(interaction.dpdv),
            dndu: self.transform_vector(interaction.dndu),
            dndv: self.transform_vector(interaction.dndv),
            shading_geometry: ShadingGeometry {
                normal: self
                    .transform_vector(interaction.shading_geometry.normal)
                    .normalize(),
                dpdu: self.transform_vector(interaction.shading_geometry.dpdu),
                dpdv: self.transform_vector(interaction.shading_geometry.dpdv),
                dndu: self.transform_vector(interaction.shading_geometry.dndu),
                dndv: self.transform_vector(interaction.shading_geometry.dndv),
            },
        };
        transformed.shading_geometry.normal =
            face_forward(transformed.shading_geometry.normal, transformed.normal);
        transformed
    }
}

trait Interaction {
    /// Returns the point in space where the interaction occurs.
    fn point(&self) -> cgmath::Point3<f32>;

    /// Returns a conservative bound on the error in the value returned by
    /// `point()`. There is generally some floating-point error present when an
    /// interactions point in space is computed from ray intersection, but the
    /// error is (0, 0, 0) for points in participating media.
    fn point_error_bound(&self) -> cgmath::Vector3<f32>;

    /// If the interaction occurs on a surface, this is the surface normal at
    /// the interaction point.
    fn normal(&self) -> Option<cgmath::Vector3<f32>>;

    /// Returns the direction of the negative/outgoing ray if the interaction
    /// lies along a ray. Interactions that do not exist on a ray, such as those
    /// found by randomly sampling points on a surface, won't have a negative
    /// ray direction.
    fn neg_ray_direction(&self) -> Option<cgmath::Vector3<f32>>;

    /// Returns the instance in time when the interaction occurs.
    fn time(&self) -> std::time::Instant;

    /// Returns the scattering media at the interaction point.
    fn medium_interface(&self) -> &Option<medium::MediumInterface>;
}

impl<'a> Interaction for SurfaceInteraction<'a> {
    fn point(&self) -> cgmath::Point3<f32> {
        self.point
    }

    fn point_error_bound(&self) -> cgmath::Vector3<f32> {
        self.point_error_bound
    }

    fn normal(&self) -> Option<cgmath::Vector3<f32>> {
        Some(self.normal)
    }

    fn neg_ray_direction(&self) -> Option<cgmath::Vector3<f32>> {
        self.neg_ray_direction
    }

    fn time(&self) -> std::time::Instant {
        self.time
    }

    fn medium_interface(&self) -> &Option<medium::MediumInterface> {
        &self.medium_interface
    }
}
