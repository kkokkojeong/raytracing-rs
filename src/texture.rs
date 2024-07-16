use std::path::Path;
use image;
use image::GenericImageView;

#[derive(Clone)]
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

    pub fn get_sample_point(&self, /* uv: &cgmath::Vector3<f32> */) -> cgmath::Vector3<f32> {
        cgmath::Vector3::new(0.0, 0.0, 0.0)
    }
}