extern crate rand;

use std::ops;
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn origin() -> Self {
        Self {x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {x, y, z}
    }

    pub fn get(&self) -> Self {
        Self::new(self.x, self.y, self.z)
    }

    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn z(&self) -> f64 { self.z }

    pub fn dot(a: &Self, b: &Self) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: &Self, b: &Self) -> Self{
        Self {
            x: a.y * b.z - a.z * a.y,
            y: a.z * b.x - a.x * a.z,
            z: a.x * b.y - a.y * a.x
        }
    }

    pub fn neg(v: &Self) -> Self {
        Self {
            x: -v.x,
            y: -v.y,
            z: -v.z
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn unit_vector(self) -> Self {
        self * (1.0/self.length())
    }

    fn random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    pub fn simple_random_in_unit_sphere() -> Self {
        let p = Vec3::random(-1.0,1.0);
        return Vec3::unit_vector(p);
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, a: Vec3) -> Vec3 {
        Vec3 {
            x: self.x() + a.x(),
            y: self.y() + a.y(),
            z: self.z() + a.z(),
        }
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, a: f64) -> Vec3 {
        Vec3 {
            x: self.x() + a,
            y: self.y() + a,
            z: self.z() + a,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, a: Vec3) -> Vec3 {
        Vec3 {
            x: self.x() - a.x(),
            y: self.y() - a.y(),
            z: self.z() - a.z(),
        }
    }
}

impl ops::Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, a: f64) -> Vec3 {
        Vec3 {
            x: self.x() - a,
            y: self.y() - a,
            z: self.z() - a,
        }
    }
}


impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, a: Vec3) -> Vec3 {
        Vec3 {
            x: self.x() * a.x(),
            y: self.y() * a.y(),
            z: self.z() * a.z(),
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, a: f64) -> Vec3 {
        Vec3 {
            x: self.x() * a,
            y: self.y() * a,
            z: self.z() * a,
        }
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, a: f64) -> Vec3 {
        Vec3::new(
            self.x() * a,
            self.y() * a,
            self.z() * a,
        )
    }
}

impl ops::Mul<i32> for Vec3 {
    type Output = Vec3;

    fn mul(self, a: i32) -> Vec3 {
        Vec3::new(
            self.x() * a as f64,
            self.y() * a as f64,
            self.z() * a as f64,
        )
    }
}

impl ops::Mul<i64> for Vec3 {
    type Output = Vec3;

    fn mul(self, a: i64) -> Vec3 {
        Vec3::new(
            self.x() * a as f64,
            self.y() * a as f64,
            self.z() * a as f64,
        )
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, a: f64) -> Vec3 {
        self*(1.0/a)
    }
}
