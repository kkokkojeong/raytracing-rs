use crate::ray::Ray;

pub struct Sphere {
    // geometry properties
    pub center: cgmath::Point3<f32>,
    pub radius: f32,

    // light properties
    pub amb: cgmath::Vector3<f32>, // ambient
    pub diff: cgmath::Vector3<f32>, // diffuse
    pub spec: cgmath::Vector3<f32>, // specular
    pub ks: f32,
    pub alpha: f32,
}

impl Sphere {
    pub fn new(center: cgmath::Point3<f32>, radius: f32) -> Sphere {
        Sphere {
            center,
            radius,
            amb: cgmath::Vector3::new(0.0, 0.0, 0.0),
            diff: cgmath::Vector3::new(0.0, 0.0, 0.0),
            spec: cgmath::Vector3::new(0.0, 0.0, 0.0),
            ks: 0.0,
            alpha: 0.0
        }
    }

    pub fn intersect_ray_collision(ray: Ray) -> bool {
        true
    }
}