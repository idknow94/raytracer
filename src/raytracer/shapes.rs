use crate::raytracer::vec3::Vec3;
use crate::raytracer::ray::{Hit, Ray};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Self { center, radius }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray) -> bool {
        let oc = self.center-ray.origin;
        let a = ray.direction.dot(ray.direction);
        let b = ray.direction.dot(oc)*-2.0;
        let c = oc.dot(oc) - self.radius*self.radius;
        return b*b - 4f64*a*c>=0f64;
    }
}

