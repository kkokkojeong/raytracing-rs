use crate::hit::{Hit, Hittable, LightProperty, Object, TextureProperty};
use crate::ray::Ray;
use crate::sphere::Sphere;
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
    pub fn new(
        v0: cgmath::Vector3<f32>, v1: cgmath::Vector3<f32>, v2: cgmath::Vector3<f32>, v3: cgmath::Vector3<f32>,
        uv0: cgmath::Vector2<f32>, uv1: cgmath::Vector2<f32>, uv2: cgmath::Vector2<f32>, uv3: cgmath::Vector2<f32>
    ) -> Square {
        Square {
            triangle1: Triangle::new(v0, v1, v2, uv0, uv1, uv2),
            triangle2: Triangle::new(v0, v2, v3, uv0, uv2, uv3),
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
            if hit1.d < hit2.d {
                hit1
            } else {
                hit2
            }
        } else if hit1.d >= 0.0 {
            hit1
        } else {
            hit2
        }
    }

    fn get_light_color_properties(&self) -> LightProperty {
        LightProperty {
            amb: self.amb,
            diff: self.diff,
            spec: self.spec,
            ks: self.ks,
            alpha: self.alpha
        }
    }

    fn get_ambient_texture(&self) -> &Option<Texture> {
        &self.amb_tex
    }

    fn get_diffuse_texture(&self) -> &Option<Texture> {
        &self.dif_tex
    }
}