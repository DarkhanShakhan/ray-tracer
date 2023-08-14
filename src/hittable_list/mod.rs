use crate::interval::Interval;

use super::hittable;

pub struct HittableList {
    pub objects: Vec<Box<dyn hittable::Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }
    pub fn add(&mut self, object: Box<dyn hittable::Hittable>) {
        self.objects.push(object);
    }
    // pub fn clear(&mut self) {
    //     self.objects.clear();
    // }
}

impl hittable::Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval, rec: &mut hittable::HitRecord) -> bool {
        let mut temp_rec = hittable::HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}
