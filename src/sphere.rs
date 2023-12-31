use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {center, radius}
    }

    pub fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let oc = r.orig() - self.center;
        let a = r.dir().length_squared();
        let half_b = Vec3::dot(&oc, &r.dir());
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;

        if discriminant < 0.0 { return false; }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if (root <= ray_tmin) || (ray_tmax <= root) {
            root = (-half_b + sqrtd) / a;
            if (root <= ray_tmin) || (ray_tmax <= root) { return false; }
        }

        rec.t = root;
        rec.p = r.at(root);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        return true;
    }
}

