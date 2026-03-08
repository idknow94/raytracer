use crate::raytracer::vec3::Vec3;
use crate::raytracer::util::write_color;
use crate::raytracer::ray::*;
use std::fs::File;
use std::io::{Write, BufWriter};

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Self {
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let viewport_width = viewport_height * aspect_ratio;
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, -viewport_height, 0.0);
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}

pub fn write_ppm(path: &str, width: u32, height: u32, data: &[Vec3]) -> std::io::Result<()> {
    let f= File::create(path)?;
    let mut writer = BufWriter::new(f);

    writeln!(writer, "P3")?;
    writeln!(writer, "{} {}", width, height)?;
    writeln!(writer, "255")?;

    for color in data.iter() {
        writeln!(writer, "{}", write_color(color))?;
    }
    Ok(())
}