use prima::geom::Vec3;

pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Ray { orig, dir }
    }
    pub fn at(&self, t: f32) -> Vec3 {
        self.orig + (t * self.dir)
    }
    // pub fn origin(&self) -> Vec3 {
    //     self.orig
    // }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
}
