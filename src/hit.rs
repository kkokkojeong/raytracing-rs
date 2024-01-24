pub struct Hit {
    pub d: f32, // distance from the start of ray
    pub point: cgmath::Vector3<f32>, // collision point
    pub normal: cgmath::Vector3<f32>, // normal vector of collision point
}