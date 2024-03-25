// use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::aabb::AABB;
use crate::material::RenderableMaterial;
use crate::ray::Ray;
use crate::renderable::{HitRecord, Renderable};
use crate::util::{Point, Vec3, Interval};
use crate::texture::{RenderableTexture, SolidColor};
// use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Sphere {
    pub center: Point,
    pub r: f32,
    pub material: RenderableMaterial,
    pub is_moving: bool,
    pub center_vec: Vec3,
    pub bbox: AABB,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, material: RenderableMaterial) -> Sphere {
        // aabb(center1 - rvec, center1 + rvec);
        let radius_vec = Vec3::new(radius, radius, radius);
        Self {
            center,
            r: radius,
            material,
            is_moving: false,
            center_vec: Vec3::zero(),
            bbox: AABB::new_from_pts(center - radius_vec, center + radius_vec)
        }
    }

    pub fn new_moving(center: Point, radius: f32, material: RenderableMaterial, center2: Point) -> Sphere {
        let radius_vec = Vec3::new(radius, radius, radius);
        let start_box = AABB::new_from_pts(center - radius_vec, center + radius_vec);
        let end_box = AABB::new_from_pts(center2 - radius_vec, center2 + radius_vec);
        Self {
            center,
            r: radius,
            material,
            is_moving: true,
            center_vec: center2 - center,
            bbox: AABB::new_from_bbox(start_box, end_box)
        }
    }

    pub fn sphere_center(&self, time: f32) -> Point {
        self.center + (time * self.center_vec) 
    }
}

impl Renderable for Sphere {
    /**
     * CALCULATING IF IT HITS THE SPHERE AT ALL
     *
     * Does this by seeing if the ray we're casting from the camera is intersecting the sphere at all
     * We know this by having a sphere be centered at a point C = (cx, cy, cz), with a radius r
     * Any point along this sphere satisfies the equation (x - cx)^2 + (y - cy)^2 + (z - cz)^2 = r^2
     * For simplicity sake, let P = (x,y,z) so then we have a sphere represented as (P - C)^2 = r^2
     * Or rather, (P - C) * (P - C) = r^2
     *
     * As we're casting a ray, defined as R = A + tB, we can represent a point on the ray as p(t) = A + tB
     * So we want to know if along that ray denoted by A + tB, if there's any intersection with the sphere
     * So we substitute A + tB in for P in our equation, as we want to know if one of those points P on the sphere
     *  is contained in our ray's path
     * Thus, we get (A + tB) * (A + tB) = r^2
     * The only unknown in this equation is t. When t has no solutions, then we know no possible point in our ray A + tB
     *  lies on the surface of the sphere.
     * If t has exactly one solution, then our ray tangentially hits the edge of the sphere (there's exactly one point)
     * If t has two solutions, then our ray goes through the sphere, and there are two points along the surface where this ray occurs.
     *
     * So, we can expand the equation algebraically, and move everthing over to the left hand side. We get the equation:
     *  t^2 * (b * b) + 2t * b * (A - C) + (A - C) * (A - C) = 0
     *      a                   b                   c  
     * So we can use our good ol' friend the quadradic formula so solve for the real roots, -b +- sqrt(b^2 - 4ac) / 2a
     * so, a ray cast from our camera (P) hits the sphere if there's one, or two real solutions. Or rather, if the
     * stuff under the square root is positive.
     *
     * NOTE: two slight performance improvements are made.
     *
     * The first optimization is that we know that a vector dotted with itself is equal to the length of the vector squared
     * so any instance where we have vec.dot(vec) we can replace with vec.len_squared()
     *
     * The second optimization is in the calculation of the b component in the quadradic formula. As b is defined as
     * 2t * b * (A - C), we can think of it as 2 * h, where h = t * b * (A - C). The b term in the quadratic formula simplifies as:
     *      (-b +- sqrt(b^2 - 4ac)) / 2a
     *    = (-(2h) +- sqrt((2h)^2 - 4ac)) / 2a ----- substitution
     *    = (-2h +- sqrt((4h^2) - 4ac)) / 2a ------  squaring 2
     *    = -2h +- 2sqrt(h^2 - ac) / 2a ----- 4 is a like term for both 4h^2 and 4ac, pull it out from under the radical
     *    = 2(-h +- sqrt(h^2 - ac)) / 2a ------ 2 is a like term for both -2h and 2*(radical) in numerator, can distribute it out
     *    = -h +- sqrt(h^2 - ac) / a ------ 2's in numerator and denominator cancel out
     *
     * CALCULATING SURFACE NORMALS
     * if our ray does hit the sphere at some point P, we can calculate what the normal vector (perpendicular to the point of intersection)
     * looks like. It is simply the point of intersection minus the center of the sphere. Think about it as we're taking the point on the sphere,
     * pointing directly at the center, and then spinning 180 degrees to point the exact opposite way.
     */
    fn hit(&self, ray: &Ray, interval: Interval) -> (bool, HitRecord) {
        let center = if self.is_moving { self.sphere_center(ray.time) } else {self.center};
        let oc = ray.origin - center;
        let a = ray.direction.len_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.len_squared() - (self.r * self.r);
        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0.0 {
            return (false, HitRecord::nothing());
        }

        // only return one of the roots, as we can't see through the sphere
        let root = (-half_b - f32::sqrt(discriminant)) / a;

        // find the root that lies within the range of values tmax, tmin can have
        if root < interval.min || interval.max < root {
            let root = (-half_b + f32::sqrt(discriminant)) / a;
            if root < interval.min || interval.max < root {
                // no solution within t bounds
                return (false, HitRecord::nothing());
            }
            let hr_point = ray.at(root);
            let normal = (hr_point - center) / self.r;
            // TODO: actually set correct u, v values
            let mut hit_record = HitRecord::new(hr_point, normal, root, 0.0, 0.0, false, self.material);
            hit_record.set_face_normal(ray, &normal);
            return (true, hit_record);
        }

        let hr_point = ray.at(root);
        let normal = (hr_point - center) / self.r;
        let mut hit_record = HitRecord::new(hr_point, normal, root, 0.0, 0.0, false, self.material);
        hit_record.set_face_normal(ray, &normal);
        return (true, hit_record);
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.is_moving && other.is_moving && self.center_vec == other.center_vec && self.bbox == other.bbox && self.r == other.r && self.material == other.material
    }
}

#[cfg(test)]
mod tests {
    use crate::{util::Color, material::LambertianMaterial};

    use super::*;

    #[test]
    fn sphere_is_hit_when_ray_cast_hitting_directly() {
        let material: RenderableMaterial = RenderableMaterial::Lambertian(LambertianMaterial::new(RenderableTexture::SolidColor(SolidColor::from_values(0.0, 0.0, 0.0))));
        let sphere_a = Sphere::new(Point::new(-5.0, -5.0, 0.0), 2.0, material);
        let r = Ray::new(Point::new(-5.0, -5.0, -5.0), Point::new(-5.0, -5.0, 0.0));
        let (did_hit, _actual_hit_record) = sphere_a.hit(&r, Interval{min: 0.0, max:10.0});
        assert!(did_hit)
    }
}

// impl fmt::Display for Sphere {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{{\n\t\"center\": {{\n\t\t\"vec\": {} \n\t}},\n\t\"r\": {},\n\t\"material\": {{\n{}\n\t}} \n}},\n", self.center, self.r, self.material)
//     }
// }
