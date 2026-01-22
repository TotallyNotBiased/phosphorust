use crate::math::{Point3D, Vector3};

pub enum Light {
    Point { 
        intensity: f64, 
        position: Point3D 
    },
    Directional { 
        intensity: f64, 
        direction: Vector3 
    },
    Ambient { 
        intensity: f64 
    },
}

impl Light {
    pub fn new_point(intensity: f64, position: Point3D) -> Self {
        Light::Point { intensity, position }
    }

    pub fn new_directional(intensity: f64, direction: Vector3) -> Self {
        Light::Directional { intensity, direction }
    }

    pub fn new_ambient(intensity: f64) -> Self {
        Light::Ambient { intensity }
    }

    pub fn intensity(&self) -> f64 {
        match self {
            Light::Point { intensity, .. } => *intensity,
            Light::Directional { intensity, .. } => *intensity,
            Light::Ambient { intensity } => *intensity,
        }
    }

    pub fn vector(&self, p: Point3D) -> Option<Vector3> {
        match self {
            Light::Point { position, .. } => {
                Some(*position - p) 
            },
            Light::Directional { direction, .. } => {
                Some(*direction)
            },
            Light::Ambient { .. } => {
                None
            }
        }
    }
}


