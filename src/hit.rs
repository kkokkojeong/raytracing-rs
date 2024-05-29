use crate::sphere::Sphere;

pub struct Hit {
    pub d: f32, // distance from the start of ray
    pub point: cgmath::Vector3<f32>, // collision point
    pub normal: cgmath::Vector3<f32>, // normal vector of collision point

    // 나중에 물체의 재질 등을 가져오기 위한 포인터
    // 리팩토링 필요
    // pub object: Option<Sphere>,
    pub object: Sphere,
}

impl Hit {
    pub fn new(d: f32, point: cgmath::Vector3<f32>, normal: cgmath::Vector3<f32>) -> Self {
        Self {
            d,
            point: point.clone(),
            normal: normal.clone(),
            object: Sphere::new(cgmath::Vector3::new(0.0, 0.0, 0.0), 0.0)
        }
    }
}