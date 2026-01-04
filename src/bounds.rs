use std::ops;
use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn empty() -> Interval {
        Interval {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Copy, Clone)]
pub struct BBox {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl BBox {
    pub fn new(x: Interval, y: Interval, z: Interval) -> BBox {
        BBox { x, y, z }
    }

    pub fn from_points(a: Vec3, b: Vec3) -> BBox {
        let x = if a[0] <= b[0] {
            Interval::new(a[0] as f64, b[0] as f64)
        } else {
            Interval::new(b[0] as f64, a[0] as f64)
        };
        let y = if a[1] <= b[1] {
            Interval::new(a[1] as f64, b[1] as f64)
        } else {
            Interval::new(b[1] as f64, a[1] as f64)
        };
        let z = if a[2] <= b[2] {
            Interval::new(a[2] as f64, b[2] as f64)
        } else {
            Interval::new(b[2] as f64, a[2] as f64)
        };
        BBox { x, y, z }
    }

    pub fn hit(&self, r: &Ray, ray_t: Interval) -> bool {
        let mut ray_t = ray_t;
        for axis in 0..3 {
            let ax = &self[axis];
            let adinv = 1.0 / r.direction[axis] as f64;

            let t0 = (ax.min - r.origin[axis] as f64) * adinv;
            let t1 = (ax.max - r.origin[axis] as f64) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min { ray_t.min = t0; }
                if t1 < ray_t.max { ray_t.max = t1; }
            } else {
                if t1 > ray_t.min { ray_t.min = t1; }
                if t0 < ray_t.max { ray_t.max = t0; }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }
}

impl Default for BBox {
    fn default() -> Self {
        BBox {
            x: Interval::default(),
            y: Interval::default(),
            z: Interval::default(),
        }
    }
}

impl ops::Index<usize> for BBox {
    type Output = Interval;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }
}
