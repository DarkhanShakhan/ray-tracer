use prima::geom::Vec3;

pub fn write_color(color: Vec3) {
    println!(
        "{} {} {}",
        (255.9 * color.x) as i32,
        (255.9 * color.y) as i32,
        (255.9 * color.z) as i32
    );
}
