use crate::vec3::{ Vec3, Point3 };
use crate::ray::Ray;
use crate::sphere::Sphere;


#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::origin(),
            normal: Vec3::origin(),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.dir(), outward_normal) < 0.0;
        self.normal =
            if self.front_face {
                Vec3::get(outward_normal)
            } else {
                Vec3::neg(outward_normal)
            };
    }

    pub fn hit(&mut self, world: &Vec<Sphere>, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for item in world {
            if item.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *self = temp_rec;
            }
        }

        return hit_anything;
    }
}
