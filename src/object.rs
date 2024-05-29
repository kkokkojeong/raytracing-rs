use crate::hit::Hit;
use crate::ray::Ray;

struct Object {
    pub amb: cgmath::Vector3<f32>,
    pub diff: cgmath::Vector3<f32>,
    pub spec: cgmath::Vector3<f32>,
    pub alpha: f32,
}

impl Object {
    pub fn new(color: cgmath::Vector3<f32>) -> Object {
        Object {
            amb: color.clone(),
            diff: color.clone(),
            spec: color.clone(),
            alpha: 10.0,
        }
    }

    // pub fn intersect_ray_collision(&self, ray: &Ray) -> Hit {
    //     Hit {
    //         d: 0.0,
    //         point: cgmath::Vector3::new(0.0, 0.0, 0.0),
    //         normal: cgmath::Vector3::new(0.0, 0.0, 0.0),
    //     }
    // }
}
