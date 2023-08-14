use crate::color;
use crate::interval::Interval;
use crate::ray::{self, Ray};
use derivative::Derivative;
use indicatif::ProgressBar;
use prima::geom::Vec3;
use rand::Rng;

use crate::hittable;

#[derive(Derivative)]
#[derivative(Debug, Default)]
pub struct Camera {
    #[derivative(Default(value = "1.0"))]
    pub aspect_ratio: f32,
    #[derivative(Default(value = "100"))]
    pub image_width: i32,
    #[derivative(Default(value = "10"))]
    pub samples_per_pixel: i32,
    image_height: i32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn render(&mut self, world: &dyn hittable::Hittable) {
        self.initialize();
        let bar = ProgressBar::new(self.image_height as u64);
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for j in 0..self.image_height {
            bar.inc(1);
            for i in 0..self.image_width {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, world);
                }
                color::write_color(pixel_color, self.samples_per_pixel);
            }
        }
    }
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as i32;
        if self.image_height < 1 {
            self.image_height = 1;
        }
        self.center = Vec3::new(0.0, 0.0, 0.0);

        //  Determine viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width =
            viewport_height * ((self.image_width as f32) / (self.image_height as f32));

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(&self, r: &ray::Ray, world: &dyn hittable::Hittable) -> Vec3 {
        let mut rec: hittable::HitRecord = hittable::HitRecord::default();
        if world.hit(r, Interval::new(0.0, f32::INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0));
        }
        let unit_direction = r.direction().normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f32 * self.pixel_delta_u) + (j as f32 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        let px = -0.5 + rng.gen_range(0.0..1.0);
        let py = -0.5 + rng.gen_range(0.0..1.0);
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}
