use std::io::Write;
use rand::{rng, Rng};
use crate::camera::Camera;
use crate::color::color;
use crate::hits::HittableList;
use crate::vec3::Vec3;

pub struct Render {
    nx: i32,
    ny: i32,
    ns: i32,
    lookat: Vec3,
    lookfrom: Vec3,
    vup: Vec3,
    vfov: f64,
    ipd: f64,
    aperture: f64,
    focus_dist: f64,
}

impl Render {
    pub fn new(nx: i32, ny: i32, ns: i32) -> Render {
        Render {
            nx,
            ny,
            ns,
            lookat: Vec3::new(0.0, 0.0, -1.0),
            lookfrom: Vec3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            vfov: 20.0,
            ipd: 0.06,
            aperture: 0.0,
            focus_dist: 10.0,
        }
    }

    pub fn lookat(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
        self.lookat = Vec3::new(x, y, z);
        self
    }

    pub fn lookfrom(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
        self.lookfrom = Vec3::new(x, y, z);
        self
    }

    pub fn vup(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
        self.vup = Vec3::new(x, y, z);
        self
    }

    pub fn vfov(&mut self, vfov: f64) -> &mut Self {
        self.vfov = vfov;
        self
    }

    pub fn ipd(&mut self, ipd: f64) -> &mut Self {
        self.ipd = ipd;
        self
    }

    pub fn aperture(&mut self, aperture: f64) -> &mut Self {
        self.aperture = aperture;
        self
    }

    pub fn focus_dist(&mut self, focus_dist: f64) -> &mut Self {
        self.focus_dist = focus_dist;
        self
    }

    pub fn render_scene(&self, world: HittableList, anaglyph: bool) {

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("out.ppm")
            .unwrap();

        writeln!(file, "P3\n{} {}\n255", self.nx, self.ny).unwrap();

        let mut rng = rng();

        let camera = Camera::new(
            self.lookfrom,
            self.lookat,
            self.vup,
            self.vfov,
            self.nx as f64 / self.ny as f64,
            self.ipd,
            self.aperture,
            self.focus_dist
        );

        for j in (0..self.ny).rev() {
            eprint!("\rScanlines remaining: {} ", j);
            for i in 0..self.nx {
                let mut col_left = Vec3::new(0.0, 0.0, 0.0);
                let mut col_right = Vec3::new(0.0, 0.0, 0.0);

                for _ in 0..self.ns {
                    let u = (i as f64 + rng.random_range(0f64..1f64)) / self.nx as f64;
                    let v = (j as f64 + rng.random_range(0f64..1f64)) / self.ny as f64;

                    if anaglyph {
                        let r_left = camera.left_eye.get_ray(u, v);
                        col_left += color(&r_left, &world, 50);
                    }

                    let r_right = camera.right_eye.get_ray(u, v);
                    col_right += color(&r_right, &world, 50);
                }

                if anaglyph{ col_left /= self.ns as f64;}
                col_right /= self.ns as f64;

                if anaglyph { col_left = Vec3::new(col_left.x.sqrt(), col_left.y.sqrt(), col_left.z.sqrt()); }
                col_right = Vec3::new(col_right.x.sqrt(), col_right.y.sqrt(), col_right.z.sqrt());

                let ir = (255.99 * if anaglyph {col_left.r()} else {col_right.r()}) as i32;
                let ig = (255.99 * col_right.g()) as i32;
                let ib = (255.99 * col_right.b()) as i32;

                writeln!(file, "{} {} {}", ir, ig, ib).unwrap();
            }
        }
        eprintln!("\nDone.");
    }
}
