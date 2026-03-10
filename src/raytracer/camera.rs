#![allow(dead_code)]
use crate::raytracer::vec3::Vec3;
use crate::raytracer::util::write_color;
use crate::raytracer::ray::*;
use std::fs::File;
use std::io::{Write, BufWriter};


pub struct Camera {
    camera_center: Vec3,
    viewport_u: Vec3,
    viewport_v: Vec3,
    viewport_upper_left: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Self {
        let camera_center = Vec3::new(0.0, 0.0, 0.0);
        let viewport_width = viewport_height * aspect_ratio;
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let viewport_upper_left = camera_center - viewport_u/2.0 - viewport_v/2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            camera_center,
            viewport_u,
            viewport_v,
            viewport_upper_left: viewport_upper_left,
        }
    }

    pub fn render(&self, path: &str, width: u32, height: u32) -> std::io::Result<()> {
        let f= File::create(path)?;
        let mut writer = BufWriter::new(f);

        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", width, height)?;
        writeln!(writer, "255")?;

        let pixel_delta_u=self.viewport_u/f64::from(width);
        let pixel_delta_v=self.viewport_v/f64::from(height);
        let pixel_00_location = self.viewport_upper_left + (pixel_delta_u + pixel_delta_v)/2.0;

        let height_inv = 1f64/height as f64;
        let width_inv = 1f64/width as f64;

        for i in 0..(height) {
            for j in 0..(width) {
                let pixel_center = pixel_00_location + (pixel_delta_v*f64::from(i)) + (pixel_delta_u*f64::from(i));
                let ray_direction = pixel_center-self.camera_center;
                #[allow(unused)]
                let ray = Ray::new(self.camera_center, ray_direction);

                writeln!(writer, "{}", write_color(Vec3::new(i as f64 * height_inv, 0.0, j as f64 * width_inv)))?;
            }
        }
        Ok(())
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.camera_center,
            self.viewport_upper_left + self.viewport_u * u + self.viewport_v * v - self.camera_center,
        )
    }
}

