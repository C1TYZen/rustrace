pub const INFINITY: f64 = f64::MAX - 1.0;
pub const PI: f64 = 3.1415926535897932385;

fn degrees_to_radians(deg: f64) -> f64 {
    deg * PI / 180.0
}
