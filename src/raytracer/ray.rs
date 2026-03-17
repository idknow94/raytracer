#![allow(dead_code)]
use core::f64;

use crate::raytracer::vec3::Vec3;
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction*t
    }
}

pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Interval {
    pub const EMPTY: Interval = Self{min: f64::INFINITY, max: f64::NEG_INFINITY};
    pub const UNIVERSE: Interval = Self{min: f64::NEG_INFINITY, max: f64::INFINITY};

    pub fn new(min: f64, max: f64) -> Self {
        Self {min: min, max: max}
    }
    pub fn contains(&self, f: f64) -> bool {
        return self.min <= f && f <= self.max
    }
    pub fn surrounds(&self, f: f64) -> bool {
        return self.min < f && f < self.max
    }
    pub fn size(&self) -> f64 {
        return self.max - self.min
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::UNIVERSE
    }
}