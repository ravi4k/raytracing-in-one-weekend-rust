use std::sync::Arc;
use crate::objects::hittable::{Hittable, FlipFace};
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

pub fn cornell_box() -> Vec<Arc<dyn Hittable>> {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let red_material = Arc::new(Lambertian::new(Color { r: 0.65, g: 0.05, b: 0.05 }));
    let white_material = Arc::new(Lambertian::new(Color { r: 0.73, g: 0.73, b: 0.73 }));
    let green_material = Arc::new(Lambertian::new(Color { r: 0.12, g: 0.45, b: 0.15 }));
    let light = Arc::new(DiffuseLight::new(Color {r: 15.0, g: 15.0, b: 15.0 }));

    world.push(Arc::new(YZRect {
        y: (0.0, 555.0),
        z: (0.0, 555.0),
        k: 555.0,
        material: green_material
    }));
    world.push(Arc::new(YZRect {
        y: (0.0, 555.0),
        z: (0.0, 555.0),
        k: 0.0,
        material: red_material
    }));
    world.push(Arc::new(FlipFace {
        object: Arc::new(XZRect {
            x: (213.0, 343.0),
            z: (227.0, 332.0),
            k: 554.0,
            material: light
        })
    }));
    world.push(Arc::new(XZRect {
        x: (0.0, 555.0),
        z: (0.0, 555.0),
        k: 0.0,
        material: white_material.clone()
    }));
    world.push(Arc::new(XZRect {
        x: (0.0, 555.0),
        z: (0.0, 555.0),
        k: 555.0,
        material: white_material.clone()
    }));
    world.push(Arc::new(XYRect {
        x: (0.0, 555.0),
        y: (0.0, 555.0),
        k: 555.0,
        material: white_material.clone()
    }));

    let mut box1: Arc<dyn Hittable> = Arc::new(AxisAlignedBox::new(Point::ORIGIN, Point { x: 165.0, y: 330.0, z: 165.0 }, white_material.clone()));
    box1 = Arc::new(RotateY::new(box1.clone(), 15.0));
    box1 = Arc::new(Translate { object: box1.clone(), offset: Vector3 { x: 265.0, y: 0.0, z: 295.0 } });
    world.push(box1);

    let mut box2: Arc<dyn Hittable> = Arc::new(AxisAlignedBox::new(Point::ORIGIN, Point { x: 165.0, y: 165.0, z: 165.0 }, white_material.clone()));
    box2 = Arc::new(RotateY::new(box2.clone(), -18.0));
    box2 = Arc::new(Translate { object: box2.clone(), offset: Vector3 { x: 130.0, y: 0.0, z: 65.0 } });
    world.push(box2);

    return world;
}
