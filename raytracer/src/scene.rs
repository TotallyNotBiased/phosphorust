use cg_common::math::{Point3D, Ray, Vector3};
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

    pub fn reflect_ray(&self, r: Vector3, n: Vector3) -> Vector3 {
        n * 2.0 * n.dot(r) + (-r)
    }

    pub fn compute_lighting(&self, p: Point3D, n: Vector3, vv: Vector3, s: u32) -> f64 {
        let mut i = 0.0;
        for light in &self.lights {
            if let Light::Ambient { intensity } = light {
                i += intensity;
                continue;
            }

            if let Some((l,t_max)) = light.vector(p) {
                // shadows
                let (_shadow_t,_ , shadow_object) = self.closest_intersection(p, l, 0.001, t_max as usize);

                match shadow_object {
                    Some(_o) => continue,
                    None => {},
                }
                
                let m = n.dot(l);
                // diffuse
                if m > 0.0 {
                    i += light.intensity() * m/(n.len() * l.len());
                }
                // specular
                if s != 0 {
                    let r = self.reflect_ray(l, n);
                    let t = r.dot(vv);
                    if t > 0.0 {
                        i += light.intensity() * (t/(r.len() * vv.len())).powi(s as i32);
                    }
                }
            }
        }
        i
    }

    pub fn closest_intersection(&self, o: Point3D, d: Vector3, distance: f64, viewrange: usize) -> (f64, Ray, Option<&Box<dyn Primitive>>) {
        let mut closest_t = viewrange as f64;
        let ray = Ray { origin: o, direction: d };
        let mut closest_object: Option<&Box<dyn Primitive>> = None;

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

    pub fn trace_ray(&self, o: Point3D, d: Vector3, distance: f64, viewrange: usize, recursion_depth: usize) -> u32 {
        let (closest_t, ray, closest_object) = 
            self.closest_intersection(o, d, distance, viewrange);

        match closest_object {
            None => self.background_color,
            Some(object) => { 
                let p = o + (ray.direction * closest_t);
                let mut n = p - object.get_origin();
                n = n.normalize();
                let local_color = apply_intensity(object.color(), 
                    self.compute_lighting(p, n, -ray.direction, object.specular()));
                let reflectivity = object.reflective();
                if recursion_depth <= 0 || reflectivity <= 0.0 {
                    return local_color;
                }

                let r = self.reflect_ray(-ray.direction, n);
                let reflected_color = self.trace_ray(p, r, 0.001, viewrange, recursion_depth - 1);

                add_colors(
                    apply_intensity(local_color, 1.0 - reflectivity),
                    apply_intensity(reflected_color,reflectivity)
                )
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

fn add_colors(color_a: u32, color_b: u32) -> u32 {
    let new_r = (((color_a >> 16) & 0xFF) + ((color_b >> 16) & 0xFF)).min(255);
    let new_g = (((color_a >> 8) & 0xFF) + ((color_b >> 8) & 0xFF)).min(255);
    let new_b = (((color_a) & 0xFF) + ((color_b) & 0xFF)).min(255);

    new_r << 16 | new_g << 8 | new_b 
}

pub struct Viewport {
    pub width: f64,
    pub height: f64,
}
