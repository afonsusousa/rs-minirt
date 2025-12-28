use std::io::Write;
use rand::{rng, Rng};
use crate::camera::Camera;
use crate::hits::{Hittable, HittableList};
use crate::material::{Dielectric, Lambertian, Metal};

mod vec3;
mod ray;
mod hits;
mod sphere;
mod camera;
mod material;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::Sphere;

fn color(ray: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    match world.hit(ray, 0.001, f64::MAX) {
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

fn main() {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("out.ppm")
        .unwrap();

    let nx = 1920;
    let ny = 1080;
    let ns = 200;

    writeln!(file, "P3\n{} {}\n255", nx, ny).unwrap();

    let lookfrom = Vec3::new(3.0, 3.0, 2.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let ipd = 0.06;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        nx as f64 / ny as f64,
        ipd
    );

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)))
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)))
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0))
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(Dielectric::new(1.5))
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        Box::new(Dielectric::new(1.5))
    )));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col_left = Vec3::new(0.0, 0.0, 0.0);
            let mut col_right = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..ns {
                let mut rng = rng();
                let u = (i as f64 + rng.random_range(0f64..1f64)) / nx as f64;
                let v = (j as f64 + rng.random_range(0f64..1f64)) / ny as f64;

                let r_left = camera.left_eye.get_ray(u, v);
                col_left += color(&r_left, &world, 50);

                let r_right = camera.right_eye.get_ray(u, v);
                col_right += color(&r_right, &world, 50);
            }

            col_left /= ns as f64;
            col_right /= ns as f64;

            col_left = Vec3::new(col_left.x.sqrt(), col_left.y.sqrt(), col_left.z.sqrt());
            col_right = Vec3::new(col_right.x.sqrt(), col_right.y.sqrt(), col_right.z.sqrt());

            let ir = (255.99 * col_left.r()) as i32;
            let ig = (255.99 * col_right.g()) as i32;
            let ib = (255.99 * col_right.b()) as i32;

            writeln!(file, "{} {} {}", ir, ig, ib).unwrap();
        }
    }
}
