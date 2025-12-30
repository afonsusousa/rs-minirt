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

    let nx = 1200;
    let ny = 675;
    let ns = 100;

    writeln!(file, "P3\n{} {}\n255", nx, ny).unwrap();

    let mut world = HittableList::new();

    // Ground
    let ground_material = Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    let mut rng = rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.random_range(0.0..1.0);
            let center = Vec3::new(
                a as f64 + 0.9 * rng.random_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.random_range(0.0..1.0)
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    world.add(Box::new(Sphere::new(center, 0.2, Box::new(Lambertian::new(albedo)))));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = rng.random_range(0.0..0.5);
                    world.add(Box::new(Sphere::new(center, 0.2, Box::new(Metal::new(albedo, fuzz)))));
                } else {
                    world.add(Box::new(Sphere::new(center, 0.2, Box::new(Dielectric::new(1.5)))));
                }
            }
        }
    }

    // Big Spheres
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Box::new(Dielectric::new(1.5)))));
    world.add(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))))));
    world.add(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)))));

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let ipd = 0.06;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        nx as f64 / ny as f64,
        ipd,
        0.6,
        dist_to_focus
    );

    for j in (0..ny).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..nx {
            let mut col_left = Vec3::new(0.0, 0.0, 0.0);
            let mut col_right = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..ns {
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
    eprintln!("\nDone.");
}
