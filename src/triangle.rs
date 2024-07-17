use cgmath::InnerSpace;
use wgpu::naga::VectorSize::Tri;
use crate::hit::{Hit, Hittable, LightProperty, Object, TextureProperty};
use crate::ray::Ray;
use crate::texture::Texture;

pub struct Triangle {
    pub v0: cgmath::Vector3<f32>,
    pub v1: cgmath::Vector3<f32>,
    pub v2: cgmath::Vector3<f32>,

    pub uv0: cgmath::Vector2<f32>,
    pub uv1: cgmath::Vector2<f32>,
    pub uv2: cgmath::Vector2<f32>,

    // light properties
    pub amb: cgmath::Vector3<f32>, // ambient
    pub diff: cgmath::Vector3<f32>, // diffuse
    pub spec: cgmath::Vector3<f32>, // specular
    pub ks: f32,
    pub alpha: f32,
}

impl Triangle {
    pub fn new(
        v0: cgmath::Vector3<f32>, v1: cgmath::Vector3<f32>, v2: cgmath::Vector3<f32>,
        uv0: cgmath::Vector2<f32>, uv1: cgmath::Vector2<f32>, uv2: cgmath::Vector2<f32>,
    ) -> Triangle {
        Triangle {
            v0,
            v1,
            v2,
            uv0,
            uv1,
            uv2,
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
        t: &mut f32,
        w0: &mut f32,
        w1: &mut f32
    ) -> bool {
        /*
         * 삼각형이 놓여있는 평면과 광선의 교점을 찾고,
         * 그 교점이 삼각형 안에 있는 밖에 있는지 판단
         */

        // 평면과 광선의 교차점 찾은 후, 삼각형 안에 있는지 계산
        let n = (v1 - v0).cross(v2 - v0).normalize();

        // 삼각형 뒷면 제거 (backface culling)
        if -dir.dot(n) < 0.0 {
            return false
        }

        // 평면과 광선이 수평에 매우 가깝다면 충돌하지 않는 것으로 판단
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

        // 작은 삼각형들의 3개의 normal 계산
        // 방향만 확인하면 되기 때문에 normalize 생략
        let cross0 = (p - v2).cross(v1 - v2);
        let cross1 = (p - v0).cross(v2 - v0);
        let cross2 = (v1 - v0).cross(p - v0);

        // cross product의 절대값으로 작은 삼각형들의 넓이 계산
        if cross0.dot(n) < 0.0 || cross1.dot(n) < 0.0 || cross2.dot(n) < 0.0 {
            return false;
        }

        // Barycentric coordinates 계산
        // cross product 의 절대값으로 작은 삼각형 넓이 계산
        let area0 = cross0.magnitude() * 0.5;
        let area1 = cross1.magnitude() * 0.5;
        let area2 = cross2.magnitude() * 0.5;

        let area_sum = area0 + area1 + area2;

        *point = p.clone();
        *face_normal = n.clone();
        *t = d;

        *w0 = area0 / area_sum;
        *w1 = area1 / area_sum;

        true
    }
}

impl Hittable for Triangle {
    fn intersect_ray_collision(&self, ray: &Ray) -> Hit {
        let mut hit = Hit::new(-1.0, cgmath::vec3(0.0, 0.0, 0.0), cgmath::vec3(0.0, 0.0, 0.0));

        let mut point = cgmath::vec3(0.0, 0.0, 0.0);
        let mut face_normal = cgmath::vec3(0.0, 0.0, 0.0);
        let mut t: f32 = -1.0;

        let mut w0: f32 = 0.0;
        let mut w1: f32 = 0.0;


        if self.intersect_ray_triangle(
            ray.start,
            ray.dir,
            self.v0,
            self.v1,
            self.v2,
            &mut point,
            &mut face_normal,
            &mut t,
            &mut w0,
            &mut w1
        ) {
            hit.d = t;
            hit.point = point;
            hit.normal = face_normal;

            // 텍스처 좌표
            hit.uv = self.uv0 * w0 + self.uv1 * w1 + self.uv2 * (1.0 - w0 - w1);

            // Barycentric coordinates 확인용
            // println!("{:} {:}", w0, w1);
            hit.w = cgmath::vec2(w0, w1);
        }

        hit
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

    fn get_texture_properties(&self) -> Option<Texture> {
        None
    }

    fn get_texture(&self) -> &Option<Texture> {
        &None
    }
}