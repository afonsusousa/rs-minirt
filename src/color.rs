use crate::hits::Hittable;
use crate::hits::HittableList;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub fn color(ray: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    match world.hit(ray, 0.001, f32::MAX) {
        Some(hit) => {
            if let Some((attenuation, scattered)) = hit.material.scatter(ray, &hit) {
                attenuation * color(&scattered, world, depth - 1)
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        },
        None => {
            let unit_dir = ray.direction().unit_vector();
            let t = 0.5 * (unit_dir.y + 1.0);
            (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Vec3::new(0.5, 0.7, 1.0) * t)
        }
    }
}
