use image::RgbImage;

use crate::geometry::color::Color;
use crate::geometry::vector::Point;
use crate::textures::texture::Texture;

pub struct ImageTexture {
    pub img_data: RgbImage,
    pub width: u32,
    pub height: u32,
    pub bytes_per_scanline: u32,
}

impl ImageTexture {
    const BYTES_PER_PIXEL: u32 = 3;

    pub fn new(path: String) -> Self {
        let img = image::open(path).unwrap().to_rgb8();
        let width = img.width();
        let height = img.height();

        return Self {
            img_data: img,
            width,
            height,
            bytes_per_scanline: width * Self::BYTES_PER_PIXEL,
        };
    }
}

impl Texture for ImageTexture {
    fn color(&self, u: f32, v: f32, _point: Point) -> Color {
        let _u = u.clamp(0.0, 1.0);
        let _v = 1.0 - v.clamp(0.0, 1.0);

        let mut i = (_u * self.width as f32) as i32;
        let mut j = (_v * self.height as f32) as i32;

        if i >= self.width as i32 {
            i = self.width as i32 - 1;
        }
        if j >= self.height as i32 {
            j = self.height as i32 - 1;
        }

        const COLOR_SCALE: f32 = 1.0 / 255.0;
        let pixel = self.img_data.get_pixel(i as u32, j as u32).clone();

        return Color {
            r: pixel[0] as f32 * COLOR_SCALE,
            g: pixel[1] as f32 * COLOR_SCALE,
            b: pixel[2] as f32 * COLOR_SCALE,
        };
    }
}
