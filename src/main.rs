mod vec3;
mod ray;
mod color;
mod sphere;
mod hittable;
mod values;

use vec3::{ Vec3, Point3 };
use ray::Ray;
use color::Color;
use hittable::HitRecord;
use sphere::Sphere;
use values::*;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc: Vec3 = r.orig() - center;
    let a: f64 = r.dir().length_squared();
    let half_b: f64 = Vec3::dot(&oc, &r.dir());
    let c: f64 = oc.length_squared() - radius*radius;
    let discriminant: f64 = half_b*half_b - a*c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / (a);
    }
}

fn ray_color(r: &Ray, world: &Vec<Sphere>) -> Color {
    let mut rec = HitRecord::new();
    if rec.hit(world, r, 0.01, INFINITY) {
        return (rec.normal + Color::new(1.0,1.0,1.0)) * 0.5;
    }

    let unit_direction: Vec3 = r.dir().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    return
        Color{x: 1.0, y: 1.0, z: 1.0}*(1.0-a) +
        Color{x: 0.5, y: 0.7, z: 1.0}*a;
}

fn main() {
    let mut world: Vec<Sphere> = Vec::new();
    world.push(Sphere::new(Point3::new( 0.0, 0.0,-1.0), 0.5));
    world.push(Sphere::new(Point3::new( 1.0, 0.0,-1.0), 0.5));
    world.push(Sphere::new(Point3::new(-1.0, 0.0,-1.0), 0.5));
    world.push(Sphere::new(Point3::new( 0.0,-100.5,-1.0), 100.0));

    let aspect_ratio: f64 = 16.0 / 9.0;
    let img_w: f64 = 400.0;
    let img_h = img_w / aspect_ratio;

    // Camera
    let focal_length: f64 = 1.0;
    let vport_h: f64 = 2.0;
    let vport_w = vport_h * (img_w / img_h);
    let camera_center = Point3::new(0.0,0.0,0.0);
    let vport_u = Vec3::new(vport_w, 0.0, 0.0);
    let vport_v = Vec3::new(0.0, -vport_h, 0.0);
    let pixel_delta_u = vport_u/img_w;
    let pixel_delta_v = vport_v/img_h;
    let vport_upper_left: Vec3 =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - vport_u/2.0 - vport_v/2.0;
    let pixe100_loc: Vec3 = vport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5);

    println!("P3\n{img_w} {img_h}\n255");

    for j in 0..img_h as i64 {
        for i in 0..img_w as i64 {
            let pixel_center = pixe100_loc + (pixel_delta_u * i) + (pixel_delta_v * j);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, &world);
            print!("{}\n", color::write_color(pixel_color));
        }
    }
}
