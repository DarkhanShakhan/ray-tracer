use super::hittable;

pub struct HittableList {
    pub objects: Vec<dyn hittable::Hittable>
}