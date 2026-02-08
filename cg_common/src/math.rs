#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn project_viewport(&self, viewport_w: f64, viewport_h: f64, canvas_w: u32, canvas_h: u32, distance: f64) -> Point3D {
        let vx = self.x * (viewport_w / canvas_w as f64);
        let vy = self.y * (viewport_h / canvas_h as f64);

        Point3D::new(vx, vy, distance)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {x, y, z}
    }

    pub fn project2d(&self) -> Point2D {
        Point2D { x: self.x, y: self.y }
    }
}

impl std::ops::Sub for Point3D {
    type Output = Vector3;

    fn sub(self, other: Point3D) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Add<Vector3> for Point3D {
    type Output = Self;

    fn add(self, other: Vector3) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn len(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.len();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn dot(&self, other: Vector3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn cross(&self, other: Vector3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl std::ops::Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl std::ops::Add for Vector3 {
    type Output = Self;
    fn add(self, other: Vector3) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3D,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn from_points(origin: Point3D, destination: Point3D) -> Self {
        let direction = destination - origin;

        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn len(&self) -> f64 {
        (self.direction.x.powi(2) + self.direction.y.powi(2) + self.direction.z.powi(2)).sqrt()
    }

    pub fn cast(&self, t: f64) -> Point3D {
        self.origin + (self.direction * t)
    }
}

pub fn bad_quadratic(a: f64, b:f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = b * b - (4.0 * a * c);

    if a == 0.0 {
        if b != 0.0 {
            return Some((-c / -b, -c / b));
        } else {
            return None;
        }
    }
    
    if discriminant >= 0.0 {
        let sqrt_dis = discriminant.sqrt();
        let r1 = (-b + sqrt_dis) / (2.0 * a);
        let r2 = (-b - sqrt_dis) / (2.0 * a);
        Some((r1.min(r2), r1.max(r2)))
    } else {
        None
    }
}

pub fn lerp(i0: f64, d0: f64, i1: f64, d1: f64) -> Vec<i32> {
    let mut values: Vec<i32> = Vec::new();

    let a = (d1 - d0) / (i1 - i0);
    let mut d = d0;
    for _i in (i0 as i32)..(i1 as i32) {
        values.push(d as i32);
        d += a;
    }
    
    values
}

#[cfg(test)]
mod tests {
    use super::*;

}
