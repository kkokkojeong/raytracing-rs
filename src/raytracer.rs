use std::ops::{Mul, Sub};
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

pub struct Raytracer {
    pub width: i32,
    pub height: i32,

    // sphere: Sphere,
    light: Light,

    objects: Vec<Box<dyn Hittable>>,
}

impl Raytracer {
    pub fn new(width: i32, height: i32) -> Self {
        let mut sphere1 = Sphere::new(cgmath::vec3(1.0, 0.0, 1.5), 0.8);
        sphere1.amb = cgmath::vec3(0.2, 0.2, 0.2);
        sphere1.diff = cgmath::vec3(1.0, 0.2, 0.2);
        sphere1.spec = cgmath::vec3(0.5, 0.5, 0.5);
        sphere1.alpha = 10.0;
        // sphere1.ks = 0.8;

        // let mut sphere2 = Sphere::new(cgmath::vec3(0.0, 0.0, 1.0), 0.5);
        // sphere2.amb = cgmath::vec3(0.2, 0.2, 0.2);
        // sphere2.diff = cgmath::vec3(0.2, 1.0, 0.2);
        // sphere2.spec = cgmath::vec3(0.5, 0.5, 0.5);
        // sphere2.alpha = 10.0;
        // sphere2.ks = 0.8;

        // let mut sphere3 = Sphere::new(cgmath::vec3(-0.5, 0.0, 1.5), 0.5);
        // sphere3.amb = cgmath::vec3(0.2, 0.2, 0.2);
        // sphere3.diff = cgmath::vec3(0.2, 0.2, 1.0);
        // sphere3.spec = cgmath::vec3(0.5, 0.5, 0.5);
        // sphere3.alpha = 10.0;
        // sphere3.ks = 0.8;

        // triangle 2개로 rectangle 생성
        // let mut triangle1 = Triangle::new(
        //     cgmath::vec3(-2.0, -1.0, 0.0),
        //     cgmath::vec3(-2.0, -1.0, 4.0),
        //     cgmath::vec3(2.0, -1.0, 4.0),
        // );
        // triangle1.amb = cgmath::vec3(0.2, 0.2, 0.2);
        // triangle1.diff = cgmath::vec3(0.8, 0.8, 0.8);
        // triangle1.spec = cgmath::vec3(1.0, 1.0, 1.0);
        // triangle1.alpha = 50.0;
        //
        // let mut triangle2 = Triangle::new(
        //     cgmath::vec3(-2.0, -1.0, 0.0),
        //     cgmath::vec3(2.0, -1.0, 4.0),
        //     cgmath::vec3(2.0, -1.0, 0.0),
        // );
        // triangle2.amb = cgmath::vec3(0.2, 0.2, 0.2);
        // triangle2.diff = cgmath::vec3(0.8, 0.8, 0.8);
        // triangle2.spec = cgmath::vec3(1.0, 1.0, 1.0);
        // triangle2.alpha = 50.0;

        let mut square = Square::new(
            // vertices
            cgmath::vec3(-2.0, 2.0, 2.0),
            cgmath::vec3(2.0, 2.0, 2.0),
            cgmath::vec3(2.0, -2.0, 2.0),
            cgmath::vec3(-2.0, -2.0, 2.0),
            // uv
            cgmath::vec2(0.0, 0.0),
            cgmath::vec2(1.0, 0.0),
            cgmath::vec2(1.0, 1.0),
            cgmath::vec2(0.0, 1.0),
            // cgmath::vec2(0.0, 0.0),
            // cgmath::vec2(4.0, 0.0),
            // cgmath::vec2(4.0, 4.0),
            // cgmath::vec2(0.0, 4.0),
        );
        square.amb = cgmath::vec3(0.2, 0.2, 0.2);
        square.diff = cgmath::vec3(1.0, 1.0, 1.0);
        square.spec = cgmath::vec3(0.0, 0.0, 0.0);
        // square.alpha = 50.0;

        //
        // let mut triangle = Triangle::new(
        //     cgmath::vec3(-2.0, -2.0, 2.0),
        //     cgmath::vec3(-2.0, 2.0, 2.0),
        //     cgmath::vec3(2.0, 2.0, 2.0),
        // );
        // triangle.amb = cgmath::vec3(1.0, 1.0, 1.0);
        // triangle.diff = cgmath::vec3(0.0, 0.0, 0.0);
        // triangle.spec = cgmath::vec3(0.0, 0.0, 0.0);

        // texture
        let texture = Texture::new("./src/images/shadertoy_abstract1.jpg");

        // square.dif_tex = Some(texture.clone());
        // square.amb_tex = Some(texture.clone());

        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

        // objects.push(sphere3);
        // objects.push(sphere2);
        // objects.push(sphere1);
        // objects.push(Box::new(triangle1));
        // objects.push(Box::new(triangle2));
        objects.push(Box::new(square));
        objects.push(Box::new(sphere1));

        // objects.push(Box::new(triangle));

        // located back of screen
        let light = Light { pos: cgmath::vec3(0.0, 1.0, 0.5) };

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

    pub fn tracy_ray(&self, ray: &Ray) -> cgmath::Vector3<f32> {
        let hit = self.find_closest_collision(ray);

        let mut color = cgmath::vec3(0.0, 0.0, 0.0);

        if hit.d >= 0.0 {
            if let Some(object) = hit.object {


                // color = self.get_ambient_color(object);

                // shadow 기능 비활성화
                // let dir_light = (self.light.pos - hit.point).normalize();
                //
                // let shadow_ray = Ray { start: hit.point + (dir_light * 1e-4f32), dir: dir_light };
                //
                // let shadow_hit = self.find_closest_collision(&shadow_ray);
                //
                // // 라이트가 그림자를 드리울 물체보다 가까이 있는 경우에도 (라이트가 가려지지 않는 경우에도) 그림자를 그리게 됩니다.
                // // 아래처럼 코드를 수정해주시면 조금 더 물리적으로 정확한 그림자를 표시할 수 있을 것 같습니다.
                // // FindClosestCollision(shadowRay).d > glm::length(light.pos - hit.point)
                // let light_hit = self.light.pos - hit.point;
                //
                // // 충돌이 없을 경우, shadow ray 가 빛을 향한다는 의미이기 때문에 기존처럼 color 계산
                // if shadow_hit.d < 0.0 || shadow_hit.d > light_hit.magnitude() {
                //     color = self.calculate_phong_model_color(&hit, &ray, &object);
                // }

                // Barycentric Coordinates 테스트 코드
                // match object {
                //     Object::Triangle(s) => {
                //         // interpolation
                //         let color0 = cgmath::vec3(1.0 ,0.0 ,0.0);
                //         let color1 = cgmath::vec3(0.0 ,1.0 ,0.0);
                //         let color2 = cgmath::vec3(0.0 ,0.0 ,1.0);
                //
                //         let w0 = hit.w.x;
                //         let w1 = hit.w.y;
                //         let w2 = 1.0 - w0 - w1;
                //
                //         // println!("{:} {:} {:}", w0, w1, w2);
                //
                //         color = color0 * w0 + color1 * w1 + color2 * w2;
                //     }
                //     _ => {}
                // }

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

                // texture calculation - ambient
                if amb_texture.is_some() {
                    color = amb.mul_element_wise(
                        amb_texture.as_ref().expect("fail to access the texture").get_sample_linear(&hit.uv)
                    );
                } else {
                    color = amb;
                }

                // texture calculation - diffuse
                if dif_texture.is_some() {
                    color += diffuse.mul_element_wise(
                        dif_texture.as_ref().expect("fail to access the texture").get_sample_linear(&hit.uv)
                    );
                } else {
                    color += diffuse;
                }

                color += specular;
            }
        }

        color
    }

    fn trace_ray_2x2(&self, eye_pos: &cgmath::Vector3<f32>, pixel_pos: &cgmath::Vector3<f32>, dx: f32, level: i32) -> cgmath::Vector3<f32> {
        if level == 0 {
            let ray = Ray { dir: (pixel_pos - eye_pos).normalize(), start: *pixel_pos };
            self.tracy_ray(&ray)
        } else {

            let sub_dx = 0.5 * dx;

            let mut pixel_color: cgmath::Vector3<f32> = cgmath::vec3(0.0, 0.0, 0.0);

            cgmath::vec3(
                pixel_pos.x - sub_dx * 0.5,
                pixel_pos.y - sub_dx * 0.5,
                pixel_pos.z
            );

            for j in 0..2 {
                for i in 0..2 {
                    let sub_pos = cgmath::vec3(
                        pixel_pos.x + (i as f32) * sub_dx,
                        pixel_pos.y + (j as f32) * sub_dx,
                        pixel_pos.z
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
                // let pixel_ray = Ray { dir: ray_dir, start: pixel_pos_world };
                // let color = self.tracy_ray(&pixel_ray);

                // super-sampling
                let color = self.trace_ray_2x2(&eye_pos, &pixel_pos_world, dx, 3);

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