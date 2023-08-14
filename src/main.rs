mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;

use prima::geom::Vec3;

fn main() {
    let mut world = hittable_list::HittableList::new();
    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
    )));
    let mut cam = camera::Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 10;
    cam.render(&world);
}
