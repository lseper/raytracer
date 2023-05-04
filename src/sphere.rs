use crate::renderable::{Renderable, HitRecord};
use crate::util::{Point};
use crate::ray::Ray;

pub struct Sphere {
    pub center: Point,
    pub r: f32
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Sphere {
        Self { center, r: radius}
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
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> (bool, HitRecord) {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.len_squared() - self.r*self.r;
        let discriminant = (half_b*half_b) - (a*c);
        if discriminant < 0.0 { return (false, HitRecord::nothing())}

        // only return one of the roots, as we can't see through the sphere
        let root = (-half_b - f32::sqrt(discriminant)) / a;

        // find the root that lies within the range of values tmax, tmin can have
        if root < t_min || t_max < root {
            let root = (-half_b + f32::sqrt(discriminant)) / a;
            if root < t_min || t_max < root {
                // no solution within t bounds
                return (false, HitRecord::nothing());
            }
            let hr_point = ray.at(root);
            let normal = (hr_point - self.center) / self.r;
            let mut hit_record = HitRecord::new(hr_point, normal, root, false);
            hit_record.set_face_normal(ray, &normal);
            return (true, hit_record)
        }

        let hr_point = ray.at(root);
        let normal = (hr_point - self.center) / self.r;
        let mut hit_record = HitRecord::new(hr_point, normal, root, false);
        hit_record.set_face_normal(ray, &normal);
        return (true, hit_record);
    }
}