use std::ops::{Add, Mul, Sub};
use std::time::Instant;
use cgmath::{ElementWise, InnerSpace};
use image::{EncodableLayout, ImageBuffer};
use crate::hit::{Hit, Hittable, Object};

// https://doc.rust-kr.org/ch17-00-oop.html
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::light::Light;
use crate::square::Square;
use crate::texture::Texture;
use crate::triangle::Triangle;

const RECURSIVE_LEVEL: i32 = 5;

pub struct Raytracer {
    pub width: i32,
    pub height: i32,

    // sphere: Sphere,
    light: Light,

    objects: Vec<Box<dyn Hittable>>,
}

impl Raytracer {
    pub fn new(width: i32, height: i32) -> Self {
        // texture
        let texture1 = Texture::new("./src/images/shadertoy_abstract1.jpg");
        let texture2 = Texture::new("./src/images/back.jpg");

        let mut sphere1 = Sphere::new(cgmath::vec3(0.0, -0.1, 1.5), 1.0);
        sphere1.amb = cgmath::vec3(0.2, 0.2, 0.2);
        sphere1.diff = cgmath::vec3(0.0, 0.0, 1.0);
        sphere1.spec = cgmath::vec3(0.0, 0.0, 0.0);
        sphere1.alpha = 50.0;
        // sphere1.ks = 0.8;
        sphere1.reflection = 0.0;
        sphere1.transparency = 1.0;

        let mut ground = Square::new(
            // vertices
            cgmath::vec3(-10.0, -1.5, 0.0),
            cgmath::vec3(-10.0, -1.5, 10.0),
            cgmath::vec3(10.0, -1.5, 10.0),
            cgmath::vec3(10.0, -1.5, 0.0),
            // uv
            cgmath::vec2(0.0, 0.0),
            cgmath::vec2(1.0, 0.0),
            cgmath::vec2(1.0, 1.0),
            cgmath::vec2(0.0, 1.0),
        );
        ground.amb = cgmath::vec3(1.0, 1.0, 1.0);
        ground.diff = cgmath::vec3(1.0, 1.0, 1.0);
        ground.spec = cgmath::vec3(1.0, 1.0, 1.0);
        ground.alpha = 10.0;
        ground.reflection = 0.0;

        ground.amb_tex = Some(texture1.clone());
        ground.dif_tex = Some(texture1.clone());

        let mut square = Square::new(
            // vertices
            cgmath::vec3(-10.0, 10.0, 10.0),
            cgmath::vec3(10.0, 10.0, 10.0),
            cgmath::vec3(10.0, -10.0, 10.0),
            cgmath::vec3(-10.0, -10.0, 10.0),
            // uv
            cgmath::vec2(0.0, 0.0),
            cgmath::vec2(1.0, 0.0),
            cgmath::vec2(1.0, 1.0),
            cgmath::vec2(0.0, 1.0),
        );
        square.amb = cgmath::vec3(1.0, 1.0, 1.0);
        square.diff = cgmath::vec3(0.0, 0.0, 0.0);
        square.spec = cgmath::vec3(0.0, 0.0, 0.0);
        square.alpha = 10.0;
        square.reflection = 0.0;

        square.amb_tex = Some(texture2.clone());
        square.dif_tex = Some(texture2.clone());

        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

        objects.push(Box::new(ground));
        objects.push(Box::new(square));
        objects.push(Box::new(sphere1));

        // located back of screen
        let light = Light { pos: cgmath::vec3(0.0, 0.3, -0.5) };

        Raytracer { width, height, light, objects }
    }

    pub fn find_closest_collision(&self, ray: &Ray) -> Hit {
        let mut closest_hit = Hit::new(-1.0, cgmath::Vector3::new(0.0, 0.0, 0.0), cgmath::Vector3::new(0.0, 0.0, 0.0));
        let mut closest_distance = f32::MAX;

        for l in self.objects.iter() {
            let hit = l.intersect_ray_collision(ray);

            if hit.d < 0.0 {
                continue;
            }

            if hit.d < closest_distance {
                closest_distance = hit.d;

                closest_hit.d = hit.d;
                closest_hit.normal = hit.normal;
                closest_hit.point = hit.point;
                closest_hit.uv = hit.uv;
                closest_hit.object = Some(l.as_ref());
            }
        }

        closest_hit
    }

    pub fn tracy_ray(&self, ray: &Ray, level: i32) -> cgmath::Vector3<f32> {
        let mut color = cgmath::vec3(0.0, 0.0, 0.0);
        let mut phong_color = cgmath::vec3(0.0, 0.0, 0.0);

        if level < 0 {
            return color;
        }

        let hit = self.find_closest_collision(ray);

        if hit.d >= 0.0 {
            if let Some(object) = hit.object {
                // 각 object 해당하는 프로퍼티 반환
                let light_properties = object.get_light_color_properties();
                let amb = light_properties.amb;
                let diff = light_properties.diff;
                let spec = light_properties.spec;
                let alpha = light_properties.alpha;

                // diffuse
                let l = (self.light.pos - hit.point).normalize();
                let n = hit.normal.normalize();

                let diffuse = cgmath::dot(n, l).max(0.0) * diff;

                // specular
                let r = 2.0 * cgmath::dot(n, l) * n - l;
                let e = (-1.0 * ray.dir).normalize();

                let specular = spec * cgmath::dot(r, e).max(0.0).powf(alpha);

                let amb_texture = object.get_ambient_texture();
                let dif_texture = object.get_diffuse_texture();

                let reflection = object.get_reflection();
                let transparency = object.get_transparency();

                // texture calculation - ambient
                if amb_texture.is_some() {
                    phong_color += amb.mul_element_wise(
                        amb_texture.as_ref().expect("fail to access the texture").get_sample_linear(&hit.uv)
                    );
                } else {
                    phong_color += amb;
                }

                // texture calculation - diffuse
                if dif_texture.is_some() {
                    phong_color += diffuse.mul_element_wise(
                        dif_texture.as_ref().expect("fail to access the texture").get_sample_linear(&hit.uv)
                    );
                } else {
                    phong_color += diffuse;
                }

                phong_color += specular;

                color += phong_color * (1.0 - reflection - transparency);

                if reflection > 0.0 {
                    let m = -1.0 * hit.normal.dot(ray.dir) * hit.normal + ray.dir;
                    let reflected_dir = ((2.0 * m) - ray.dir).normalize();

                    let reflected_ray = Ray { start: hit.point + (reflected_dir * 1e-4f32), dir: reflected_dir };
                    color += self.tracy_ray(&reflected_ray, level - 1);
                }

                // 참고
                // https://samdriver.xyz/article/refraction-sphere (그림들이 좋아요)
                // https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel (오류있음)
                // https://web.cse.ohio-state.edu/~shen.94/681/Site/Slides_files/reflection_refraction.pdf (슬라이드가 보기 좋지는 않지만 정확해요)
                if transparency > 0.0 {
                    const INDEX_OF_REFRACTION: f32 = 1.5;

                    let mut eta: f32;
                    let mut normal: cgmath::Vector3<f32>;

                    // 밖에서 안에서 들어가는 경우 (예: 공기->유리)
                    if ray.dir.dot(hit.normal) < 0.0 {
                        eta = INDEX_OF_REFRACTION;
                        normal = hit.normal;
                    }
                    // 안에서 밖으로 나가는 경우 (예: 유리->공기)
                    else {
                        eta = 1.0 / INDEX_OF_REFRACTION;
                        normal = -1.0 * hit.normal;
                    }

                    let cos_theta1 = -normal.dot(ray.dir);
                    let sin_theta1 = (1.0 - cos_theta1 * cos_theta1).sqrt();
                    let sin_theta2 = sin_theta1 / eta;
                    let cos_theta2 = (1.0 - sin_theta2 * sin_theta2).sqrt();

                    let m = (normal.dot(-ray.dir) * normal + ray.dir).normalize();
                    let a = m * sin_theta2;
                    let b = -normal * cos_theta2;
                    let refracted_dir = (a + b).normalize();

                    let refracted_ray = Ray { start: hit.point + (refracted_dir * 1e-4f32), dir: refracted_dir };
                    color += self.tracy_ray(&refracted_ray, level - 1) * transparency;
                }
            }
        }

        color
    }

    fn trace_ray_2x2(&self, eye_pos: &cgmath::Vector3<f32>, pixel_pos: &cgmath::Vector3<f32>, dx: f32, level: i32) -> cgmath::Vector3<f32> {
        if level == 0 {
            let ray = Ray { dir: (pixel_pos - eye_pos).normalize(), start: *pixel_pos };
            self.tracy_ray(&ray, 0)
        } else {
            let sub_dx = 0.5 * dx;

            let mut pixel_color: cgmath::Vector3<f32> = cgmath::vec3(0.0, 0.0, 0.0);
            let mut pos = cgmath::vec3(
                pixel_pos.x - sub_dx * 0.5,
                pixel_pos.y - sub_dx * 0.5,
                pixel_pos.z,
            );

            for j in 0..2 {
                for i in 0..2 {
                    let sub_pos = cgmath::vec3(
                        pos.x + (i as f32) * sub_dx,
                        pos.y + (j as f32) * sub_dx,
                        pos.z,
                    );
                    let color = self.trace_ray_2x2(eye_pos, &sub_pos, sub_dx, level - 1);
                    pixel_color += color;
                }
            }

            pixel_color.mul(0.25)
        }
    }

    pub fn render(&self, imgbuf: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
        println!("start of render!");

        let start = Instant::now();

        let eye_pos = cgmath::Vector3::new(0.0, 0.0, -1.5);

        let dx = 2.0 / self.height as f32;

        for j in 0..self.height {
            for i in 0..self.width {
                let pixel_pos_world = self.transform_screen_to_world(cgmath::vec2(i as f32, j as f32));

                // 광선의 방향 벡터
                // 스크린에 수직인 z 방향, 유닛벡터
                // let ray_dir = cgmath::vec3(0.0, 0.0, 1.0);
                let ray_dir = (pixel_pos_world - eye_pos).normalize();

                // cgmath::vec3(0.0, 0.0, 1.0);

                // general
                let pixel_ray = Ray { dir: ray_dir, start: pixel_pos_world };
                let color = self.tracy_ray(&pixel_ray, RECURSIVE_LEVEL);

                // super-sampling
                // let color = self.trace_ray_2x2(&eye_pos, &pixel_pos_world, dx, 3);

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
            0.0,
        )
    }

    fn calculate_phong_model_color(
        &self,
        hit: &Hit,
        ray: &Ray,
        object: &Object,
    ) -> cgmath::Vector3<f32> {
        let mut color = cgmath::vec3(0.0, 0.0, 0.0);

        // diffuse
        let l = (self.light.pos - hit.point).normalize();
        let n = hit.normal.normalize();

        let diff = cgmath::dot(n, l).max(0.0);

        // specular
        let r = 2.0 * cgmath::dot(n, l) * n - l;
        let e = (-1.0 * ray.dir).normalize();

        let specular = cgmath::dot(r, e)
            .max(0.0);

        match object {
            Object::Sphere(s) => {
                let specular_pow = specular.powf(s.alpha);
                color = s.amb + (s.diff * diff) + (s.spec * specular_pow);
            }
            Object::Triangle(t) => {
                let specular_pow = specular.powf(t.alpha);
                color = t.amb + (t.diff * diff) + (t.spec * specular_pow);
            }
            Object::Square(s) => {
                let specular_pow = specular.powf(s.alpha);
                color = s.amb + (s.diff * diff) + (s.spec * specular_pow);
            }
            _ => {}
        }

        color
    }

    // fn get_ambient_color(&self, object: &dyn Hittable) -> cgmath::Vector3<f32> {
    //     let mut color = cgmath::vec3(0.0, 0.0, 0.0);
    //
    //     object.
    //
    //     match object {
    //         Object::Sphere(s) => {
    //             color = s.amb;
    //         }
    //         Object::Triangle(t) => {
    //             color = t.amb;
    //         }
    //         Object::Square(s) => {
    //             color = s.amb;
    //         }
    //         _ => {}
    //     }
    //
    //     color
    // }

    // fn get_specular_color(&self, object: &dyn Hittable) -> (cgmath::Vector3<f32>, f32, f32) {
    //     match object {
    //         Object::Sphere(s) => (s.spec, s.ks, s.alpha),
    //         Object::Triangle(t) => (t.spec, t.ks, t.alpha),
    //         Object::Square(s) => (s.spec, s.ks, s.alpha),
    //         _ => (cgmath::Vector3::new(0.0, 0.0, 0.0), 0.0, 0.0)
    //     }
    // }
    //
    // fn get_diffuse_color(&self, object: &dyn Hittable) -> cgmath::Vector3<f32> {
    //     match object {
    //         Object::Sphere(s) => s.diff,
    //         Object::Triangle(t) => t.diff,
    //         Object::Square(s) => s.diff,
    //         _ => cgmath::Vector3::new(0.0, 0.0, 0.0),
    //     }
    // }
}

// #[test]
// fn test_raytracer_render() {
//     let raytracer = Raytracer::new(100, 100);
//     raytracer.render();
// }