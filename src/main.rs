use ray_tracer::raytracer::{camera::Camera, hittables::{self, Sphere}, material::Material, util::Color, vec3::Vec3};

use crate::raytracer::util::random_f64;

mod raytracer;

fn main() -> Result<(), std::io::Error>{
    let mut world = hittables::Hittables::new();
    world.add(Sphere::new(Vec3::new(0.0,-105.0,-20.0), 100.0,
     Material::Metal { albedo: Color::new(0.3, 0.3, 0.8), fuzz: 0.01 }).share());
    for _ in 0..40 {
        world.add(
            Sphere::new(Vec3::new(random_f64(-40.0, 40.0), random_f64(0.0, 20.0), random_f64(-40.0, -10.0)),
            random_f64(1.0, 4.0),
            Material::Metal { albedo: Color::random(0.0, 1.0),
                 fuzz: random_f64(0.0, 1.0) }).share());
    }
    let background = Vec3::new(0.1, 0.3, 1.0);
    let cam = Camera::new(
        Vec3::ZERO,
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::UP,
        90.0,
        16.0 / 9.0,
        50,
        background,
    );

    cam.render("out/img.ppm", 1000, world)
}