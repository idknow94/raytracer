#![allow(dead_code)]
use crate::raytracer::hittables::Hittable;
use crate::raytracer::hittables::Hittables;
use crate::raytracer::vec3::Vec3;
use crate::raytracer::util::*;
use crate::raytracer::ray::*;
use core::f64;
use std::fs::File;
use std::io::{Write, BufWriter};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    aspect_ratio: f64,
    samples_per_pixel: u32,
    background: Color,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov_degrees: f64,
        aspect_ratio: f64,
        samples_per_pixel: u32,
        background: Color,
    ) -> Self {
        let h = (vfov_degrees.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            aspect_ratio,
            samples_per_pixel,
            background,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }

    pub fn render(&self, path: &str, width: u32, hittables: Hittables) -> std::io::Result<()> {
        let height = (width as f64 / self.aspect_ratio).round() as u32;
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", width, height)?;
        writeln!(writer, "255")?;

        for i in (0..height).rev() {
            for j in 0..width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let u = (j as f64 + random_f64(0.0, 1.0)) / (width as f64 - 1.0);
                    let v = (i as f64 + random_f64(0.0, 1.0)) / (height as f64 - 1.0);
                    let ray = self.get_ray(u, v);
                    pixel_color += self.ray_color(&ray, &hittables, 100);
                }

                pixel_color /= self.samples_per_pixel as f64;
                // gamma correction for gamma=2.0
                pixel_color = Color::new(
                    pixel_color.x.sqrt(),
                    pixel_color.y.sqrt(),
                    pixel_color.z.sqrt(),
                );

                writeln!(writer, "{}", write_color(pixel_color))?;
            }
        }

        Ok(())
    }
    fn ray_color(&self, ray: &Ray, hittables: &Hittables, depth: u32) -> Color {
        if depth == 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        if let Some(record) = hittables.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            if let Some((attenuation, scattered)) = record.material.scatter(ray, &record) {
                return self.ray_color(&scattered, hittables, depth - 1) * attenuation;
            } else {
                return Color::BLACK;
            }
        }

        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        Color::WHITE * (1.0 - t) + self.background * t
    }
}


