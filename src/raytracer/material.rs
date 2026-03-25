#![allow(dead_code)]

use crate::raytracer::ray::Ray;
use crate::raytracer::hittables::HitRecord;
use crate::raytracer::util::Color;
use crate::raytracer::vec3::Vec3;


#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
}
impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Material::Lambertian { albedo } => {
                let scatter_direction = hit_record.normal + Vec3::random_unit_vector();
                if scatter_direction.near_zero() {
                    // Catch degenerate scatter direction
                    return Some((*albedo, Ray::new(hit_record.point, hit_record.normal)));
                }
                let scattered = Ray::new(hit_record.point, scatter_direction);
                Some((*albedo, scattered))
            }
            Material::Metal { albedo, fuzz } => {
                let fuzz = f64::clamp(*fuzz, 0.0, 1.0);
                let reflected = ray_in.direction.unit_vector().reflect(hit_record.normal);
                let scattered = Ray::new(hit_record.point, reflected + Vec3::random_unit_vector() * fuzz);
                if scattered.direction.dot(hit_record.normal) > 0.0 {
                    Some((*albedo * 0.9, scattered))  // Slight energy loss for metals too
                } else {
                    None
                }
            }
        }
    }
}