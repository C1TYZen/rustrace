use crate::vec3::Vec3;

pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn origin() -> Self {
        Self {
            orig: Vec3::origin(),
            dir: Vec3::origin(),
        }
    }

    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            orig: a,
            dir: b,
        }
    }

    pub fn orig(&self) -> Vec3 {
        Vec3::new(
            self.orig.x(),
            self.orig.y(),
            self.orig.z()
        )
    }
    pub fn dir(&self) -> Vec3 {
        Vec3::new(
            self.dir.x(),
            self.dir.y(),
            self.dir.z()
        )
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + (self.dir * t)
    }
}
