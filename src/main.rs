use rand::{rng, Rng};
use crate::hits::{Hittable, HittableList};
use crate::material::{Dielectric, Lambertian, Metal};

mod vec3;
mod ray;
mod hits;
mod sphere;
mod camera;
mod material;
mod render;
mod color;

use crate::vec3::Vec3;
use crate::render::Render;
use crate::sphere::Sphere;

fn simple_world() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.8)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, ground_material)));

    let material_center = Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center)));

    let material_left = Box::new(Dielectric::new(1.5, Vec3::new(1.0, 0.8, 0.8)));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left.clone())));

    let material_right = Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));
    world.add(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    world
}

fn main() {
    let world = simple_world();

    let mut render = Render::new(400, 225, 50);
    render
        .lookfrom(3.0, 3.0, 2.0)
        .lookat(0.0, 0.0, -1.0)
        .vup(0.0, 1.0, 0.0)
        .vfov(20.0)
        .aperture(0.1)
        .focus_dist((Vec3::new(3.0, 3.0, 2.0) - Vec3::new(0.0, 0.0, -1.0)).length())
        .ipd(0.06)
        .render_scene(world, false);
}
