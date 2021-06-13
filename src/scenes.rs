use std::sync::Arc;
use crate::objects::hittable::Hittable;
use crate::objects::sphere::{Sphere, MovingSphere};
use crate::geometry::vector::{Point, Vector3};
use crate::materials::lambertian::Lambertian;
use crate::textures::solid::SolidColor;
use crate::geometry::color::Color;
use crate::textures::perlin::{NoiseTexture, Perlin};
use crate::materials::light::DiffuseLight;
use crate::objects::rectangle::{XYRect, YZRect, XZRect};
use crate::objects::boxes::AxisAlignedBox;
use crate::objects::instances::{RotateY, Translate};
use crate::utils::random_f32_range;
use crate::world::bvh_node::BVHNode;
use crate::materials::dielectric::Dielectric;
use crate::materials::metal::Metal;
use crate::objects::medium::ConstMedium;
use crate::textures::image::ImageTexture;

pub fn final_scene() -> Vec<Arc<dyn Hittable>> {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();


    // Ground
    let mut boxes1: Vec<Arc<dyn Hittable>> = Vec::new();
    let ground = Arc::new(Lambertian {
        albedo: Arc::new(SolidColor { color: Color { r: 0.48, g: 0.83, b: 0.53 } })
    });
    const BOXES_PER_SIDE: u32 = 20;
    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0 as f32;
            let x1 = x0 + w;
            let y1 = random_f32_range(1.0, 101.0);
            let z1 = z0 + w;
            boxes1.push(Arc::new(AxisAlignedBox::new(Point { x: x0, y: y0, z: z0 }, Point {x: x1, y: y1, z: z1 }, ground.clone() )));
        }
    }
    world.push(BVHNode::create_tree(&mut boxes1, 0.0, 1.0));


    // Light
    let light = Arc::new(DiffuseLight {
        emit: Arc::new(SolidColor { color: Color { r: 7.0, g: 7.0, b: 7.0 } })
    });
    world.push(Arc::new(XZRect {
        x: (123.0, 423.0),
        z: (147.0, 412.0),
        k: 554.0,
        material: light
    }));


    // Moving Sphere
    let center1 = Point { x: 400.0, y: 400.0, z: 200.0 };
    let center2 = center1 + Point { x: 30.0, y: 0.0, z: 0.0 };
    let moving_sphere_mat = Arc::new(Lambertian {
        albedo: Arc::new(SolidColor { color: Color { r: 0.7, g: 0.3, b: 0.1 } })
    });
    world.push(Arc::new(MovingSphere {
        centre0: center1,
        center1: center2,
        time0: 0.0,
        time1: 1.0,
        radius: 50.0,
        material: moving_sphere_mat
    }));


    // Glass Sphere
    world.push(Arc::new(Sphere {
        center: Point { x: 260.0, y: 150.0, z: 45.0 },
        radius: 50.0,
        material: Arc::new(Dielectric { refractive_index: 1.5 })
    }));


    // Metal Sphere
    world.push(Arc::new(Sphere {
        center: Point { x: 0.0, y: 150.0, z: 145.0 },
        radius: 50.0,
        material: Arc::new(Metal { color: Color { r: 0.8, g: 0.8, b: 0.9 }, fuzz: 1.0 })
    }));


    // Shiny Spheres
    let mut boundary = Arc::new(Sphere {
        center: Point { x: 360.0, y: 150.0, z: 145.0 },
        radius: 70.0,
        material: Arc::new(Dielectric { refractive_index: 1.5 })
    });
    world.push(boundary.clone());
    world.push(Arc::new(ConstMedium::new(boundary, 0.2, Color { r: 0.2, g: 0.4, b: 0.9 })));
    boundary = Arc::new(Sphere {
        center: Point { x: 0.0, y: 0.0, z: 0.0 },
        radius: 5000.0,
        material: Arc::new(Dielectric { refractive_index: 1.5 })
    });
    world.push(Arc::new(ConstMedium::new(boundary, 0.0001, Color::WHITE)));


    // Earth
    let e_mat = Arc::new(Lambertian { albedo: Arc::new(ImageTexture::new(String::from("/home/ravi/CLionProjects/RayTracer/src/earthmap.jpg"))) });
    world.push(Arc::new(Sphere {
        center: Point { x: 400.0, y: 200.0, z: 400.0 },
        radius: 100.0,
        material: e_mat
    }));

    let per_text = Arc::new(NoiseTexture {
        noise: Perlin::new(),
        scale: 0.1
    });
    world.push(Arc::new(Sphere {
        center: Point { x: 220.0, y: 280.0, z: 300.0 },
        radius: 80.0,
        material: Arc::new(Lambertian {
            albedo: per_text
        })
    }));


    // Cubical cluster of spheres
    let mut boxes2: Vec<Arc<dyn Hittable>> = Vec::new();
    let white = Arc::new(Lambertian { albedo: Arc::new(SolidColor { color: Color { r: 0.73, g: 0.73, b: 0.73 } }) });
    let ns = 1000;
    for _j in 0..ns {
        boxes2.push(Arc::new(Sphere {
            center: Point { x: random_f32_range(0.0, 165.0), y: random_f32_range(0.0, 165.0), z: random_f32_range(0.0, 165.0) },
            radius: 10.0,
            material: white.clone(),
        }));
    }
    world.push(Arc::new(Translate {
        object: Arc::new(RotateY::new(BVHNode::create_tree(&mut boxes2, 0.0, 1.0), 15.0)),
        offset: Vector3 {x: -100.0, y: 270.0, z: 395.0}
    }));

    return world;
}