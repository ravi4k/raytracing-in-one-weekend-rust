use image::{ImageBuffer, RgbImage, Rgb};

mod geometry;
use geometry::color::Color;
use geometry::ray::Ray;
use geometry::vector::{Point, Vector3};

mod objects;
use objects::hittable::HittableList;
use objects::sphere::Sphere;

mod world;
use world::camera::Camera;

mod materials;
use materials::lambertian::Lambertian;
use materials::metal::Metal;
use materials::dielectric::Dielectric;

mod utils;
use utils::{random_f32, random_f32_range};
use utils::INF_F32;

use std::thread;
use std::sync::{Mutex, Arc};
use std::borrow::Borrow;

fn ray_color(ray: Ray, world: &HittableList, depth: u32) -> Color {
    if depth == 0 {
        return Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
    }

    let hit_rec = world.hit(&ray, 0.001, INF_F32);
    if hit_rec.object.is_some() {
        let color = hit_rec.object.unwrap().color();
        let scattered = hit_rec.object.unwrap().scatter(ray.direction, hit_rec);
        return color * ray_color(scattered, world, depth - 1);
    }

    let t = 0.5 * (ray.direction.y + 1.0);
    (1.0 - t) * Color { r: 1.0, g: 1.0, b: 1.0, } + t * Color { r: 0.5, g: 0.7, b: 1.0, }
}

fn scene() -> HittableList {
    let mut world = HittableList {
        objects: Vec::new(),
    };

    // Ground
    world.add(Box::new(Sphere {
        center: Point {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        material: Box::new(Lambertian {
            color: Color {
                r: 0.5,
                g: 0.5,
                b: 0.5,
            }
        }),
    }));

    // Spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f32();
            let center = Point { x: a as f32 + 0.9 * random_f32(), y: 0.2, z: b as f32 + 0.9 * random_f32() };

            if (center - Point { x: 4.0, y: 0.2, z: 0.0 }).length() > 0.9 {
                if choose_mat < 0.8 {
                    let color = Color::random() * Color::random();
                    world.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Lambertian {
                            color,
                        })
                    }))
                } else if choose_mat < 0.95 {
                    let color = 0.5 * Color::random() + Color { r: 0.5, g: 0.5, b: 0.5 } ;
                    let fuzz = random_f32_range(0.0, 0.5);
                    world.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Metal {
                            color,
                            fuzz,
                        })
                    }))
                } else {
                    world.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Dielectric {
                            refractive_index: 1.5,
                        })
                    }));
                }
            }
        }
    }

    world.add(Box::new(Sphere {
        center: Point {x: 0.0, y: 1.0, z: 0.0 },
        radius: 1.0,
        material: Box::new(Dielectric {
            refractive_index: 1.5,
        })
    }));

    world.add(Box::new(Sphere {
        center: Point {x: -4.0, y: 1.0, z: 0.0 },
        radius: 1.0,
        material: Box::new(Lambertian {
            color: Color {
                r: 0.4,
                g: 0.2,
                b: 0.1
            }
        })
    }));

    world.add(Box::new(Sphere {
        center: Point {x: 4.0, y: 1.0, z: 0.0 },
        radius: 1.0,
        material: Box::new(Metal {
            color: Color {
                r: 0.7,
                g: 0.6,
                b: 0.5
            },
            fuzz: 0.0,
        })
    }));

    world
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

fn process_block(mut block_info: ImageBlockInfo, image_blocks: Arc<Mutex<Vec<ImageBlockInfo>>>, camera: Camera, world: Arc<HittableList>) {
    for j in block_info.start_row..block_info.end_row {
        let mut row: Vec<Rgb<u8>> = Vec::with_capacity(block_info.image_width as usize) ;
        for i in 0..block_info.image_width {
            let mut pixel_color = Color { r: 0.0, g: 0.0, b: 0.0 };
            for _ in 0..block_info.spp {
                let u = (i as f32 + random_f32()) / (block_info.image_width - 1) as f32;
                let v = (j as f32 + random_f32()) / (block_info.image_height - 1) as f32;

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(ray, &world, block_info.max_depth);
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
    const IMAGE_WIDTH: u32 = 1200;
    const IMAGE_HEIGHT: u32 = 800;
    const ASPECT_RATIO: f32 = IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32;
    const SAMPLES_PER_PIXEL: u32 = 50;
    const MAX_DEPTH: u32 = 50;

    // World
    let world = Arc::new(scene());

    //Camera
    let look_from = Point { x: 13.0, y: 2.0, z: 3.0 };
    let look_at = Point { x: 0.0, y: 0.0, z: 0.0 };
    let v_up = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
    let v_fov = 20.0;
    let aperture = 0.1;
    let focus_dist = 10.0;

    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        v_fov,
        ASPECT_RATIO,
        aperture,
        focus_dist,
    );

    // Render
    const NTHREADS: u32 = 10;
    let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();
    let mut image_blocks: Arc<Mutex<Vec<ImageBlockInfo>>> = Arc::new(Mutex::new(Vec::new()));

    let block_size = IMAGE_HEIGHT / NTHREADS;
    let end_block_size = block_size + (IMAGE_HEIGHT % NTHREADS);

    for i in 0..NTHREADS {
        let block_info = ImageBlockInfo {
            start_row: i * block_size,
            end_row: i * block_size + ( if i == NTHREADS - 1 { end_block_size } else { block_size } ),
            image_height: IMAGE_HEIGHT,
            image_width: IMAGE_WIDTH,
            spp: SAMPLES_PER_PIXEL,
            max_depth: MAX_DEPTH,
            image_block: Vec::with_capacity(block_size as usize),
        };

        let camera_new = camera.clone();
        let image_blocks_new = image_blocks.clone();
        let world_new = world.clone();

        let handle = thread::spawn(|| {
            process_block(block_info, image_blocks_new, camera_new, world_new);
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
