use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use cargo::color::Color;

pub struct Camera {
    pub aspect_ratio: f64,      // Ratio of image width over height
    pub image_width: i32,       // Rendered image width in pixel count
    pub samples_per_pixel: i32, // Count of random samples for each pixel
    pub max_depth: i32,         // Maximum number of ray bounces into scene

    image_height: i32,   // Rendered image height
    center: Point3,      // Camera center
    pixel00_loc: Point3, // Location of pixel 0, 0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
}

impl Camera {
    fn render(world: &Hittable) {
        println!("P3\n{img_w} {img_h}\n255");

        for j in 0..img_h as i32 {
            let pixel_color: Vec3 = Vec3(0,0,0);
            for i in 0..img_w as i32 {
                let pixel_center = pixe100_loc + (pixel_delta_u * i) + (pixel_delta_v * j);
                let ray_direction = pixel_center - camera_center;
                let r = Ray::new(camera_center, ray_direction);

                let pixel_color = ray_color(&r);
                print!("{}\n", color::write_color(pixel_color));
            }
        }
    }
}
