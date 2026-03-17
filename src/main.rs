use ray_tracer::raytracer::{camera::Camera, hittables::{self, Sphere}, vec3::Vec3};

mod raytracer;

fn main() -> Result<(), std::io::Error>{
    let mut world = hittables::Hittables::new();
    world.add(Sphere::new(Vec3::new(0.0,-10.0,-20.0), 5.0).share());
    world.add(Sphere::new(Vec3::new(10.0,10.0,-30.0), 5.0).share());
    world.add(Sphere::new(Vec3::new(-10.0,10.0,-30.0), 5.0).share());
    let cam = Camera::new(16f64/9f64, 2f64, 1f64);
    cam.render("out/img.ppm", 1600, 900, world)
}