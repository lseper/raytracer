use std::{cmp::Ordering};

use serde::{Deserialize, Serialize};

use crate::{util::{Interval, Point, random_int}, renderable::{Renderable, RenderableList, Object}};
use crate::ray::Ray;
use crate::renderable::HitRecord;


#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AABB {
    pub x_interval: Interval,
    pub y_interval: Interval,
    pub z_interval: Interval
}


impl AABB {
    pub fn empty() -> Self {
        Self {x_interval: Interval {min: 0.0, max: 0.0}, y_interval: Interval { min: 0.0, max: 0.0 }, z_interval: Interval { min: 0.0, max: 0.0}}
    }

    pub fn new_from_pts(a: Point, b: Point) -> Self {
        // create a bounding box via two "corners" of a box
        let x_interval = Interval { min: a.x().min(b.x()), max: a.x().max(b.x())};
        let y_interval = Interval { min: a.y().min(b.y()), max: a.y().max(b.y())};
        let z_interval = Interval { min: a.z().min(b.z()), max: a.z().max(b.z())};
        Self {x_interval, y_interval, z_interval}
    }

    pub fn new_from_bbox(a: Self, b: Self) -> AABB {
        Self {x_interval: Interval::new_from_intervals(a.x_interval, b.x_interval), y_interval: Interval::new_from_intervals(a.y_interval, b.y_interval), z_interval: Interval::new_from_intervals(a.z_interval
            , b.z_interval)}
    }

    pub fn axis(&self, n: i32) -> &Interval {
        match n {
            1 => &self.x_interval,
            2 => &self.y_interval,
            3 => &self.z_interval,
            _ => &self.z_interval
        }
    }

    pub fn hit_regular(&self, ray: &Ray, mut interval: Interval) -> (bool, HitRecord) {
        for a in 0..3 {
            let t0 = f32::min((self.axis(a).min - ray.origin[a as usize]) / ray.direction[a as usize], (self.axis(a).max - ray.origin[a as usize]) / ray.direction[a as usize]);
            let t1 = f32::max((self.axis(a).min - ray.origin[a as usize]) / ray.direction[a as usize], (self.axis(a).max - ray.origin[a as usize]) / ray.direction[a as usize]);
            interval.min = f32::max(t0, interval.min);
            interval.max = f32::min(t1, interval.max);
            if (interval.max <= interval.min) {
                return (false, HitRecord::nothing());
            }
        }
        (true, HitRecord::nothing())
    }

    // Optimization of ^, based on code from Andrew Kensler at Pixar
    pub fn hit_pixar_optimization(&self, ray: &Ray, mut interval: Interval) -> (bool, HitRecord) {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction[a as usize];
            let orig = ray.origin[a as usize];
            
            let mut t0 = (self.axis(a).min - orig) * inv_d;
            let mut t1 = (self.axis(a).max - orig) * inv_d;

            if (inv_d < 0.0) {
                // swap
                let temp = t0;
                t0 = t1;
                t1 = temp;
            }

            if (t0 > interval.min) {
                interval.min = t0;
            }
            if (t1 < interval.max) {
                interval.min = t1;
            }
            if (interval.max <= interval.min) {
                return (false, HitRecord::nothing());
            }
        }
        return (true, HitRecord::nothing());
    }
}

impl Renderable for AABB {
    fn hit(&self, ray: &Ray, interval: Interval) -> (bool, HitRecord) {
        self.hit_pixar_optimization(ray, interval)
    }
}

#[derive(Serialize, Deserialize)]
pub struct BvhNode {
    pub bbox: AABB,
    pub left: Option<Box<BvhNode>>,
    pub right: Option<Box<BvhNode>>
}

impl BvhNode {

    fn root() -> BvhNode {
        BvhNode { bbox: AABB::empty(), left: None, right: None }
    }

    pub fn new_from_renderables(list: &Vec<Object>) -> BvhNode {
        let mut root = BvhNode::root();
        root.new_from_renderables_with_index( list, 0, list.len());
        root
    }

    fn new_leaf(object: &Object) -> Option<Box<BvhNode>> {
        Some(Box::new(BvhNode { bbox: object.bounding_box(), left: None, right: None }))
    }

    fn new_from_renderables_with_index(&mut self, list: &Vec<Object>, start: usize, end: usize) {
        let axis = random_int(0, 2);

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            2 => Self::box_z_compare,
            _ => Self::box_z_compare
        };

        let object_span = end - start;
        // base case - just make two leaf nodes for the same object
        if object_span == 1 {
            self.left = Self::new_leaf(&list[start]);
            self.right = Self::new_leaf(&list[start]);
        }
        // two, assign accordingly
        else if object_span == 2 {
            // if the first is < the second, build tree as:
            //              root
            //              /  \
            //         first    second
            if Ordering::is_le(comparator(&list[start], &list[start + 1])) {
                self.left = Self::new_leaf(&list[start]);
                self.right = Self::new_leaf(&list[start + 1]);
            } else {
                self.right = Self::new_leaf(&list[start]);
                self.left = Self::new_leaf(&list[start + 1]);
            }
        } else {
            let mut rec_list = list[start..end].to_vec();
            rec_list.sort_by(|a, b| comparator(a, b));
            let mid = start + (object_span / 2);
            self.left = Some(Box::new(BvhNode::root()));
            self.right = Some(Box::new(BvhNode::root()));
            self.left.as_mut().expect("left childe is defined above").new_from_renderables_with_index(&rec_list, start, mid);
            self.right.as_mut().expect("right child is defined above").new_from_renderables_with_index(&rec_list, mid, end);
        }
        let left_bbox = match &self.left {
            Some(node) => node.bounding_box(),
            None => AABB::empty()
        };
        let right_bbox = match &self.right {
            Some(node) => node.bounding_box(),
            None => AABB::empty()
        };
        self.bbox = AABB::new_from_bbox(left_bbox, right_bbox);
        
    }

    fn box_compare(a: &Object, b: &Object, axis_index: i32) -> Ordering {
       let result = a.bounding_box().axis(axis_index).min < b.bounding_box().axis(axis_index).min;
       match result {
        true => Ordering::Less,
        false => Ordering::Greater
       }
    }

    fn box_x_compare(a: &Object, b: &Object) -> Ordering {
        Self::box_compare(a, b, 0)
    }
    fn box_y_compare(a: &Object, b: &Object) -> Ordering {
        Self::box_compare(a, b, 1)
    }
    fn box_z_compare(a: &Object, b: &Object) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Renderable for BvhNode {
    fn hit(&self, ray: &Ray, interval: Interval) -> (bool, HitRecord) {
        let (did_hit, _hit_rec) = self.bbox.hit(ray, interval);
        if !did_hit {
            return (false, HitRecord::nothing());
        }
        let (did_hit_left, left_hit_rec) = match &self.left {
            Some(node) => node.hit(ray, interval),
            None => (false, HitRecord::nothing())
        };
        let right_interval_max = if did_hit_left { left_hit_rec.t } else { interval.max };
        let (did_hit_right, right_hit_rec) = match &self.right {
            Some(node) => node.hit(ray, Interval { min: interval.min, max: right_interval_max}),
            None => (false, HitRecord::nothing())
        };
        if did_hit_left {
            return (did_hit_left, left_hit_rec);
        }
        (did_hit_right, right_hit_rec)

    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

// TODO: write tests for this file!
#[cfg(test)]
mod tests {
    use crate::{ray::Ray, util::{Point, Color}, sphere::Sphere, material::{LambertianMaterial, RenderableMaterial}};
    use super::*;

    #[test]
    fn should_hit_aabb_of_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Point::new(10.0, 0.0, 0.0));
        let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
        let bbox = Sphere::new(Point::new(0.0, 5.0, 0.0), 5.0, material ).bbox;
        let (did_hit, _) = bbox.hit(&r, Interval{min: 0.0, max: 10.0});
        assert!(did_hit)
    }

    #[test]
    fn should_not_hit_aabb_of_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Point::new(10.0, 0.0, 0.0));
        let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
        let bbox = Sphere::new(Point::new(5.0, 0.0, 0.0), 1.0, material ).bbox;
        let (did_hit, _) = bbox.hit(&r, Interval{min: 0.0, max: 10.0});
        assert!(!did_hit)
    }

    #[test]
    fn creating_bvh_node_from_single_renderable_works() {
        let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
        let sphere = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0, material);
        let renderables = vec![Object::Sphere(sphere)];

        let root = BvhNode::new_from_renderables(&renderables);
        let left_exists = match root.left {
            None => false,
            Some(_n) => true
        };
        let right_exists = match root.right {
            None => false,
            Some(_n) => true
        };
        assert!(left_exists && right_exists);
    }

    #[test]
    fn creating_bvh_node_from_two_renderables_within_eachother_works() {
        let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
        // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
        // a lower min for an axis = left
        let sphere1 = Sphere::new(Point::new(0.0, 0.0, 0.0), 3.0, material);
        let sphere2 = Sphere::new(Point::new(0.0, 0.5, 1.0), 1.0, material);
        let renderables = vec![Object::Sphere(sphere1), Object::Sphere(sphere2)];

        let root = BvhNode::new_from_renderables(&renderables);
        let left_child_bbox = root.left.expect("root left child should be assigned").bbox;
        let right_child_bbox = root.right.expect("root right child should be assigned").bbox;
        let result = left_child_bbox == sphere1.bbox && right_child_bbox == sphere2.bbox;
        assert!(result)
    }

    #[test]
    fn creating_bvh_node_from_two_renderables_separated_works() {
        let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
        // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
        // a lower min for an axis = left
        let sphere1 = Sphere::new(Point::new(0.0, 0.0, 0.0), 3.0, material);
        let sphere2 = Sphere::new(Point::new(1.0, 4.0, 4.0), 1.0, material);
        let renderables = vec![Object::Sphere(sphere1), Object::Sphere(sphere2)];

        let root = BvhNode::new_from_renderables(&renderables);
        let left_child_bbox = root.left.expect("root left child should be assigned").bbox;
        let right_child_bbox = root.right.expect("root right child should be assigned").bbox;
        let result = left_child_bbox == sphere1.bbox && right_child_bbox == sphere2.bbox;
        assert!(result)
    }

    #[test]
    fn creating_bvh_node_from_multiple_renderables_separated_works_total_bbox() {
        let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
        // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
        // a lower min for an axis = left
        /*
         * Scene structire: 
         * y
         * y ______
         * y|      |      __
         * y|   A  |     |B_| z = 
         * y|______|     
         * x/z -----------------------                         
         * y            _____
         * y           |  C  |
         * y           |_____|
         */  
        let sphere_a = Sphere::new(Point::new(0.0, 1.0, 0.0), 3.0, material);
        let sphere_b = Sphere::new(Point::new(3.0, 1.0, 3.0), 1.0, material);
        let sphere_c = Sphere::new(Point::new(1.0, -2.0, -2.0), 0.5, material);
        let renderables = vec![Object::Sphere(sphere_a), Object::Sphere(sphere_b), Object::Sphere(sphere_c)];

        let root = BvhNode::new_from_renderables(&renderables);

        // sphere bbox are calculated and assigned accordingly
        // let child_left_bbox = root.left.expect("should be sphere_a").bbox;
        // let child_right = root.right.expect("should be a BvhNode");
        // let child_right_left_bbox = child_right.left.expect("should be sphere c").bbox;
        // let child_right_right_bbox = child_right.right.expect("should be sphere b").bbox;

        // total bbox is assigned accordingly
        let expected_total_bbox = AABB::new_from_pts(Point::new(-3.0, -2.5, -3.0), Point::new(4.0, 4.0, 4.0));
        let actual_total_bbox = root.bbox;

        assert_eq!(expected_total_bbox, actual_total_bbox);
    }

    #[test]
    fn creating_bvh_node_from_multiple_renderables_separated_works_child_bvh_node_bbox() {
        let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
        // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
        // a lower min for an axis = left
        /*
         * Scene structire: 
         * y
         * y ______
         * y|      |      __
         * y|   A  |     |B_| z = 
         * y|______|     
         * x/z -----------------------                         
         * y            _____
         * y           |  C  |
         * y           |_____|
         */  
        let sphere_a = Sphere::new(Point::new(0.0, 0.0, 0.0), 3.0, material);
        let sphere_b = Sphere::new(Point::new(3.0, 1.0, 3.0), 1.0, material);
        let sphere_c = Sphere::new(Point::new(1.0, -2.0, -2.0), 0.5, material);
        let renderables = vec![Object::Sphere(sphere_a), Object::Sphere(sphere_b), Object::Sphere(sphere_c)];

        let root = BvhNode::new_from_renderables(&renderables);

        // sphere bbox are calculated and assigned accordingly
        // let child_left_bbox = root.left.expect("should be sphere_a").bbox;
        let child_right = root.right.expect("should be a BvhNode");
        // let child_right_left_bbox = child_right.left.expect("should be sphere c").bbox;
        // let child_right_right_bbox = child_right.right.expect("should be sphere b").bbox;

        // total bbox is assigned accordingly
        let expected_right_child_bbox = AABB::new_from_pts(Point::new(0.5, -2.5, -2.5), Point::new(4.0, 2.0, 4.0));
        let actual_right_child_bbox = child_right.bbox;

        assert_eq!(expected_right_child_bbox, actual_right_child_bbox);
    }

    #[test]
    fn creating_bvh_node_from_multiple_renderables_separated_works_left_leaf_child_bbox() {
        let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
        // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
        // a lower min for an axis = left
        /*
         * Scene structire: 
         * y
         * y ______
         * y|      |      __
         * y|   A  |     |B_| z = 
         * y|______|     
         * x/z -----------------------                         
         * y            _____
         * y           |  C  |
         * y           |_____|
         */  
        let sphere_a = Sphere::new(Point::new(0.0, 0.0, 0.0), 3.0, material);
        let sphere_b = Sphere::new(Point::new(3.0, 1.0, 3.0), 1.0, material);
        let sphere_c = Sphere::new(Point::new(1.0, -2.0, -2.0), 0.5, material);
        let renderables = vec![Object::Sphere(sphere_a), Object::Sphere(sphere_b), Object::Sphere(sphere_c)];

        let root = BvhNode::new_from_renderables(&renderables);

        // sphere bbox are calculated and assigned accordingly
        let child_left_bbox = root.left.expect("should be sphere_a").bbox;
        // let child_right = root.right.expect("should be a BvhNode");
        // let child_right_left_bbox = child_right.left.expect("should be sphere c").bbox;
        // let child_right_right_bbox = child_right.right.expect("should be sphere b").bbox;

        // total bbox is assigned accordingly
        let expected_left_child_bbox = AABB::new_from_pts(Point::new(0.0, 0.0, 0.0), Point::new(3.0, 3.0, 3.0));

        assert_eq!(expected_left_child_bbox, child_left_bbox);
    }

    #[test]
    fn creating_bvh_node_from_multiple_renderables_separated_works_subtree_left_child_bbox() {
        let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
        // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
        // a lower min for an axis = left
        /*
         * Scene structire: 
         * y
         * y ______
         * y|      |      __
         * y|   A  |     |B_| z = 
         * y|______|     
         * x/z -----------------------                         
         * y            _____
         * y           |  C  |
         * y           |_____|
         */  
        let sphere_a = Sphere::new(Point::new(0.0, 0.0, 0.0), 3.0, material);
        let sphere_b = Sphere::new(Point::new(3.0, 1.0, 3.0), 1.0, material);
        let sphere_c = Sphere::new(Point::new(1.0, -2.0, -2.0), 0.5, material);
        let renderables = vec![Object::Sphere(sphere_a), Object::Sphere(sphere_b), Object::Sphere(sphere_c)];

        let root = BvhNode::new_from_renderables(&renderables);

        // sphere bbox are calculated and assigned accordingly
        // let child_left_bbox = root.left.expect("should be sphere_a").bbox;
        let child_right = root.right.expect("should be a BvhNode");
        let child_right_left_bbox = child_right.left.expect("should be sphere c").bbox;
        // let child_right_right_bbox = child_right.right.expect("should be sphere b").bbox;

        // total bbox is assigned accordingly
        let expected_left_child_bbox = sphere_c.bbox;

        assert_eq!(expected_left_child_bbox, child_right_left_bbox);
    }

    #[test]
    fn creating_bvh_node_from_multiple_renderables_separated_works_subtree_right_child_bbox() {
        let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
        // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
        // a lower min for an axis = left
        /*
         * Scene structire: 
         * y
         * y ______
         * y|      |      __
         * y|   A  |     |B_| z = 
         * y|______|     
         * x/z -----------------------                         
         * y            _____
         * y           |  C  |
         * y           |_____|
         */  
        let sphere_a = Sphere::new(Point::new(0.0, 0.0, 0.0), 3.0, material);
        let sphere_b = Sphere::new(Point::new(3.0, 1.0, 3.0), 1.0, material);
        let sphere_c = Sphere::new(Point::new(1.0, -2.0, -2.0), 0.5, material);
        let renderables = vec![Object::Sphere(sphere_a), Object::Sphere(sphere_b), Object::Sphere(sphere_c)];

        let root = BvhNode::new_from_renderables(&renderables);

        // sphere bbox are calculated and assigned accordingly
        // let child_left_bbox = root.left.expect("should be sphere_a").bbox;
        let child_right = root.right.expect("should be a BvhNode");
        // let child_right_left_bbox = child_right.left.expect("should be sphere c").bbox;
        let child_right_right_bbox = child_right.right.expect("should be sphere b").bbox;

        // total bbox is assigned accordingly
        let expected_right_child_bbox = sphere_b.bbox;

        assert_eq!(expected_right_child_bbox, child_right_right_bbox);
    }

    // TODO: write tests regarding hit detection with BvHNode World


}