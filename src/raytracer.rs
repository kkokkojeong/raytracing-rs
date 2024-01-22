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
        let mut sphere = Sphere::new(cgmath::Point3::new(0.0, 0.0, 0.5), 0.5);
        sphere.amb = cgmath::Vector3::new(0.0, 0.0, 0.0);
        sphere.diff = cgmath::Vector3::new(0.0, 0.0, 1.0);
        sphere.spec = cgmath::Vector3::new(1.0, 1.0, 1.0);
        sphere.alpha = 9.0;
        sphere.ks = 0.8;

        // located back of screen
        let light = Light{ pos: cgmath::Vector3::new(0.0, 0.0, -1.0) };


        Raytracer { width, height, sphere, light }
    }

    pub fn tracy_ray(ray: Ray) -> cgmath::Point3<f32> {
        let hit = cgmath::point3(0.0, 0.0, 0.0);

        return hit;
    }

    pub fn render(&self) {
        println!("start of render!");

        for j in 0..self.height {
            for i in 0..self.width {
                let pixelPosWorld = self.transform_screen_to_world(cgmath::vec2(i as f32, j as f32));
            }
        }

        println!("end of render!");
    }

    fn transform_screen_to_world(&self, pos: cgmath::Vector2<f32>) -> cgmath::Vector3<f32> {
        let w = self.width as f32;
        let h = self.height as f32;
        let x_scale = 2.0 / w;
        let y_scale = 2.0 / h;
        let aspect = w / h;

        println!("apsect {}", aspect);

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