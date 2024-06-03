use cgmath::InnerSpace;
use crate::hit::{Hit, Hittable, Object};
use crate::ray::Ray;

pub struct Triangle {
    pub v0: cgmath::Vector3<f32>,
    pub v1: cgmath::Vector3<f32>,
    pub v2: cgmath::Vector3<f32>,

    // light properties
    pub amb: cgmath::Vector3<f32>, // ambient
    pub diff: cgmath::Vector3<f32>, // diffuse
    pub spec: cgmath::Vector3<f32>, // specular
    pub ks: f32,
    pub alpha: f32,
}

impl Triangle {
    pub fn new(v0: cgmath::Vector3<f32>, v1: cgmath::Vector3<f32>, v2: cgmath::Vector3<f32>) -> Triangle {
        Triangle {
            v0,
            v1,
            v2,
            amb: cgmath::Vector3::new(0.0, 0.0, 0.0),
            diff: cgmath::Vector3::new(0.0, 0.0, 0.0),
            spec: cgmath::Vector3::new(0.0, 0.0, 0.0),
            ks: 0.0,
            alpha: 0.0
        }
    }

    // https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/ray-triangle-intersection-geometric-solution.html
    fn intersect_ray_triangle(
        &self,
        start: cgmath::Vector3<f32>,
        dir: cgmath::Vector3<f32>,
        v0: cgmath::Vector3<f32>,
        v1: cgmath::Vector3<f32>,
        v2: cgmath::Vector3<f32>,
        point: &mut cgmath::Vector3<f32>,
        face_normal: &mut cgmath::Vector3<f32>,
        t: &mut f32
    ) -> bool {
        // 평면과 광선의 교차점 찾은 후, 삼각형 안에 있는지 계산
        let n = (v1 - v0).cross(v2 - v0).normalize();

        if -dir.dot(n) < 0.0 {
            return false
        }

        let deno = dir.dot(n);

        if deno.abs() < 1e-2f32 {
            return false
        }

        // 광선과 평면의 충돌 위치 계산
        let d =  (v0.dot(n) - start.dot(n)) / deno;
        let p = start + (d * dir);

        // 광선의 시작점 이전에 충돌한다면 렌더링할 필요 없음
        if d < 0.0 {
            return false;
        }

        let normal0 = (p - v2).cross(v1 - v2).normalize();
        let normal1 = (p - v0).cross(v2 - v0).normalize();
        let normal2 = (v1 - v0).cross(p - v0).normalize();

        // cross product의 절대값으로 작은 삼각형들의 넓이 계산
        if normal0.dot(n) < 0.0 || normal1.dot(n) < 0.0 || normal2.dot(n) < 0.0 {
            return false;
        }

        *point = p.clone();
        *face_normal = n.clone();
        *t = d;

        // println!("{:}", t);

        true
    }
}

impl Hittable for Triangle {
    fn intersect_ray_collision(&self, ray: &Ray) -> Hit {
        let mut hit = Hit::new(-1.0, cgmath::vec3(0.0, 0.0, 0.0), cgmath::vec3(0.0, 0.0, 0.0));

        let mut point = cgmath::vec3(0.0, 0.0, 0.0);
        let mut face_normal = cgmath::vec3(0.0, 0.0, 0.0);
        let mut t: f32 = -1.0;


        if self.intersect_ray_triangle(
            ray.start,
            ray.dir,
            self.v0,
            self.v1,
            self.v2,
            &mut point,
            &mut face_normal,
            &mut t
        ) {
            hit.d = t;
            hit.point = point;
            hit.normal = face_normal;
        }

        hit
    }

    fn as_object(&self) -> Object {
        let mut triangle = Triangle::new(self.v0, self.v1, self.v2);
        triangle.amb = self.amb;
        triangle.diff = self.diff;
        triangle.spec = self.spec;
        triangle.ks = self.ks;
        triangle.alpha = self.alpha;

        Object::Triangle(triangle)
    }
}