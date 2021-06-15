use cgmath::Vector3;
use efloat::EFloat32;
use std::ops::Mul;

// /// A wrapper around `cgmath::Vector<3>` for which arithmetic operator traits
// /// can be implemented.
// pub struct Vector3Arith<S>(Vector3<S>);

// impl Mul<Vector3Arith<EFloat32>> for EFloat32 {
//     type Output = Vector3<EFloat32>;

//     fn mul(self, rhs: Vector3Arith<EFloat32>) -> Self::Output {
//         let rhs = rhs.0;
//         cgmath::Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
//     }
// }
