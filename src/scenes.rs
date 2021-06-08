use crate::objects::hittable::Hittable;
use std::sync::Arc;
use crate::objects::sphere::{Sphere, MovingSphere};
use crate::geometry::vector::{Point, Vector3};
use crate::textures::checkered::CheckeredTexture;
use crate::materials::lambertian::Lambertian;
use crate::textures::solid::SolidColor;
use crate::geometry::color::Color;
use crate::utils::{random_f32, random_f32_range};
use crate::materials::dielectric::Dielectric;
use crate::materials::metal::Metal;
use crate::textures::perlin::{NoiseTexture, Perlin};
use crate::textures::image::ImageTexture;
use crate::materials::light::DiffuseLight;
use crate::objects::rectangle::XYRect;

pub fn simple_light() -> Vec<Arc<dyn Hittable>> {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let perlin_material = Arc::new(Lambertian {
        albedo: Arc::new(NoiseTexture {
            noise: Perlin::new(),
            scale: 4.0,
        })
    });
    world.push(Arc::new(Sphere {
        center: Point { x: 0.0, y: -1000.0, z: 0.0 },
        radius: 1000.0,
        material: perlin_material.clone(),
    }));
    world.push(Arc::new(Sphere {
        center: Point { x: 0.0, y: 2.0, z: 0.0 },
        radius: 2.0,
        material: perlin_material,
    }));

    let diffuse_light = Arc::new(DiffuseLight {
        emit: Arc::new(SolidColor {
            color: Color { r: 4.0, g: 4.0, b: 4.0 }
        })
    });
    world.push(Arc::new(XYRect {
        x: (3.0, 5.0),
        y: (1.0, 3.0),
        k: -2.0,
        material: diffuse_light,
    }));

    return world;
}

pub fn earth() -> Vec<Arc<dyn Hittable>> {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let earth_texture = Arc::new(ImageTexture::new(String::from("/home/ravi/CLionProjects/RayTracer/src/earthmap.jpg")));
    let earth_surface = Arc::new(Lambertian {
        albedo: earth_texture,
    });
    let globe = Arc::new(Sphere {
        center: Point { x: 0.0, y: 0.0, z: 0.0 },
        radius: 2.0,
        material: earth_surface,
    });
    world.push(globe);

    return world;
}

pub fn random_spheres() -> Vec<Arc<dyn Hittable>> {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    // Ground
    world.push(Arc::new(Sphere {
        center: Point {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(CheckeredTexture {
                even: Arc::new(SolidColor {
                    color: Color { r: 0.2, g: 0.3, b: 0.1 }
                }),
                odd: Arc::new(SolidColor {
                    color: Color { r: 0.9, g: 0.9, b: 0.9 }
                }),
            }),
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
                    world.push(Arc::new(MovingSphere {
                        centre0: center,
                        center1: center + Vector3 {x: 0.0, y: random_f32() / 4.0, z: 0.0},
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        material: Arc::new(Lambertian {
                            albedo: Arc::new(SolidColor {
                                color,
                            }),
                        }),
                    }))
                } else if choose_mat < 0.95 {
                    let color = 0.5 * Color::random() + Color { r: 0.5, g: 0.5, b: 0.5 } ;
                    let fuzz = random_f32_range(0.0, 0.5);
                    world.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Metal {
                            color,
                            fuzz,
                        })
                    }))
                } else {
                    world.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Dielectric {
                            refractive_index: 1.5,
                        })
                    }));
                }
            }
        }
    }

    world.push(Arc::new(Sphere {
        center: Point {x: 0.0, y: 1.0, z: 0.0 },
        radius: 1.0,
        material: Arc::new(Dielectric {
            refractive_index: 1.5,
        })
    }));

    world.push(Arc::new(Sphere {
        center: Point {x: -4.0, y: 1.0, z: 0.0 },
        radius: 1.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(SolidColor {
                color: Color {
                    r: 0.4,
                    g: 0.2,
                    b: 0.1,
                }
            }),
        }),
    }));

    world.push(Arc::new(Sphere {
        center: Point {x: 4.0, y: 1.0, z: 0.0 },
        radius: 1.0,
        material: Arc::new(Metal {
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

pub fn two_checkered_spheres() -> Vec<Arc<dyn Hittable>> {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let checker = Arc::new(CheckeredTexture {
        even: Arc::new(SolidColor {
            color: Color {
                r: 0.2,
                g: 0.3,
                b: 0.1,
            }
        }),
        odd: Arc::new(SolidColor {
            color: Color {
                r: 0.9,
                g: 0.9,
                b: 0.9,
            }
        })
    });

    world.push(Arc::new(Sphere {
        center: Vector3 {
            x: 0.0,
            y: -10.0,
            z: 0.0
        },
        radius: 10.0,
        material: Arc::new(Lambertian {
            albedo: checker.clone(),
        })
    }));

    world.push(Arc::new(Sphere {
        center: Vector3 {
            x: 0.0,
            y: 10.0,
            z: 0.0
        },
        radius: 10.0,
        material: Arc::new(Lambertian {
            albedo: checker,
        })
    }));
    
    return world;
}

pub fn two_perlin_spheres() -> Vec<Arc<dyn Hittable>> {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let perlin_text = Arc::new(NoiseTexture {
        noise: Perlin::new(),
        scale: 4.0,
    });

    world.push(Arc::new(Sphere {
        center: Point {
            x: 0.0,
            y: -1000.0,
            z: 0.0
        },
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: perlin_text.clone(),
        }),
    }));

    world.push(Arc::new(Sphere {
        center: Point {
            x: 0.0,
            y: 2.0,
            z: 0.0
        },
        radius: 2.0,
        material: Arc::new(Lambertian {
            albedo: perlin_text,
        }),
    }));

    return world;
}
