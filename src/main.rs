use std::sync::{Arc, Mutex};
use std::thread;

use image::{ImageBuffer, Rgb, RgbImage};

use geometry::color::Color;
use geometry::pdf::{CosinePDF, HittablePDF, MixturePDF, PDF};
use geometry::ray::Ray;
use geometry::vector::{Point, Vector3};
use materials::light::DiffuseLight;
use objects::hittable::Hittable;
use objects::rectangle::XZRect;
use scenes::cornell_box;
use utils::INF_F32;
use utils::random_f32;
use world::bvh_node::BVHNode;
use world::camera::Camera;

use crate::objects::sphere::Sphere;
use crate::world::hittable_list::HittableList;

mod geometry;
mod objects;
mod world;
mod materials;
mod utils;
mod textures;
mod scenes;

fn ray_color(ray: Ray, background: Color, world: Arc<dyn Hittable>, lights: Arc<dyn Hittable>, depth: u32) -> Color {
    if depth == 0 {
        return Color::BLACK;
    }

    let opt_hit_rec = world.hit(ray, 0.01, INF_F32);
    if opt_hit_rec.is_none() {
        return background;
    }
    let hit_rec = opt_hit_rec.unwrap();

    let emitted = hit_rec.material.emitted(ray, &hit_rec, hit_rec.u, hit_rec.v, hit_rec.intersection);

    let opt_scatter_rec = hit_rec.material.scatter(ray, &hit_rec);
    if opt_scatter_rec.is_none() {
        return emitted;
    }
    let scatter_rec = opt_scatter_rec.unwrap();

    if scatter_rec.specular_ray.is_some() {
        return scatter_rec.attenuation * ray_color(scatter_rec.specular_ray.unwrap(), background, world, lights, depth - 1);
    }
    let light_pdf: Arc<dyn PDF> = Arc::new(HittablePDF {
        o: hit_rec.intersection,
        ptr: lights.clone(),
    });
    let mix_pdf = MixturePDF {
        ptr: [light_pdf, scatter_rec.pdf_ptr.unwrap()]
    };

    let scattered = Ray {
        origin: hit_rec.intersection,
        direction: mix_pdf.generate().direction(),
        time: ray.time,
    };
    let pdf_val = mix_pdf.value(scattered.direction);

    return emitted + (hit_rec.material.scattering_pdf(ray, &hit_rec, scattered) / pdf_val) *
        scatter_rec.attenuation * ray_color(scattered, background, world, lights, depth - 1);
}

struct ImageBlockInfo {
    start_row: u32,
    end_row: u32,
    image_height: u32,
    image_width: u32,
    spp: u32,
    max_depth: u32,
    image_block: Vec<Vec<Rgb<u8>>>,
}

fn process_block(mut block_info: ImageBlockInfo, image_blocks: Arc<Mutex<Vec<ImageBlockInfo>>>, camera: Camera, world: Arc<dyn Hittable>, lights: Arc<dyn Hittable>, background: Color) {
    for j in block_info.start_row..block_info.end_row {
        let mut row: Vec<Rgb<u8>> = Vec::with_capacity(block_info.image_width as usize);
        for i in 0..block_info.image_width {
            let mut pixel_color = Color { r: 0.0, g: 0.0, b: 0.0 };
            for _ in 0..block_info.spp {
                let u = (i as f32 + random_f32()) / (block_info.image_width - 1) as f32;
                let v = (j as f32 + random_f32()) / (block_info.image_height - 1) as f32;

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(ray, background, world.clone(), lights.clone(), block_info.max_depth);
            }
            row.push(pixel_color.get_pixel(block_info.spp));
        }
        block_info.image_block.push(row);
    }

    let mut image = image_blocks.lock().unwrap();
    image.push(block_info);
}

fn main() {
    // Image
    const IMAGE_WIDTH: u32 = 800;
    const IMAGE_HEIGHT: u32 = 800;
    const ASPECT_RATIO: f32 = IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32;
    const SAMPLES_PER_PIXEL: u32 = 1000;
    const MAX_DEPTH: u32 = 25;


    //Camera
    let look_from = Point { x: 278.0, y: 278.0, z: -800.0 };
    let look_at = Point { x: 278.0, y: 278.0, z: 0.0 };
    let v_up = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
    let v_fov = 40.0;
    let aperture = 0.0;
    let focus_dist = 10.0;

    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        v_fov,
        ASPECT_RATIO,
        aperture,
        focus_dist,
        0.0,
        1.0,
    );


    // World
    let mut world = cornell_box();
    let world = BVHNode::create_tree(&mut world, 0.0, 1.0);
    let background = Color::BLACK;
    let mut lights = HittableList::new();
    lights.add(Arc::new(XZRect {
        x: (213.0, 343.0),
        z: (227.0, 332.0),
        k: 554.0,
        material: Arc::new(DiffuseLight::new(Color::WHITE)),
    }));
    lights.add(Arc::new(Sphere {
        center: Point { x: 190.0, y: 90.0, z: 190.0 },
        radius: 90.0,
        material: Arc::new(DiffuseLight::new(Color::WHITE)),
    }));
    let light_list = Arc::new(lights);

    // Render
    const NTHREADS: u32 = 10;
    let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();
    let image_blocks: Arc<Mutex<Vec<ImageBlockInfo>>> = Arc::new(Mutex::new(Vec::new()));

    let block_size = IMAGE_HEIGHT / NTHREADS;
    let end_block_size = block_size + (IMAGE_HEIGHT % NTHREADS);

    for i in 0..NTHREADS {
        let block_info = ImageBlockInfo {
            start_row: i * block_size,
            end_row: i * block_size + (if i == NTHREADS - 1 { end_block_size } else { block_size }),
            image_height: IMAGE_HEIGHT,
            image_width: IMAGE_WIDTH,
            spp: SAMPLES_PER_PIXEL,
            max_depth: MAX_DEPTH,
            image_block: Vec::with_capacity(block_size as usize),
        };

        let camera_new = camera.clone();
        let image_blocks_new = image_blocks.clone();
        let world_new = world.clone();
        let lights_new = light_list.clone();

        let handle = thread::spawn(move || {
            process_block(block_info, image_blocks_new, camera_new, world_new, lights_new, background);
        });
        threads.push(handle);
    }

    for thread in threads.into_iter() {
        thread.join().unwrap();
    }

    let final_blocks = image_blocks.lock().unwrap();
    let mut img_buf: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for block in final_blocks.iter() {
        for y in 0..block.image_block.len() {
            for x in 0..block.image_block[0].len() {
                let u = x as u32;
                let v = block.start_row + y as u32;
                img_buf.put_pixel(u, v, block.image_block[y][x]);
            }
        }
    }
    img_buf.save("render.png").unwrap();
}
