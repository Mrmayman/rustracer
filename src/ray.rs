use crate::vector::Vec3;

#[derive(Default)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    time: f64
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time:f64) -> Ray {
        Ray { origin: origin, direction: direction, time: time }
    }
    
    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, distance: f64) -> Vec3 {
        self.origin + (self.direction * distance)
    }
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * std::f64::consts::PI / 180.0;
}