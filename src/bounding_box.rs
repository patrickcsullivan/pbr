use std::ops::Bound;

pub type Bounds2i = Bounds2<i32>;
pub type Bounds2f = Bounds2<f32>;
pub type Bounds3i = Bounds3<i32>;
pub type Bounds3f = Bounds3<f32>;

pub struct Bounds2<S> {
    min_max: Option<(cgmath::Point2<S>, cgmath::Point2<S>)>,
}

pub struct Bounds3<S> {
    min_max: Option<(cgmath::Point3<S>, cgmath::Point3<S>)>,
}

impl<S: cgmath::BaseNum + std::cmp::Ord> Bounds3<S> {
    /// Creates a bounding box that encloses a single point.
    pub fn from_point(p: cgmath::Point3<S>) -> Self {
        Self {
            min_max: Some((p, p)),
        }
    }

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
        Self {
            min_max: Some((min, max)),
        }
    }

    pub fn corners(&self) -> Vec<cgmath::Point3<S>> {
        if let Some((min, max)) = self.min_max {
            vec![
                cgmath::Point3::new(min.x, min.y, min.z),
                cgmath::Point3::new(max.x, min.y, min.z),
                cgmath::Point3::new(min.x, max.y, min.z),
                cgmath::Point3::new(max.x, max.y, min.z),
                cgmath::Point3::new(min.x, min.y, max.z),
                cgmath::Point3::new(max.x, min.y, max.z),
                cgmath::Point3::new(min.x, max.y, max.z),
                cgmath::Point3::new(max.x, max.y, max.z),
            ]
        } else {
            vec![]
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        let min_max = match (self.min_max, other.min_max) {
            (None, None) => None,
            (None, Some(other_min_max)) => Some(other_min_max),
            (Some(self_min_max), None) => Some(self_min_max),
            (Some((self_min, self_max)), Some((other_min, other_max))) => {
                let min = cgmath::Point3::new(
                    std::cmp::min(self_min.x, other_min.x),
                    std::cmp::min(self_min.y, other_min.y),
                    std::cmp::min(self_min.z, other_min.z),
                );
                let max = cgmath::Point3::new(
                    std::cmp::max(self_max.x, other_max.x),
                    std::cmp::max(self_max.y, other_max.y),
                    std::cmp::max(self_max.z, other_max.z),
                );
                Some((min, max))
            }
        };
        Self { min_max }
    }

    pub fn union_with_point(&self, p: &cgmath::Point3<S>) -> Self {
        self.union(&Bounds3::from_point(*p))
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let min_max = match (self.min_max, other.min_max) {
            (None, None) => None,
            (None, _) => None,
            (_, None) => None,
            (Some((self_min, self_max)), Some((other_min, other_max))) => {
                let min = cgmath::Point3::new(
                    std::cmp::max(self_min.x, other_min.x),
                    std::cmp::max(self_min.y, other_min.y),
                    std::cmp::max(self_min.z, other_min.z),
                );
                let max = cgmath::Point3::new(
                    std::cmp::min(self_max.x, other_max.x),
                    std::cmp::min(self_max.y, other_max.y),
                    std::cmp::min(self_max.z, other_max.z),
                );
                if leq_3d(&min, &max) {
                    Some((min, max))
                } else {
                    None
                }
            }
        };
        Self { min_max }
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        match (self.min_max, other.min_max) {
            (None, None) => false,
            (None, _) => false,
            (_, None) => false,
            (Some((self_min, self_max)), Some((other_min, other_max))) => {
                let x_overlaps = self_min.x <= other_max.x && self_max.x >= other_min.x;
                let y_overlaps = self_min.y <= other_max.y && self_max.y >= other_min.y;
                let z_overlaps = self_min.z <= other_max.z && self_max.z >= other_min.z;
                x_overlaps && y_overlaps && z_overlaps
            }
        }
    }

    pub fn inside(&self, p: &cgmath::Point3<S>) -> bool {
        if let Some((min, max)) = self.min_max {
            p.x >= min.x
                && p.x <= max.x
                && p.y >= min.y
                && p.y <= max.y
                && p.z >= min.z
                && p.z <= max.z
        } else {
            false
        }
    }

    pub fn inside_exclusive(&self, p: &cgmath::Point3<S>) -> bool {
        if let Some((min, max)) = self.min_max {
            p.x > min.x && p.x < max.x && p.y > min.y && p.y < max.y && p.z > min.z && p.z < max.z
        } else {
            false
        }
    }

    pub fn expand(&mut self, delta: S) {
        let min_max = if let Some((min, max)) = self.min_max {
            let min = min - cgmath::Vector3::new(delta, delta, delta);
            let max = max + cgmath::Vector3::new(delta, delta, delta);
            if leq_3d(&min, &max) {
                Some((min, max))
            } else {
                None
            }
        } else {
            None
        };

        self.min_max = min_max;
    }

    pub fn diagonal(&self) -> Option<cgmath::Vector3<S>> {
        if let Some((min, max)) = self.min_max {
            Some(max - min)
        } else {
            None
        }
    }

    pub fn surface_area(&self) -> Option<S> {}
}

/// Returns true if all dimensions of the first point are less than or equal
/// to the respective dimensions of the second point.
fn leq_3d<S: std::cmp::Ord>(p1: &cgmath::Point3<S>, p2: &cgmath::Point3<S>) -> bool {
    p1.x <= p2.x && p1.y <= p2.y && p1.z <= p2.z
}
