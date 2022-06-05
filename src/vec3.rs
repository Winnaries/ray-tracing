#![allow(dead_code)]

use rand::{thread_rng, Rng};
use std::fmt::{self, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};

pub type Color = Vec3;
pub type Point3 = Vec3;
pub struct Rgb(u64, u64, u64);

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zeros() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();
        Self::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = thread_rng();
        Self::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_within_unit_sphere() -> Self {
        Self::random_within_sphere(1.0)
    }

    pub fn random_within_sphere(radius: f64) -> Self {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length() < radius {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_within_unit_sphere().unit()
    }

    pub fn min(&self, v: f64) -> Self {
        Self {
            x: self.x.min(v), 
            y: self.y.min(v), 
            z: self.z.min(v), 
        }
    }

    pub fn sqrt(&self) -> Self {
        Self {
            x: self.x.sqrt(), 
            y: self.y.sqrt(), 
            z: self.z.sqrt(), 
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(), 
            y: self.y.abs(), 
            z: self.z.abs(), 
        }
    }

    pub fn sum(&self) -> f64 {
        self.x + self.y + self.z
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self[1] * other[2] - self[2] * other[1],
            y: self[2] * other[0] - self[0] * other[2],
            z: self[0] * other[1] - self[1] * other[0],
        }
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    pub fn rgb(&self, sample_per_pixel: u64) -> Rgb {
        let scale = 1.0 / sample_per_pixel as f64;
        let r = (self.x * scale).min(0.999).max(0.0).sqrt();
        let g = (self.y * scale).min(0.999).max(0.0).sqrt();
        let b = (self.z * scale).min(0.999).max(0.0).sqrt();
        Rgb(
            (256.0 * r).floor() as u64,
            (256.0 * g).floor() as u64,
            (256.0 * b).floor() as u64,
        )
    }

    pub fn near_zero(&self) -> bool {
        let s: f64 = 1e-8;
        self.x < s && self.y < s && self.z < s
    }

    pub fn length(&self) -> f64 {
        self.lengthsq().sqrt()
    }

    pub fn lengthsq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            1 => &self.x,
            2 => &self.y,
            3 => &self.z,
            _ => panic!(),
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, other: f64) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
