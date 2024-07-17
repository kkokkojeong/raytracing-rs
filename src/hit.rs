use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::square::Square;
use crate::texture::Texture;
use crate::triangle::Triangle;

pub enum Object {
    Triangle(Triangle),
    Sphere(Sphere),
    Square(Square),
}

pub struct Hit<'a> {
    pub d: f32, // distance from the start of ray
    pub point: cgmath::Vector3<f32>, // collision point
    pub normal: cgmath::Vector3<f32>, // normal vector of collision point

    pub w: cgmath::Vector2<f32>, // 삼각형의 barycentric coordinates 저장 변수 (임시)

    pub uv: cgmath::Vector2<f32>, // texture coordinates

    // 나중에 물체의 재질 등을 가져오기 위한 포인터
    pub object: Option<&'a dyn Hittable>
}

pub struct LightProperty {
    pub amb: cgmath::Vector3<f32>,
    pub diff: cgmath::Vector3<f32>,
    pub spec: cgmath::Vector3<f32>,
    pub ks: f32,
    pub alpha: f32,
}

pub struct TextureProperty {
    pub amb_tex: Option<Texture>,
    pub dif_tex: Option<Texture>,
}

impl Hit<'_> {
    pub fn new(d: f32, point: cgmath::Vector3<f32>, normal: cgmath::Vector3<f32>) -> Self {
        Self {
            d,
            point,
            normal,
            w: cgmath::vec2(0.0, 0.0),
            uv: cgmath::vec2(0.0, 0.0),
            object: None
        }
    }
}

// interface 같은 역할. 각 Object 에서 intersect_ray_collision 메소드 구현
pub trait Hittable {
    fn intersect_ray_collision(&self, ray: &Ray) -> Hit;
    fn get_light_color_properties(&self) -> LightProperty;
    fn get_texture_properties(&self) -> Option<Texture>;
    fn get_texture(&self) -> &Option<Texture>;
}