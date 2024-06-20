use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::square::Square;
use crate::triangle::Triangle;

pub enum Object {
    Triangle(Triangle),
    Sphere(Sphere),
    Square(Square),
}

pub struct Hit {
    pub d: f32, // distance from the start of ray
    pub point: cgmath::Vector3<f32>, // collision point
    pub normal: cgmath::Vector3<f32>, // normal vector of collision point

    // 나중에 물체의 재질 등을 가져오기 위한 포인터
    pub object: Option<Object>,
}

impl Hit {
    pub fn new(d: f32, point: cgmath::Vector3<f32>, normal: cgmath::Vector3<f32>) -> Self {
        Self {
            d,
            point,
            normal,
            object: None
        }
    }
}

// interface 같은 역할. 각 Object 에서 intersect_ray_collision 메소드 구현
pub trait Hittable {
    fn intersect_ray_collision(&self, ray: &Ray) -> Hit;
    fn as_object(&self) -> Object;
}