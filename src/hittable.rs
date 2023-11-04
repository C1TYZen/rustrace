use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.dir(), outward_normal) < 0.0;
        self.normal =
            if self.front_face {
                Vec3::get(outward_normal)
            } else {
                Vec3::neg(outward_normal)
            };
    }
}

pub struct HittableList<'a> {
    data: (),
    next: &'a Self,
    prev: &'a Self,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}
