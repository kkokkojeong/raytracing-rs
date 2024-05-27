use std::ops::Mul;
use std::time::Instant;
use cgmath::InnerSpace;
use image::{EncodableLayout, ImageBuffer};
// https://doc.rust-kr.org/ch17-00-oop.html
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::light::Light;

pub struct Raytracer {
    pub width: i32,
    pub height: i32,

    sphere: Sphere,
    light: Light,
}

impl Raytracer {
    pub fn new(width: i32, height: i32) -> Self {
        let mut sphere = Sphere::new(cgmath::vec3(0.0, 0.0, 0.5), 0.5);
        sphere.amb = cgmath::vec3(0.0, 0.0, 0.0);
        sphere.diff = cgmath::vec3(0.0, 0.0, 1.0);
        sphere.spec = cgmath::vec3(1.0, 1.0, 1.0);
        sphere.alpha = 9.0;
        sphere.ks = 0.8;

        // located back of screen
        let light = Light{ pos: cgmath::Vector3::new(0.0, 0.0, -1.0) };

        Raytracer { width, height, sphere, light }
    }

    pub fn tracy_ray(&self, ray: Ray) -> cgmath::Vector3<f32> {
        let hit = self.sphere.intersect_ray_collision(&ray);

        if hit.d < 0.0 {
            cgmath::vec3(0.0, 0.0, 0.0)
        } else {
            // Phong reflection model.

            // diffuse
            let l = (self.light.pos - hit.point).normalize();
            let n = hit.normal.normalize();

            let diff = cgmath::dot(n, l).max(0.0);

            // specular
            let r = 2.0 * cgmath::dot(n, l) * n - l;
            let e = (-1.0 * ray.dir).normalize();

            let specular = cgmath::dot(r, e)
                .max(0.0)
                .powf(self.sphere.alpha);

            self.sphere.amb + (self.sphere.diff * diff) + (self.sphere.spec * specular * self.sphere.ks)
        }
    }

    pub fn render(&self, mut imgbuf: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
        println!("start of render!");

        let start = Instant::now();

        for j in 0..self.height {
            for i in 0..self.width {
                let pixel_pos_world = self.transform_screen_to_world(cgmath::vec2(i as f32, j as f32));

                // 광선의 방향 벡터
                // 스크린에 수직인 z 방향, 유닛벡터
                let ray_dir = cgmath::vec3(0.0, 0.0, 1.0);

                let pixel_ray = Ray { dir: ray_dir, start: pixel_pos_world };
                let color = self.tracy_ray(pixel_ray);

                let r = (color.x * 255.0).clamp(0.0, 255.0) as u8;
                let g = (color.y * 255.0).clamp(0.0, 255.0) as u8;
                let b = (color.z * 255.0).clamp(0.0, 255.0) as u8;

                imgbuf.put_pixel(i as u32, j as u32, image::Rgb([r, g, b]));
            }
        }

        let elapsed = start.elapsed();

        println!("end of render! {:?} ms", elapsed.as_millis());

        // For debugging
        image::save_buffer("tmp_ray_result.png", imgbuf.as_bytes(), self.width as u32, self.height as u32, image::ExtendedColorType::Rgb8).unwrap();
    }

    fn transform_screen_to_world(&self, pos: cgmath::Vector2<f32>) -> cgmath::Vector3<f32> {
        let w = self.width as f32;
        let h = self.height as f32;
        let x_scale = 2.0 / w;
        let y_scale = 2.0 / h;
        let aspect = w / h;

        cgmath::vec3(
            (pos.x * x_scale - 1.0) * aspect,
            -pos.y * y_scale + 1.0,
            0.0
        )
    }
}

// #[test]
// fn test_raytracer_render() {
//     let raytracer = Raytracer::new(100, 100);
//     raytracer.render();
// }