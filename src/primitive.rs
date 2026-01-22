use crate::math::Point3D;

pub struct Sphere {
    origin: Point3D,
    radius: f64,
    color: u32,
}

impl Sphere {
    pub fn new(origin: Point3D, radius: f64, color: u32) -> Self {
        Self { origin, radius, color }
    }
}

