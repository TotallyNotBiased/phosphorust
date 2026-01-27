use crate::math::{Point3D, Ray, Vector3};
use crate::primitive::Primitive;
use crate::light::*;

pub struct Scene {
    pub origin: Point3D,
    pub objects: Vec<Box<dyn Primitive>>,
    pub lights: Vec<Light>,
    pub background_color: u32,
}

impl Scene {
    pub fn new() -> Self {
        Self { 
            origin: Point3D { x: 0.0, y: 0.0, z: 0.0 },
            objects: Vec::new(),
            lights: Vec::new(),
            background_color: 0x101010,
        }
    }

    pub fn add(&mut self, object: Box<dyn Primitive>) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn compute_lighting(&self, p: Point3D, n: Vector3, vv: Vector3, s: u32) -> f64 {
        let mut i = 0.0;
        for light in &self.lights {
            if let Light::Ambient { intensity } = light {
                i += intensity;
                continue;
            }

            if let Some(l) = light.vector(p) {
                let m = n.dot(l);
                if m > 0.0 {
                    i += light.intensity() * m/(n.len() * l.len());
                }

                if s != 0 {
                    let r = n * 2.0 * n.dot(l) + -l;
                    let t = r.dot(vv);
                    if t > 0.0 {
                        i += light.intensity() * (t/(r.len() * vv.len())).powi(s as i32);
                    }
                }
            }
        }
        i
    }

    pub fn closest_intersection(&self, o: Point3D, d: Point3D, distance: f64, viewrange: usize) -> (f64, Ray, Option<&Box<dyn Primitive>>) {
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
                None => { /* don't do anything lol*/ }
            }
        }
        (closest_t, ray, closest_object)
    }
    pub fn trace_ray(&self, o: Point3D, d: Point3D, distance: f64, viewrange: usize) -> u32 {
        let (closest_t, ray, closest_object) = 
            self.closest_intersection(o, d, distance, viewrange);
        match closest_object {
            None => self.background_color,
            Some(object) => { 
                let p = o + (ray.direction * closest_t);
                let mut n = p - object.get_origin();
                n = n.normalize();
                apply_intensity(object.color(), self.compute_lighting(p, n, -ray.direction, object.specular()))
            },
        }
    }
}

fn apply_intensity(color: u32, n: f64) -> u32 {
    let red = (((color >> 16) & 0xFF) as f64 * n) as u8;
    let green = (((color >> 8) & 0xFF) as f64 * n) as u8;
    let blue = (((color) & 0xFF) as f64 * n) as u8;

    (((red.clamp(0, 255) as u32) << 16) | 
    ((green.clamp(0, 255) as u32) << 8) | 
    (blue.clamp(0, 255) as u32)).into()
}

pub struct Viewport {
    pub width: f64,
    pub height: f64,
}
