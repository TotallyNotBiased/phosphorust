use crate::math::{Point3D, Ray};
use crate::primitive::Primitive;

pub struct Scene {
    pub origin: Point3D,
    pub objects: Vec<Box<dyn Primitive>>,
    pub background_color: u32,
}

impl Scene {
    pub fn new() -> Self {
        Self { 
            origin: Point3D { x: 0.0, y: 0.0, z: 0.0 },
            objects: Vec::new(),
            background_color: 0,
        }
    }

    pub fn add(&mut self, object: Box<dyn Primitive>) {
        self.objects.push(object);
    }

    pub fn trace_ray(&self, o: Point3D, d: Point3D, distance: f64, viewrange: usize) -> u32 {
        let mut closest_t = viewrange as f64;
        let mut closest_object: Option<&Box<dyn Primitive>> = None;
        let ray = Ray { origin: self.origin, direction: (d - o).normalize() };
        for object in &self.objects {
            
            let t = object.intersect(&ray);

            match t {
                Some(t) => { 
                    if (distance <= t && t <= viewrange as f64) && t < closest_t {
                        closest_t = t;
                        closest_object = Some(object);
                    }
                }
                None => {
                }
            }
        }

        match closest_object {
            None => self.background_color,
            Some(object) => object.color(),
        }
    }
}

pub struct Viewport {
    pub width: f64,
    pub height: f64,
}
