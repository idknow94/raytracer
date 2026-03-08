use crate::raytracer::vec3::Vec3;
use crate::raytracer::util::write_color;
use std::fs::File;
use std::io::{Write, BufWriter};

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