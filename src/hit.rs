pub struct Hit {
    d: f32, // distance from the start of ray
    point: cgmath::Point3<f32>, // collision point
    normal: cgmath::Vector3<f32>, // normal vector of collision point
}