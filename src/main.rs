use image::{ImageBuffer, RgbImage};

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
use utils::random_f32;
use utils::INF_F32;
use crate::materials::material::Material;
use crate::utils::random_f32_range;

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

fn main() {
    // Image
    const IMAGE_WIDTH: u32 = 1200;
    const IMAGE_HEIGHT: u32 = 800;
    const ASPECT_RATIO: f32 = IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32;
    const SAMPLES_PER_PIXEL: u32 = 10;
    const MAX_DEPTH: u32 = 25;

    // World
    let world = scene();

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
    let mut img_buf: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color { r: 0.0, g: 0.0, b: 0.0 };
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + random_f32()) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + random_f32()) / (IMAGE_HEIGHT - 1) as f32;

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(ray, &world, MAX_DEPTH);
            }
            img_buf.put_pixel(
                i, IMAGE_HEIGHT - 1 - j,
                pixel_color.get_pixel(SAMPLES_PER_PIXEL),
            );
        }
    }
    img_buf.save("render.png").unwrap();
}
