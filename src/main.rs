use crate::hits::{HittableList};
use crate::material::{Dielectric, Lambertian, Metal};
use rand::Rng;

mod vec3;
mod ray;
mod hits;
mod sphere;
mod camera;
mod material;
mod render;
mod color;
mod bounds;

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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    let mut rng = rand::rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.random::<f32>();
            let center = Vec3::new(
                a as f32 + 0.9 * rng.random::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.random::<f32>()
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random_range(0.0, 1.0) * Vec3::random_range(0.0, 1.0);
                    let sphere_material = Box::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = rng.random_range(0.0..0.5);
                    let sphere_material = Box::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Box::new(Dielectric::new(1.5, Vec3::new(1.0, 1.0, 1.0)));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Box::new(Dielectric::new(1.5, Vec3::new(1.0, 1.0, 1.0)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3)));

    world
}

fn main() {
    let world = random_scene();

    let mut render = Render::new( 3840, 2160, 1000);
    render
        .lookfrom(13.0, 2.0, 3.0)
        .lookat(0.0, 0.0, 0.0)
        .vup(0.0, 1.0, 0.0)
        .vfov(20.0)
        .aperture(0.6)
        .focus_dist(10.0)
        .ipd(0.06)
        .render_scene(world, true);
}
