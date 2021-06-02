pub struct Shape {
    /// Flag that indicates whether the shape's normals should be flipped from
    /// their original directions so that they point to the outside of the
    /// shape.
    pub reverse_orientation: bool,

    /// Flag that indicates whether the shape's `transform` swaps the handedness
    /// of the object coordinate system for the shape.
    pub transform_swaps_handedness: bool,
}
