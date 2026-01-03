use crate::hits::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<dyn Material>) -> Sphere {
        Sphere { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();

            let temp = (-b - sqrt_discriminant) / a;
            if temp < t_max && temp > t_min {
                let p = ray.at(temp);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord {
                    t: temp,
                    p,
                    normal,
                    material: self.material.as_ref(),
                });
            }

            let temp = (-b + sqrt_discriminant) / a;
            if temp < t_max && temp > t_min {
                let p = ray.at(temp);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord {
                    t: temp,
                    p,
                    normal,
                    material: self.material.as_ref(),
                });
            }
        }
        None
    }
}
