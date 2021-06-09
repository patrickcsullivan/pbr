use super::Shape;
use crate::bounding_box;
use crate::ray::Ray;
use crate::transform::SwapHandedness;
use crate::transform::Transform;

/// A sphere centered at the origin in object space.
pub struct Sphere<'a> {
    // Generic shape fields
    object_to_world: &'a cgmath::Matrix4<f32>,
    world_to_object: &'a cgmath::Matrix4<f32>,
    object_to_world_swaps_handedness: bool,
    reverse_orientation: bool,

    // Sphere-specific fields
    radius: f32,
    z_min: f32,
    z_max: f32,
    theta_min: f32,
    theta_max: f32,
    phi_max: f32,
}

impl<'a> Sphere<'a> {
    pub fn new(
        object_to_world: &'a cgmath::Matrix4<f32>,
        world_to_object: &'a cgmath::Matrix4<f32>,
        reverse_orientation: bool,
        radius: f32,
        z_min: f32,
        z_max: f32,
        phi_max: f32,
    ) -> Self {
        Self {
            // Generic shape fields
            object_to_world,
            world_to_object,
            object_to_world_swaps_handedness: object_to_world.swaps_handedness(),
            reverse_orientation,
            // Sphere-specific fields
            radius,
            z_min,
            z_max,
            theta_min: 0.0,
            theta_max: 0.0,
            phi_max: 0.0,
        }
    }
}

impl<'a> Shape<'a> for Sphere<'a> {
    fn object_to_world(&self) -> &'a cgmath::Matrix4<f32> {
        self.object_to_world
    }

    fn world_to_object(&self) -> &'a cgmath::Matrix4<f32> {
        self.world_to_object
    }

    fn object_to_world_swaps_handedness(&self) -> bool {
        self.object_to_world_swaps_handedness
    }

    fn reverse_orientation(&self) -> bool {
        self.reverse_orientation
    }

    fn object_bound(&self) -> bounding_box::Bounds3<f32> {
        bounding_box::Bounds3::from_corners(
            cgmath::Point3::new(-1.0 * self.radius, -1.0 * self.radius, self.z_min),
            cgmath::Point3::new(self.radius, self.radius, self.z_max),
        )
    }

    fn world_bound(&self) -> bounding_box::Bounds3<f32> {
        self.object_to_world().transform(&self.object_bound())
    }

    fn ray_intersection(
        &self,
        ray: &crate::ray::Ray,
        test_alpha_texture: bool,
    ) -> Option<(f32, crate::interaction::SurfaceInteraction)> {
        let ray = self.object_to_world.transform(ray);
        // let (o_err, d_err) = ...from transform...
        // TODO: Compute quatratic sphere coordinates.
        // TODO: Solve quadratic equation for t values.
        // TODO: Compute sphere hit position and phi.
        // TODO: Test sphere intersection against clipping parameters.
        // TODO: Find parametric representation of sphere hit.
        // TODO: Compute error bounds for sphere intersection.
        // TODO: Initialize SurfaceInteraction from parametric information.
        // TODO: Update tHit for quadric intersection.
        todo!()
    }

    fn does_ray_intersect(&self, ray: &crate::ray::Ray, test_alpha_texture: bool) -> bool {
        self.ray_intersection(ray, test_alpha_texture).is_some()
    }

    fn surface_area(&self) -> f32 {
        todo!()
    }
}
