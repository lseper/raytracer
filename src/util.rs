use core::fmt;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;
use std::{fmt::Display, ops};

pub fn degrees_to_radians(degrees: f32) -> f32 {
    return (degrees * PI) / 180.0;
}

pub fn random_between_0_1() -> f32 {
    let mut rng = rand::thread_rng();
    let random_float = rng.gen();
    random_float
}

pub fn random_in_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random_between_0_1()
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Vec3 {
    vec: [f32; 3],
}

pub type Color = Vec3;
pub type Point = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { vec: [x, y, z] }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            vec: [0.0, 0.0, 0.0],
        }
    }

    pub fn random(min: f32, max: f32) -> Point {
        Vec3 {
            vec: [
                random_in_range(min, max),
                random_in_range(min, max),
                random_in_range(min, max),
            ],
        }
    }

    pub fn random_in_unit_sphere() -> Point {
        loop {
            let p: Point = Point::random(-1.0, 1.0);
            if p.len_squared() <= 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Point {
        Point::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_unit_disk() -> Point {
        loop {
            let p = Point::new(random_between_0_1(), random_between_0_1(), 0.0);
            if p.len_squared() <= 1.0 {
                return p;
            }
        }
    }

    pub fn near_zero(&self) -> bool {
        let e: f32 = 1e-8;
        (f32::abs(self.x()) < e) && (f32::abs(self.y()) < e) && (f32::abs(self.z()) < e)
    }

    // where n is a unit vector, and v assumedly points opposite direction as n
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - (2.0 * (v.dot(n) * n))
    }

    /**
     * This come from Snell's Law, which describes the angles regarding a reflecting ray (R) and a refracted ray (R')
     * n * sin(theta) = n' * sin(theta')
     * n is the refractive index of the material the ray is passing through when it hits the surface (usually air)
     * n' is the refractive index of the material the ray is passing through once it starts refracting
     * theta is the angle the incoming ray hits the surface at, with respect to the normal (so 0 <= theta <= 90)
     * theta' is the angle that the refracted ray refracts at with respect to the normal (so 0 <= theta <= 90)
     *
     * We know then that the refracted ray (R') is composed of two components: R_perp and R_para, where
     * R_para is the component of R' that is parallel to the normal (n)
     * R_perp is the component of R' that is perpendicular to the normal (n)
     *
     * Solving these yields:
     * R_perp = 𝜂/𝜂' * (R + cos(theta) * n)
     * R_para = -sqrt(1 - R_perp^2 * n)
     *
     * for cos(theta), we use the idea that for any two vectors a, b, we know that
     *  a dot b = |a||b|cos(theta)
     *
     * If we assume that a and b are both unit vectors, we thus get
     * a dot b = cos(theta)
     *
     * and as the incoming vector (R) and our normal (n) are what directly composes theta, we then get
     * cos(theta) = R dot n
     */
    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
        // have to negate uv in this as it is ??
        let cos_theta = f32::min(-uv.dot(n), 1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_para = -f32::sqrt(f32::abs(1.0 - r_out_perp.len_squared())) * n;
        r_out_perp + r_out_para
    }

    pub fn x(&self) -> f32 {
        self.vec[0]
    }

    pub fn y(&self) -> f32 {
        self.vec[1]
    }

    pub fn z(&self) -> f32 {
        self.vec[2]
    }

    fn get(&self, index: usize) -> Option<&f32> {
        self.vec.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut f32> {
        self.vec.get_mut(index)
    }

    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f32 {
        (self[0] * self[0]) + (self[1] * self[1]) + (self[2] * self[2])
    }

    pub fn dot(&self, b: Vec3) -> f32 {
        (self[0] * b[0]) + (self[1] * b[1]) + (self[2] * b[2])
    }

    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        Vec3 {
            vec: [
                (a[1] * b[2]) - (a[2] * b[1]),
                (a[2] * b[0]) - (a[0] * b[2]),
                (a[0] * b[1]) - (a[1] * b[0]),
            ],
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.len()
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        return self.get(i).unwrap();
    }
}
impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        return self.get_mut(i).unwrap();
    }
}
impl std::cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        (self.vec[0] == other.vec[0])
            && (self.vec[1] == other.vec[1])
            && (self.vec[2] == other.vec[2])
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            vec: [-self[0], -self[1], -self[2]],
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, b: Vec3) -> Vec3 {
        Vec3 {
            vec: [self[0] + b[0], self[1] + b[1], self[2] + b[2]],
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, b: Vec3) -> Vec3 {
        Vec3 {
            vec: [self[0] - b[0], self[1] - b[1], self[2] - b[2]],
        }
    }
}
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 {
            vec: [self[0] * scalar, self[1] * scalar, self[2] * scalar],
        }
    }
}
impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, scalar: f32) -> Vec3 {
        let frac: f32 = 1.0 / scalar;
        self * frac
    }
}
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {
            vec: [self * vec[0], self * vec[1], self * vec[2]],
        }
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, b: Vec3) -> Vec3 {
        Vec3 {
            vec: [self[0] * b[0], self[1] * b[1], self[2] * b[2]],
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, b: Vec3) {
        self[0] += b[0];
        self[1] += b[1];
        self[2] += b[2];
    }
}
impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, b: Vec3) {
        self[0] -= b[0];
        self[1] -= b[1];
        self[2] -= b[2];
    }
}
impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, scalar: f32) {
        self[0] /= scalar;
        self[1] /= scalar;
        self[2] /= scalar;
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self[0], self[1], self[2])
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn length_squared_is_correct() {
        let v = Vec3::new(2.0, 3.0, 1.0);
        let actual = 14.0;
        assert_eq!(v.len_squared(), actual)
    }

    #[test]
    fn euclidean_length_is_correct() {
        let v = Vec3::new(2.0, 3.0, 1.0);
        let actual = f32::sqrt(14.0);
        assert_eq!(v.len(), actual)
    }

    #[test]
    fn unit_vector_is_correct() {
        let v = Vec3::new(2.0, 3.0, 1.0);
        let length = f32::sqrt(14.0);
        let actual_unit_v = Vec3::new(2.0 / length, 3.0 / length, 1.0 / length);
        assert_eq!(v.unit_vector(), actual_unit_v);
    }

    #[test]
    fn negating_vector_is_correct() {
        let v = Vec3::new(2.0, 3.0, 1.0);
        let v_neg_actual = Vec3::new(-2.0, -3.0, -1.0);
        assert_eq!(-v, v_neg_actual);
    }

    #[test]
    fn adding_vectors_is_correct() {
        let v1 = Vec3::new(2.0, 3.0, 1.0);
        let v2: Vec3 = Vec3::new(1.0, 0.0, -1.0);

        let actual = Vec3::new(3.0, 3.0, 0.0);
        assert_eq!(v1 + v2, actual);
    }
    #[test]
    fn subtracting_vectors_is_correct() {
        let v1 = Vec3::new(2.0, 3.0, 1.0);
        let v2: Vec3 = Vec3::new(1.0, 0.0, -1.0);

        let actual = Vec3::new(1.0, 3.0, 2.0);
        assert_eq!(v1 - v2, actual);
    }
    #[test]
    fn dividing_vectors_by_scalar_is_correct() {
        let v1 = Vec3::new(2.0, 4.0, 1.0);
        let scalar: f32 = 2.0;

        let actual = Vec3::new(1.0, 2.0, 0.5);
        assert_eq!(v1 / scalar, actual);
    }
    #[test]
    fn multiplying_vectors_by_scalar_is_correct() {
        let v1 = Vec3::new(2.0, 3.0, 1.0);
        let scalar: f32 = 2.0;

        let actual = Vec3::new(4.0, 6.0, 2.0);
        assert_eq!(v1 * scalar, actual);
    }
    #[test]
    fn multiplying_vectors_together_is_correct() {
        let v1 = Vec3::new(2.0, 3.0, 1.0);
        let v2: Vec3 = Vec3::new(3.0, 0.5, -1.0);

        let actual = Vec3::new(6.0, 1.5, -1.0);
        assert_eq!(v1 * v2, actual);
    }
    #[test]
    fn dot_product_is_correct() {
        let v1 = Vec3::new(2.0, 3.0, 1.0);
        let v2: Vec3 = Vec3::new(3.0, 0.5, -1.0);

        let actual: f32 = 6.5;
        assert_eq!(v1.dot(v2), actual);
    }
    #[test]
    fn cross_product_is_correct() {
        let v1 = Vec3::new(2.0, 3.0, 1.0);
        let v2: Vec3 = Vec3::new(3.0, 0.5, -1.0);

        let actual = Vec3::new(-3.5, 5.0, -8.0);
        assert_eq!(Vec3::cross(v1, v2), actual);
    }
}
