use rand::{rng, Rng};
use crate::ray::Ray;
use crate::hits::HitRecord;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self { albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = r_in.direction().unit_vector().reflect(rec.normal);
        let scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz);

        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Dielectric {
    pub albedo: Vec3,
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64, albedo: Vec3) -> Self {
        Self { albedo, ref_idx }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {

        let dot = r_in.direction().dot(rec.normal);
        let front_face = dot < 0.0;

        let normal = if front_face { rec.normal } else { -rec.normal };
        let refraction_ratio = if front_face { 1.0 / self.ref_idx } else { self.ref_idx };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = (-unit_direction).dot(normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction;

        let mut rng = rng();
        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.random() {
            direction = unit_direction.reflect(normal);
        } else {
            if let Some(refracted) = unit_direction.refract(normal, refraction_ratio) {
                direction = refracted;
            } else {
                direction = unit_direction.reflect(normal);
            }
        }

        let scattered = Ray::new(rec.p, direction);
        Some((self.albedo, scattered))
    }
}
