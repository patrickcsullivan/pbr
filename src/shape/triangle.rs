use super::Shape;
use crate::bounding_box::Bounds3;
use cgmath::{Matrix4, Point2, Point3, Transform, Vector3};

/// A mesh of triangles.
pub struct TriangleMesh<'a> {
    object_to_world: &'a Matrix4<f32>,
    world_to_object: &'a cgmath::Matrix4<f32>,
    reverse_orientation: bool,

    /// The vertices in world space that make up the mesh.
    world_space_vertices: Vec<Point3<f32>>,

    /// An array that describes each triangle in the mesh. Each element of the
    /// array is a tuple that contains three indices into the `vertices` array.
    triangle_vertex_indices: Vec<(usize, usize, usize)>,

    /// An array containing a tangent vector for each vertex in the mesh.
    tangents: Option<Vec<Vector3<f32>>>,

    /// An array containing a normal vector for each vertex in the mesh.
    normals: Option<Vec<Vector3<f32>>>,

    /// An array containing a UV coordinate for each vertex in the mesh.
    uvs: Option<Vec<Point2<f32>>>,
}

/// A reference to an individual triangle in a mesh.
pub struct Triangle<'a> {
    mesh: &'a TriangleMesh<'a>,
    index_in_mesh: usize,
}

impl<'a> Triangle<'a> {
    pub fn object_space_vertices(&self) -> (Point3<f32>, Point3<f32>, Point3<f32>) {
        let (i0, i1, i2) = self.mesh.triangle_vertex_indices[self.index_in_mesh];
        let p0 = self.world_to_object().transform_point(self.mesh.world_space_vertices[i0]);
        let p1 = self.world_to_object().transform_point(self.mesh.world_space_vertices[i1]);
        let p2 = self.world_to_object().transform_point(self.mesh.world_space_vertices[i2]);
        (p0, p1, p2)
    }

    pub fn world_space_vertices(&self) -> (Point3<f32>, Point3<f32>, Point3<f32>) {
        let (i0, i1, i2) = self.mesh.triangle_vertex_indices[self.index_in_mesh];
        let p0 = self.mesh.world_space_vertices[i0];
        let p1 = self.mesh.world_space_vertices[i1];
        let p2 = self.mesh.world_space_vertices[i2];
        (p0, p1, p2)
    }
}

impl<'a> TriangleMesh<'a> {
    pub fn triangle_at(&'a self, index: usize) -> Triangle<'a> {
        Triangle {
            mesh: self,
            index_in_mesh: index,
        }
    }
}

pub struct TiangleMeshBuilder<'a> {
    object_to_world: &'a Matrix4<f32>,
    world_to_object: &'a cgmath::Matrix4<f32>,
    reverse_orientation: bool,
    object_space_vertices: Vec<Point3<f32>>,
    triangle_vertex_indices: Vec<(usize, usize, usize)>,
    tangents: Option<Vec<Vector3<f32>>>,
    normals: Option<Vec<Vector3<f32>>>,
    uvs: Option<Vec<Point2<f32>>>,
}

impl<'a> TiangleMeshBuilder<'a> {
    pub fn new(
        object_to_world: &'a Matrix4<f32>,
        world_to_object: &'a cgmath::Matrix4<f32>,
        reverse_orientation: bool,
        object_space_vertices: Vec<Point3<f32>>,
        triangle_vertex_indices: Vec<(usize, usize, usize)>,
    ) -> Self {
        Self {
            object_to_world,
            world_to_object,
            reverse_orientation,
            object_space_vertices,
            triangle_vertex_indices,
            tangents: None,
            normals: None,
            uvs: None,
        }
    }

    pub fn tangents(mut self, tangents: Vec<Vector3<f32>>) -> Self {
        self.tangents = Some(tangents);
        self
    }

    pub fn normals(mut self, normals: Vec<Vector3<f32>>) -> Self {
        self.normals = Some(normals);
        self
    }

    pub fn uvs(mut self, uvs: Vec<Point2<f32>>) -> Self {
        self.uvs = Some(uvs);
        self
    }
}

impl<'a> Shape<'a> for Triangle<'a> {
    fn object_to_world(&self) -> &'a Matrix4<f32> {
        self.mesh.object_to_world
    }

    fn world_to_object(&self) -> &'a Matrix4<f32> {
        self.mesh.world_to_object
    }

    fn object_to_world_swaps_handedness(&self) -> bool {
        todo!();
    }

    fn reverse_orientation(&self) -> bool {
        self.mesh.reverse_orientation
    }

    fn object_bound(&self) -> Bounds3<f32> {
        let (p0, p1, p2) = self.object_space_vertices();
        Bounds3::from_corners(p0, p1).union_with_point(&p2)
    }

    fn world_bound(&self) -> Bounds3<f32> {
        let (p0, p1, p2) = self.world_space_vertices();
        Bounds3::from_corners(p0, p1).union_with_point(&p2)
    }

    fn ray_intersection(
        &self,
        ray: &crate::ray::Ray,
        test_alpha_texture: bool,
    ) -> Option<(f32, crate::interaction::SurfaceInteraction)> {
        let (p0, p1, p2) = self.world_space_vertices();
        // TODO: Perform ray-triangle intersection test.
        // TODO: Compute triangle partial derivatives.
        // TODO: Compute error bounds for triangle intersection.
        // TODO: Interpolate (u, v) parametric coordinates and hit point.
        // TODO: Test interesection against alpha texture, if present.
        // TODO: Fill in SurfaceInteraction from triangle hit.
        todo!()
    }

    fn does_ray_intersect(&self, ray: &crate::ray::Ray, test_alpha_texture: bool) -> bool {
        self.ray_intersection(ray, test_alpha_texture).is_some()
    }

    fn surface_area(&self) -> f32 {
        todo!()
    }
}
