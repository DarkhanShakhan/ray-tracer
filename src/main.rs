use indicatif::{ProgressBar, ProgressStyle};
mod color;
mod ray;
use prima::geom::Vec3;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1
    let mut image_height = (image_width as f32 / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1;
    }

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    // let ppm_image(image_width, image_height);
    let bar = ProgressBar::new(image_height as u64);
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        bar.inc(1);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let c = ray_color(ray::Ray::new(camera_center, ray_direction));
            color::write_color(c);
        }
    }
}

fn ray_color(r: ray::Ray) -> Vec3 {
    let unit_direction = r.direction();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
}

fn ppm_image(image_width: i32, image_height: i32) {
    let bar = ProgressBar::new(image_height as u64);
    let mut c: Vec3;
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        bar.inc(1);
        for i in 0..image_width {
            c = Vec3::new(
                i as f32 / (image_width - 1) as f32,
                j as f32 / (image_height - 1) as f32,
                0.25,
            );
            color::write_color(c);
        }
    }
    let r = ray::Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 2.0, 1.0));
    r.at(0.5);

    bar.set_style(ProgressStyle::with_template("{msg}").unwrap());
    bar.finish_with_message("done");
}
