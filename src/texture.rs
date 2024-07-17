use std::ops::Mul;
use std::path::Path;
use image;
use image::{GenericImageView, ImageBuffer, Rgb};

#[derive(Debug, Clone)]
pub struct Texture {
    pub width: i32,
    pub height: i32,
    pub channels: u8,
    pub image: Vec<u8>,
}

impl Texture {
    pub fn new(file_name: &str) -> Self {
        let img_path = Path::new(file_name);
        let img = image::open(&img_path).expect("Failed to open image");

        println!("image information: {}, {}, {}", img.width(), img.height(), img.color().channel_count());

        Texture {
            width: img.width() as i32,
            height: img.height() as i32,
            channels: img.color().channel_count(),
            image: img.into_bytes().clone()
        }

        // 가상의 이미지 생성 테스트 코드
        // let mut img = ImageBuffer::new(4, 4);
        // for j in 0..4 {
        //     for i in 0..4 {
        //         let mut rgb: cgmath::Vector3<f32>;
        //
        //         if i % 4 == 0 {
        //             rgb = cgmath::vec3(1.0, 0.0, 0.0).mul(1.0 + j as f32).mul(0.25);
        //         } else if i % 4 == 1 {
        //             rgb = cgmath::vec3(0.0, 1.0, 0.0).mul(1.0 + j as f32).mul(0.25);
        //         } else if i % 4 == 2 {
        //             rgb = cgmath::vec3(0.0, 0.0, 1.0).mul(1.0 + j as f32).mul(0.25);
        //         } else {
        //             rgb = cgmath::vec3(1.0, 1.0, 1.0).mul(1.0 + j as f32).mul(0.25);
        //         }
        //
        //         let r = (rgb.x * 255.0).floor() as u8;
        //         let g = (rgb.y * 255.0).floor() as u8;
        //         let b = (rgb.z * 255.0).floor() as u8;
        //
        //
        //         let pixel = Rgb([r, g, b]);
        //         img.put_pixel(i, j, pixel);
        //     }
        // }
        //
        // Texture {
        //     width: 4,
        //     height: 4,
        //     channels: 3,
        //     image: img.into_raw(),
        // }
    }

    // Nearest sampling
    pub fn get_sample_point(&self, uv: &cgmath::Vector2<f32>) -> cgmath::Vector3<f32> {
        // 텍스춰 좌표의 범위 uv [0.0, 1.0] x [0.0, 1.0]
        // 이미지 좌표의 범위 xy [-0.5, width - 1 + 0.5] x [-0.5, height - 1 + 0.5]
        // 배열 인덱스의 정수 범위 ij [0, width-1] x [0, height - 1]
        let w = self.width as f32;
        let h = self.height as f32;
        let xy = cgmath::vec2(uv.x * w - 0.5, uv.y * h - 0.5);

        let i = xy.x.round() as i32;
        let j = xy.y.round() as i32;

        self.clamped(i, j)
    }

    // bi-linear sampling
    pub fn get_sample_linear(&self, uv: &cgmath::Vector2<f32>) -> cgmath::Vector3<f32> {
        // 텍스춰 좌표의 범위 uv [0.0, 1.0] x [0.0, 1.0]
        // 이미지 좌표의 범위 xy [-0.5, width - 1 + 0.5] x [-0.5, height - 1 + 0.5]
        // std::cout << floor(-0.3f) << " " << int(-0.3f) << std::endl; // -1 0
        let w = self.width as f32;
        let h = self.height as f32;
        let xy = cgmath::vec2(uv.x * w - 0.5, uv.y * h - 0.5);

        let i = xy.x as i32;
        let j = xy.y as i32;
        
        let dx = xy.x - (i as f32);
        let dy = xy.y - (j as f32);
        
        self.bilinear_interpolation(
            dx, dy,
            self.wrapped(i, j),
            self.wrapped(i + 1, j),
            self.wrapped(i, j + 1),
            self.wrapped(i + 1, j + 1)
        )
    }

    fn bilinear_interpolation(
        &self,
        dx: f32,
        dy: f32,
        c00: cgmath::Vector3<f32>,
        c10: cgmath::Vector3<f32>,
        c01: cgmath::Vector3<f32>,
        c11: cgmath::Vector3<f32>,
    ) -> cgmath::Vector3<f32> {
        let a = c00.mul(1.0 - dx) + c10.mul(dx);
        let b = c01.mul(1.0 - dx) + c11.mul(dx);

        a.mul(1.0 - dy) + b.mul(dy)
    }


    fn clamped(&self, i: i32, j: i32) -> cgmath::Vector3<f32> {
        let x = i.clamp(0, self.width - 1) as usize;
        let y = j.clamp(0, self.height - 1) as usize;

        let index = (x + (self.width as usize) * y) * self.channels as usize;

        let r: f32 = self.image[index + 0] as f32 / 255.0;
        let g: f32 = self.image[index + 1] as f32 / 255.0;
        let b: f32 = self.image[index + 2] as f32 / 255.0;

        cgmath::vec3(r, g, b)
    }

    fn wrapped(&self, i: i32, j: i32) -> cgmath::Vector3<f32> {
        let w = self.width as usize;
        let h = self.height as usize;
        let mut x = (i as usize) % w;
        let mut y = (j as usize) % h;

        if i < 0 {
            x += w
        }
        if j < 0 {
            y += h;
        }

        let index = (x + (self.width as usize) * y) * self.channels as usize;

        let r: f32 = self.image[index + 0] as f32 / 255.0;
        let g: f32 = self.image[index + 1] as f32 / 255.0;
        let b: f32 = self.image[index + 2] as f32 / 255.0;

        cgmath::vec3(r, g, b)
    }
}