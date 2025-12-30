use std::fmt;
use std::ops;
use std::ops::{DivAssign, MulAssign};
use rand::{rng, Rng};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 { pub x: f64, pub y: f64, pub z: f64, }

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn random() -> Vec3 {
        let mut rng = rng();
        Vec3::new(rng.random(), rng.random(), rng.random())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        let mut rng = rng();
        Vec3::new(
            rng.random_range(min..max),
            rng.random_range(min..max),
            rng.random_range(min..max)
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.sqr_length() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rng();
        loop {
            let p = Vec3::new(
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
                0.0
            );
            if p.sqr_length() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - n * 2.0 * self.dot(n)
    }

    pub fn refract(self, n: Vec3, etai_over_etat: f64) -> Option<Vec3> {
        let uv = self.unit_vector();
        let dt = uv.dot(n);
        let discriminant = 1.0 - etai_over_etat * etai_over_etat * (1.0 - dt * dt);
        if discriminant > 0.0 {
            Some((uv - n * dt) * etai_over_etat - n * discriminant.sqrt())
        } else {
            None
        }
    }

    pub fn r(&self) -> f64 { self.x }
    pub fn g(&self) -> f64 { self.y }
    pub fn b(&self) -> f64 { self.z }

    pub fn length(&self) -> f64 {
        self.sqr_length().sqrt()
    }

    pub fn sqr_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Div for Vec3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}


impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }
}


impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vec3"),
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds for Vec3"),
        }
    }
}