use crate::util::{Point, Vec3};

pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin + (self.direction * t)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn at_works_for_positive_values() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vec3::new(10.0, 10.0, 10.0));
        let t = 0.5;
        let r_at_actual = Point::new(5.0, 5.0, 5.0);
        assert_eq!(r_at_actual, r.at(t));
    }
    #[test]
    fn at_works_for_negative_values() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vec3::new(10.0, 10.0, 10.0));
        let t = -0.5;
        let r_at_actual = Point::new(-5.0, -5.0, -5.0);
        assert_eq!(r_at_actual, r.at(t));
    }
}
