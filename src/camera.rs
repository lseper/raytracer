use crate::util::{Point, Vec3, degrees_to_radians};
use crate::ray::Ray;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(look_from: Point, look_at: Point, vup: Vec3, vfov: f32, aspect_ratio: f32, aperature: f32, focus_distance: f32) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = Vec3::cross(vup, w).unit_vector();
        let v = Vec3::cross(w, u);

        eprintln!("w: {}\nu: {}\nv: {}", w, u, v);

        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - (focus_distance * w);

        let lens_radius = aperature / 2.0;
        Self { origin, horizontal, lower_left_corner, vertical, u, v, w, lens_radius }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        // start ray from random spot in aperature
        let disk_samp = self.lens_radius * Vec3::random_in_unit_disk();
        // calculate the offset in reference to the origin (this ties back to how u, v are calculated in constructor)
        let offset = (self.u * disk_samp.x()) + (self.v * disk_samp.y());
        Ray::new(self.origin + offset, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin - offset)
    }
}