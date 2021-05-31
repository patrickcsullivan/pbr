use crate::medium;
use cgmath::Transform;

pub struct Ray {
    pub origin: cgmath::Point3<f32>,
    pub direction: cgmath::Vector3<f32>,

    /// The upper bound of t in the ray's parametric equation,
    /// r(t) = o + t*d, 0 < t < time_max
    /// Limits the ray to a finite segment.
    pub t_max: f32,

    /// A time value used when rendering an animated scene.
    /// Not really sure why this is necessary. From p. 73.
    pub time: f32,

    /// The medium containing the ray's origin. This is used in accounting for
    /// effects when a ray passes from one medium to another.
    pub medium: Option<medium::Medium>,
}

impl Ray {
    /// Get the position along the ray for a given value for the parameter, t.
    fn at_t(&self, t: f32) -> cgmath::Point3<f32> {
        self.origin + self.direction * t
    }

    fn transform(&mut self, transform: cgmath::Matrix4<f32>) {
        // TODO: Deal with round-off error in origin transformation. (p. 95)
        self.origin = transform.transform_point(self.origin);
        self.direction = transform.transform_vector(self.direction);
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: cgmath::Point3::new(0.0, 0.0, 0.0),
            direction: cgmath::Vector3::new(0.0, 0.0, 0.0),
            t_max: 0.0,
            time: 0.0,
            medium: None,
        }
    }
}

/// A primary ray along with two auxilary rays. Tee auxilary rays are offset
/// from the primary ray by one sample in the x and y directions, respectively,
/// on the film plane.
pub struct RayDifferential {
    pub primary: Ray,

    // TODO: Use Option instead.
    pub has_differentials: bool,

    /// Origin of a ray that is offset from the primary ray by one sample in the
    /// x direction on the film plane.
    pub dx_origin: cgmath::Point3<f32>,

    /// Direction of a ray that is offset from the primary ray by one sample in
    /// the x direction on the film plane.
    pub dx_direction: cgmath::Vector3<f32>,

    /// Origin of a ray that is offset from the primary ray by one sample in the
    /// y direction on the film plane.
    pub dy_origin: cgmath::Point3<f32>,

    /// Direction of a ray that is offset from the primary ray by one sample in
    /// the x direction on the film plane.
    pub dy_direction: cgmath::Vector3<f32>,
}

impl RayDifferential {
    /// Scale the sample distance between the auxilary rays and the primary ray
    /// by the given factor.
    pub fn scale_sample_distance(&mut self, factor: f32) {
        self.dx_origin = self.primary.origin + (self.dx_origin - self.primary.origin) * factor;
        self.dy_origin = self.primary.origin + (self.dy_origin - self.primary.origin) * factor;
        self.dx_direction =
            self.primary.direction + (self.dx_direction - self.primary.direction) * factor;
        self.dy_direction =
            self.primary.direction + (self.dy_direction - self.primary.direction) * factor;
    }
}

impl Default for RayDifferential {
    fn default() -> Self {
        Self {
            primary: Ray::default(),
            has_differentials: false,
            dx_origin: cgmath::Point3::new(0.0, 0.0, 0.0),
            dx_direction: cgmath::Vector3::new(0.0, 0.0, 0.0),
            dy_origin: cgmath::Point3::new(0.0, 0.0, 0.0),
            dy_direction: cgmath::Vector3::new(0.0, 0.0, 0.0),
        }
    }
}
