use indicatif::{ProgressBar, ProgressStyle};
mod color;
use prima::geom::Vec3;

fn main() {
    ppm_image();
}

fn ppm_image() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;
    let bar = ProgressBar::new(IMAGE_HEIGHT as u64);
    let mut c: Vec3;
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        bar.inc(1);
        for i in 0..IMAGE_WIDTH {
            c = Vec3::new(
                i as f32 / (IMAGE_WIDTH - 1) as f32,
                j as f32 / (IMAGE_HEIGHT - 1) as f32,
                0.25,
            );
            color::write_color(c);
        }
    }
    bar.set_style(ProgressStyle::with_template("{msg}").unwrap());
    bar.finish_with_message("done");
}
