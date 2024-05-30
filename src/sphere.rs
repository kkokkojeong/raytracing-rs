use cgmath::InnerSpace;
use crate::ray::Ray;
use crate::hit::{Hit, Hittable};

pub struct Sphere {
    // geometry properties
    pub center: cgmath::Vector3<f32>,
    pub radius: f32,

    // light properties
    pub amb: cgmath::Vector3<f32>, // ambient
    pub diff: cgmath::Vector3<f32>, // diffuse
    pub spec: cgmath::Vector3<f32>, // specular
    pub ks: f32,
    pub alpha: f32,
}

impl Sphere {
    pub fn new(center: cgmath::Vector3<f32>, radius: f32) -> Sphere {
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
}

impl Hittable for Sphere {
    fn intersect_ray_collision(&self, ray: &Ray) -> Hit {
        // Wikipedia Line–sphere intersection
        // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let mut hit = Hit::new(-1.0, cgmath::vec3(0.0, 0.0, 0.0), cgmath::vec3(0.0, 0.0, 0.0));

        let to_ray_dir = ray.start - self.center;

        let b = 2.0 * cgmath::dot(ray.dir, to_ray_dir);
        let c = cgmath::dot(to_ray_dir, to_ray_dir) - self.radius * self.radius;

        let det = b * b - 4.0 * c;

        if (det >= 0.0) {
            let d1 = (-b - det.sqrt()) / 2.0;
            let d2 = (-b + det.sqrt()) / 2.0;

            hit.d = d1.min(d2);
            hit.point = ray.start + ray.dir * hit.d;
            hit.normal = (hit.point - self.center).normalize();
        }

        return hit;
    }
}