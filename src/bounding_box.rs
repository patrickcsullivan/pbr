use crate::axis;
use cgmath::VectorSpace;

pub type Bounds2i = Bounds2<i32>;
pub type Bounds2f = Bounds2<f32>;
pub type Bounds3i = Bounds3<i32>;
pub type Bounds3f = Bounds3<f32>;

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Bounds2<S> {
    min: cgmath::Point2<S>,
    max: cgmath::Point2<S>,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Bounds3<S> {
    min: cgmath::Point3<S>,
    max: cgmath::Point3<S>,
}

impl<S: cgmath::BaseNum + std::cmp::Ord> Bounds3<S> {
    /// Creates a bounding box that encloses a single point.
    pub fn from_point(p: cgmath::Point3<S>) -> Self {
        Self { min: p, max: p }
    }

    /// Creates a bounding box that encloses the given corner points.
    pub fn from_corners(p1: cgmath::Point3<S>, p2: cgmath::Point3<S>) -> Self {
        let min = cgmath::Point3::new(
            std::cmp::min(p1.x, p2.x),
            std::cmp::min(p1.y, p2.y),
            std::cmp::min(p1.z, p2.z),
        );
        let max = cgmath::Point3::new(
            std::cmp::max(p1.x, p2.x),
            std::cmp::max(p1.y, p2.y),
            std::cmp::max(p1.z, p2.z),
        );
        Self { min, max }
    }

    /// Returns the corner points of the bounding box.
    pub fn corners(&self) -> Vec<cgmath::Point3<S>> {
        vec![
            cgmath::Point3::new(self.min.x, self.min.y, self.min.z),
            cgmath::Point3::new(self.max.x, self.min.y, self.min.z),
            cgmath::Point3::new(self.min.x, self.max.y, self.min.z),
            cgmath::Point3::new(self.max.x, self.max.y, self.min.z),
            cgmath::Point3::new(self.min.x, self.min.y, self.max.z),
            cgmath::Point3::new(self.max.x, self.min.y, self.max.z),
            cgmath::Point3::new(self.min.x, self.max.y, self.max.z),
            cgmath::Point3::new(self.max.x, self.max.y, self.max.z),
        ]
    }

    /// Returns a union of two bounding boxes.
    pub fn union(&self, other: &Self) -> Self {
        let min = cgmath::Point3::new(
            std::cmp::min(self.min.x, other.min.x),
            std::cmp::min(self.min.y, other.min.y),
            std::cmp::min(self.min.z, other.min.z),
        );
        let max = cgmath::Point3::new(
            std::cmp::max(self.max.x, other.max.x),
            std::cmp::max(self.max.y, other.max.y),
            std::cmp::max(self.max.z, other.max.z),
        );
        Self { min, max }
    }

    /// Returns a union of the bounding box and an additional point..
    pub fn union_with_point(&self, p: &cgmath::Point3<S>) -> Self {
        self.union(&Bounds3::from_point(*p))
    }

    /// Returns the intersection of the bounding boxes.
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let min = cgmath::Point3::new(
            std::cmp::max(self.min.x, other.min.x),
            std::cmp::max(self.min.y, other.min.y),
            std::cmp::max(self.min.z, other.min.z),
        );
        let max = cgmath::Point3::new(
            std::cmp::min(self.max.x, other.max.x),
            std::cmp::min(self.max.y, other.max.y),
            std::cmp::min(self.max.z, other.max.z),
        );
        if leq_3d(&min, &max) {
            Some(Self { min, max })
        } else {
            None
        }
    }

    /// Returns true if and only if the bounding boxes overlap inclusively.
    pub fn overlaps(&self, other: &Self) -> bool {
        let x_overlaps = self.min.x <= other.max.x && self.max.x >= other.min.x;
        let y_overlaps = self.min.y <= other.max.y && self.max.y >= other.min.y;
        let z_overlaps = self.min.z <= other.max.z && self.max.z >= other.min.z;
        x_overlaps && y_overlaps && z_overlaps
    }

    /// Returns true if and only if the point is inside the bounding box
    /// inclusively.
    pub fn inside(&self, p: &cgmath::Point3<S>) -> bool {
        p.x >= self.min.x
            && p.x <= self.max.x
            && p.y >= self.min.y
            && p.y <= self.max.y
            && p.z >= self.min.z
            && p.z <= self.max.z
    }

    /// Returns true if and only if the point is inside the bounding box
    /// exclusively.
    pub fn inside_exclusive(&self, p: &cgmath::Point3<S>) -> bool {
        p.x >= self.min.x
            && p.x < self.max.x
            && p.y > self.min.y
            && p.y < self.max.y
            && p.z > self.min.z
            && p.z < self.max.z
    }

    /// Expands the bounding box by the given `delta`. If `delta` is zero or
    /// negative the bounding box is unchanged.
    pub fn expand(&mut self, delta: S) {
        if delta > S::zero() {
            let min = self.min - cgmath::Vector3::new(delta, delta, delta);
            let max = self.max + cgmath::Vector3::new(delta, delta, delta);
            self.min = min;
            self.max = max;
        }
    }

    /// Returns a vector across the diagonal of the bounding box, pointing from
    /// the miminum corner to the maximum corner.
    pub fn diagonal(&self) -> cgmath::Vector3<S> {
        self.max - self.min
    }

    /// Returns the surface area of the bounding box.
    pub fn surface_area(&self) -> S {
        let d = self.diagonal();
        let half = d.x * d.y + d.x * d.z + d.y * d.z;
        half + half
    }

    /// Returns the volume of the bounding box.
    pub fn volume(&self) -> S {
        let d = self.diagonal();
        d.x * d.y * d.z
    }

    /// Returns the longest axis of the bounding box.
    pub fn maximum_extend(&self) -> axis::Axis3 {
        let d = self.diagonal();
        if d.x > d.y && d.x > d.z {
            axis::Axis3::X
        } else if d.y > d.z {
            axis::Axis3::Y
        } else {
            axis::Axis3::Z
        }
    }

    /// Returns the result of linearly interpolating from the bounding box's
    /// minimum corner to its maximum corner.
    pub fn lerp(&self, amount: S) -> cgmath::Point3<S> {
        let origin = cgmath::Point3::new(S::zero(), S::zero(), S::zero());
        let min = self.min - origin;
        let max = self.max - origin;
        let lerp = min.lerp(max, amount);
        cgmath::Point3::new(lerp.x, lerp.y, lerp.z)
    }

    // TODO: offset, p. 81
    // TODO: bounding_sphere, p. 81
}

/// Returns true if all dimensions of the first point are less than or equal
/// to the respective dimensions of the second point.
fn leq_3d<S: std::cmp::Ord>(p1: &cgmath::Point3<S>, p2: &cgmath::Point3<S>) -> bool {
    p1.x <= p2.x && p1.y <= p2.y && p1.z <= p2.z
}
