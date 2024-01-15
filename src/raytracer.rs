// https://doc.rust-kr.org/ch17-00-oop.html
use crate::ray::Ray;

pub struct Raytracer {
    pub width: i32,
    pub height: i32,
}

impl Raytracer {
    pub fn new(w: i32, h: i32) -> Raytracer {
        println!("Raytracer create {}, {}", w, h);
        Raytracer { width: w, height: h }
    }

    pub fn tracy_ray(ray: Ray) -> cgmath::Point3<f32> {
        let hit = cgmath::point3(0.0, 0.0, 0.0);

        return hit;
    }
}