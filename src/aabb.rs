use serde::{Deserialize, Serialize};

use crate::{util::{Interval, Point}, renderable::Renderable};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct AABB {
    pub x_interval: Interval,
    pub y_interval: Interval,
    pub z_interval: Interval
}


impl AABB {
    pub fn empty() -> Self {
        Self {x_interval: Interval {min: 0.0, max: 0.0}, y_interval: Interval { min: 0.0, max: 0.0 }, z_interval: Interval { min: 0.0, max: 0.0}}
    }

    pub fn new_from_pts(a: &Point, b: &Point) -> Self {
        // create a bounding box via two "corners" of a box
        let x_interval = Interval { min: a.x().min(b.x()), max: a.x().max(b.x())};
        let y_interval = Interval { min: a.y().min(b.y()), max: a.y().max(b.y())};
        let z_interval = Interval { min: a.z().min(b.z()), max: a.z().max(b.z())};
        Self {x_interval, y_interval, z_interval}
    }

    pub fn axis(&self, n: i32) -> &Interval {
        match n {
            1 => &self.x_interval,
            2 => &self.y_interval,
            3 => &self.z_interval,
            _ => &self.z_interval
        }
    }
}

// impl Renderable for AABB {
//     // for (int a = 0; a < 3; a++) {
//     //     auto t0 = fmin((axis(a).min - r.origin()[a]) / r.direction()[a],
//     //                    (axis(a).max - r.origin()[a]) / r.direction()[a]);
//     //     auto t1 = fmax((axis(a).min - r.origin()[a]) / r.direction()[a],
//     //                    (axis(a).max - r.origin()[a]) / r.direction()[a]);
//     //     ray_t.min = fmax(t0, ray_t.min);
//     //     ray_t.max = fmin(t1, ray_t.max);
//     //     if (ray_t.max <= ray_t.min)
//     //         return false;
//     // }
//     // return true;
//     pub fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> (bool, crate::renderable::HitRecord) {
//         for a in 0..3 {
//             let t0 = f32::min((self.axis(a).min - ray.origin[a]) / ray.direction[a], (self.axis(a).max - ray.origin[a]) / ray.direction[a]);
//             let t1 = f32::min((self.axis(a).min - ray.origin[a]) / ray.direction[a], (self.axis(a).max - ray.origin[a]) / ray.direction[a]);
//         }
//     }
// }