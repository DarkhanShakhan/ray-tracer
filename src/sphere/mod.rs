use super::hittable::Hittable;
use prima::geom::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32
}

impl Sphere {
    pub fn new(center:Vec3, radius:f32) -> Self {
        Sphere{center, radius}
    }
}

impl Hittable for Sphere {
    fn hit(&self,r: &crate::ray::Ray,ray_tmin: f32, ray_tmax:f32,rec :&mut crate::hittable::HitRecord) -> bool {
        let oc = r.origin() - self.center; 
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a*c;
        if discriminant < 0.0 {
            return false
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if (root <= ray_tmin || ray_tmax <= root) {
            root = (-half_b + sqrtd) / a;
            if (root  <= ray_tmin || ray_tmax <= root) {
                return false
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        true
    }
}