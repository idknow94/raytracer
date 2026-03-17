#![allow(unused)]
use std::f64::consts::PI;

use crate::raytracer::{hittables::HitRecord, vec3::Vec3};

pub type Color = Vec3;
pub fn write_color(pixel_color: Vec3) -> String {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    let r_byte = (255.999 * r) as u32;
    let g_byte = (255.999 * g) as u32;
    let b_byte = (255.999 * b) as u32;

    format!("{} {} {}", r_byte, g_byte, b_byte)
}
pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees*PI/180.0
}
pub fn normal_color(record: &HitRecord) -> Color {
    (record.normal+Color::new(1.0,1.0,1.0))*0.5
}