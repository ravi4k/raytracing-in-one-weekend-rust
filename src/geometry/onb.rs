use crate::geometry::vector::Vector3;

pub struct ONB {
    pub u: Vector3,
    pub v: Vector3,
    pub w: Vector3,
}

impl ONB {
    pub fn local(&self, a: Vector3) -> Vector3 {
        return a.x * self.u + a.y * self.v + a.z * self.w;
    }

    pub fn build_from_w(n: Vector3) -> Self {
        let w = n.direction();
        let a = if w.x.abs() > 0.9 { Vector3 { x: 0.0, y: 1.0, z: 0.0 } } else { Vector3 { x: 1.0, y: 0.0, z: 0.0 } };
        let v = w.cross(a).direction();
        let u = w.cross(v);
        return Self { u, v, w };
    }
}