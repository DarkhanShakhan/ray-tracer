use crate::interval::Interval;

use super::ray;
use prima::geom::Vec3;
pub trait Hittable {
    fn hit(&self, r: &ray::Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

#[derive(Copy, Clone, Default)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &ray::Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(*outward_normal) < 0.0;
        self.normal = {
            if self.front_face {
                *outward_normal
            } else {
                -*outward_normal
            }
        }
    }
}
