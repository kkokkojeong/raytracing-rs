use crate::light::Light;
// https://doc.rust-kr.org/ch17-00-oop.html
use crate::ray::Ray;
use crate::sphere::Sphere;

pub struct Raytracer {
    pub width: i32,
    pub height: i32,

    // sphere: Sphere,
    // light: Light,
}

impl Raytracer {
    pub fn new(width: i32, height: i32) -> Raytracer {
        Raytracer { width, height }
    }

    pub fn tracy_ray(ray: Ray) -> cgmath::Point3<f32> {
        let hit = cgmath::point3(0.0, 0.0, 0.0);

        return hit;
    }
}