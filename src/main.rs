use image::{RgbImage, ImageBuffer};

fn main() {

    // Image
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    // RenderS
    let mut img_buf: RgbImage = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev()  {
        for i in 0..IMAGE_WIDTH {
            let r = i as f32/ (IMAGE_WIDTH - 1) as f32;
            let g = j as f32/ (IMAGE_HEIGHT - 1) as f32;
            let b = 0.25 as f32;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            let pixel = image::Rgb([ir, ig, ib]);
            img_buf.put_pixel(i, j, pixel);
        }
    }
    img_buf.save("render.png").unwrap();
}
