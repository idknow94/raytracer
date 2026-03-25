#![allow(unused)]
use std::f64::consts::PI;
use rand;

use crate::raytracer::{
    hittables::{HitRecord, Hittable, Hittables},
    ray::{Interval, Ray},
    vec3::Vec3,
};

pub type Color = Vec3;

pub fn write_color(pixel_color: Vec3) -> String {
    let r = linear_to_gamma(pixel_color.x);
    let g = linear_to_gamma(pixel_color.y);
    let b = linear_to_gamma(pixel_color.z);

    let intencity = Interval::new(0.0000, 0.9999);

    let r_byte = (256.0 * intencity.clamp(r)) as u32;
    let g_byte = (256.0 * intencity.clamp(g)) as u32;
    let b_byte = (256.0 * intencity.clamp(b)) as u32;

    format!("{} {} {}", r_byte, g_byte, b_byte)
}

pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees*PI/180.0
}

pub fn normal_color(record: &HitRecord) -> Color {
    (record.normal + Color::new(1.0, 1.0, 1.0)) * 0.5
}

pub fn random_f64(min: f64, max: f64) -> f64 {
    rand::random_range(min..=max)
}

pub fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0.0 {
        linear.sqrt()
    }
    else {0.0}
}
