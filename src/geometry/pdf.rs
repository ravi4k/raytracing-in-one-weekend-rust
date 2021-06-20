use std::sync::Arc;

use crate::geometry::onb::ONB;
use crate::geometry::vector::{Point, Vector3};
use crate::objects::hittable::Hittable;
use crate::utils::{PI, random_f32};

pub fn random_cosine_direction() -> Vector3 {
    let r1 = random_f32();
    let r2 = random_f32();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    return Vector3 { x, y, z };
}

pub trait PDF {
    fn value(&self, direction: Vector3) -> f32;
    fn generate(&self) -> Vector3;
}

pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    pub fn new(w: Vector3) -> Self {
        return Self {
            uvw: ONB::build_from_w(w)
        };
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: Vector3) -> f32 {
        let cosine = direction.dot(self.uvw.w);
        return if cosine <= 0.0 { 0.0 } else { cosine / PI };
    }

    fn generate(&self) -> Vector3 {
        return self.uvw.local(random_cosine_direction());
    }
}

pub struct HittablePDF {
    pub o: Point,
    pub ptr: Arc<dyn Hittable>,
}

impl PDF for HittablePDF {
    fn value(&self, direction: Vector3) -> f32 {
        return self.ptr.pdf_value(self.o, direction);
    }

    fn generate(&self) -> Vector3 {
        return self.ptr.random(self.o);
    }
}

pub struct MixturePDF {
    pub ptr: [Arc<dyn PDF>; 2],
}

impl PDF for MixturePDF {
    fn value(&self, direction: Vector3) -> f32 {
        return 0.5 * (self.ptr[0].value(direction) + self.ptr[1].value(direction));
    }

    fn generate(&self) -> Vector3 {
        return if random_f32() < 0.5 {
            self.ptr[0].generate()
        } else {
            self.ptr[1].generate()
        };
    }
}
