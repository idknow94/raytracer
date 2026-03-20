use ray_tracer::raytracer::{camera::Camera, hittables::{self, Sphere}, vec3::Vec3};

mod raytracer;

fn main() -> Result<(), std::io::Error>{
    let mut world = hittables::Hittables::new();
    world.add(Sphere::new(Vec3::new(0.0,0.0,-20.0), 5.0).share());
    world.add(Sphere::new(Vec3::new(0.0,-105.0,-20.0), 100.0).share());
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