use ray_tracer::raytracer::camera::Camera;

mod raytracer;
fn main() -> Result<(), std::io::Error>{
    let cam = Camera::new(16f64/9f64, 2f64, 1f64);
    cam.render("img.ppm", 1600, 900)
}