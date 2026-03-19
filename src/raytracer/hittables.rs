#![allow(dead_code)]

use std::sync::Arc;

use crate::raytracer::vec3::Vec3;
use crate::raytracer::ray::{Ray, Interval};

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}

pub struct Hittables {
    pub hittables: Vec<Arc<dyn Hittable>>
}

impl Hittables {
    pub fn new() -> Self {
        Self { hittables: Vec::new()}
    }

    pub fn clear(&mut self) {
        self.hittables.clear();
    }

    pub fn add(&mut self, hittable: Arc<dyn Hittable>) {
        self.hittables.push(hittable);
    }
}

impl Hittable for Hittables {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let mut hr = None;
        let mut closest = interval.max;
        for hittable in &self.hittables {
            if let Some(temp_rec) = hittable.hit(ray, Interval::new(interval.min, closest)) {
                closest = temp_rec.t;
                hr = Some(temp_rec);
            }
        }
        hr
    }
}


pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub is_face_front: bool,
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f64, front_face: bool) -> Self {
        Self { point, normal, t, is_face_front: front_face }
    }
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Self { center, radius }
    }
    pub fn share(&self) -> Arc<Self> {
        Arc::new(self.clone())
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.dot(ray.direction);
        let b_half = ray.direction.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant: f64 = b_half*b_half - a*c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut root = (b_half - sqrt_discriminant)/(a);

        if !interval.surrounds(root){
            root = (b_half + sqrt_discriminant)/(a);
            if !interval.surrounds(root){
                return None;
            }
        }

        let t = root;
        let point = ray.at(root);
        let mut normal = (point - self.center)/self.radius;

        let is_face_front = ray.direction.dot(normal) < 0.0;
        if !is_face_front {
            normal = -normal;
        }

        Some(HitRecord::new(point, normal, t, is_face_front))
    }
    
}