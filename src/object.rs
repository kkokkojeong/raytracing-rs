// use crate::sphere::Sphere;
// use crate::triangle::Triangle;
//
// pub struct Material {
//     pub amb: cgmath::Vector3<f32>,
//     pub diff: cgmath::Vector3<f32>,
//     pub spec: cgmath::Vector3₩<f32>,
//     pub alpha: f32,
// }
//
// pub enum Object {
//     Triangle(Triangle),
//     Sphere(Sphere),
// }
//
// impl Object {
//     fn material(&self) -> Material {
//         match self {
//             Object::Triangle(t) => Material { amb: *t.amb, diff: *t.diff, spec: *t.spec, alpha: *t.alpha },
//             Object::Sphere(t) => Material { amb: *t.amb, diff: *t.diff, spec: *t.spec, alpha: *t.alpha }
//         }
//     }
// }
