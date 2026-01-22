use crate::math::{Point3D, Ray, bad_quadratic};

pub trait Primitive {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn color(&self) -> u32;
}

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

impl Primitive for Sphere {
    
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let to_origin = ray.origin - self.origin;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * (to_origin.dot(ray.direction));
        let c = (to_origin.dot(to_origin)) - self.radius.powi(2);

        let roots = bad_quadratic(a, b, c);

        match roots { 
            None => None,
            Some((t1, t2)) => {

                if t1 < 0.0 && t2 < 0.0 {
                    return None; // sphere is behind camera
                }

                if t1 < 0.0 { return Some(t2); } // camera is inside the sphere

                Some(t1.min(t2))
            }
        }
    }

    fn color(&self) -> u32 {
        self.color
    }
}

