use crate::hit::{Hit, Hittable, Object};
use crate::ray::Ray;
use crate::texture::Texture;
use crate::triangle::Triangle;

pub struct Square {
    pub triangle1: Triangle,
    pub triangle2: Triangle,

    // light properties
    pub amb: cgmath::Vector3<f32>, // ambient
    pub diff: cgmath::Vector3<f32>, // diffuse
    pub spec: cgmath::Vector3<f32>, // specular
    pub ks: f32,
    pub alpha: f32,

    pub amb_tex: Option<Texture>,
    pub dif_tex: Option<Texture>,
}

impl Square {
    pub fn new(v0: cgmath::Vector3<f32>, v1: cgmath::Vector3<f32>, v2: cgmath::Vector3<f32>, v3: cgmath::Vector3<f32>) -> Square {
        Square {
            triangle1: Triangle::new(v0, v1, v2),
            triangle2: Triangle::new(v0, v2, v3),
            amb: cgmath::Vector3::new(0.0, 0.0, 0.0),
            diff: cgmath::Vector3::new(0.0, 0.0, 0.0),
            spec: cgmath::Vector3::new(0.0, 0.0, 0.0),
            ks: 0.0,
            alpha: 0.0,
            amb_tex: None,
            dif_tex: None,
        }
    }
}

impl Hittable for Square {
    fn intersect_ray_collision(&self, ray: &Ray) -> Hit {
        let hit1 = self.triangle1.intersect_ray_collision(ray);
        let hit2 = self.triangle2.intersect_ray_collision(ray);

        if hit1.d >= 0.0 && hit2.d >= 0.0 {
            // rust 는 삼항 연산자 사용 불가
            if hit1.d < hit2.d { hit1 } else { hit2 }
        } else if hit1.d >= 0.0 {
            hit1
        } else {
            hit2
        }
    }

    fn as_object(&self) -> Object {
        let tri1 = self.triangle1.clone();
        let tri2 = self.triangle2.clone();

        let square = Square {
            triangle1: tri1,
            triangle2: tri2,
            amb: self.amb,
            diff: self.diff,
            spec: self.spec,
            ks: self.ks,
            alpha: self.alpha,
            amb_tex: None,
            dif_tex: None,
        };

        Object::Square(square)
    }

    fn has_ambient_texture(&self) -> bool {
        self.amb_tex.is_some()
    }

    fn has_diffuse_texture(&self) -> bool {
        self.dif_tex.is_some()
    }
}