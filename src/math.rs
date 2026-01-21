#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
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
