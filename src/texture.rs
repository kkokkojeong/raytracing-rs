use std::path::Path;
use image;
use image::GenericImageView;

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

    fn clamped(&self, i: i32, j: i32) -> cgmath::Vector3<f32> {
        let x = i.clamp(0, self.width - 1) as usize;
        let y = j.clamp(0, self.height - 1) as usize;

        let index = (x + (self.width as usize) * y) * self.channels as usize;

        let r: f32 = self.image[index + 0] as f32 / 255.0;
        let g: f32 = self.image[index + 1] as f32 / 255.0;
        let b: f32 = self.image[index + 2] as f32 / 255.0;

        cgmath::vec3(r, g, b)
    }
}