use std::f32::consts::PI;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Eye {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub lens_radius: f32,
}

impl Eye {
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}

pub struct Camera {
    pub left_eye: Eye,
    pub right_eye: Eye,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        ipd: f32,
        defocus_angle: f32,
        focus_dist: f32
    ) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let lens_radius = focus_dist * (defocus_angle * PI / 180.0 / 2.0).tan();

        let half_ipd = ipd / 2.0;

        let left_origin = lookfrom - u * half_ipd;
        let left_eye = Eye {
            origin: left_origin,
            lower_left_corner: left_origin - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist,
            horizontal: u * 2.0 * half_width * focus_dist,
            vertical: v * 2.0 * half_height * focus_dist,
            u,
            v,
            lens_radius,
        };

        let right_origin = lookfrom + u * half_ipd;
        let right_eye = Eye {
            origin: right_origin,
            lower_left_corner: right_origin - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist,
            horizontal: u * 2.0 * half_width * focus_dist,
            vertical: v * 2.0 * half_height * focus_dist,
            u,
            v,
            lens_radius,
        };

        Camera {
            left_eye,
            right_eye,
        }
    }
}
