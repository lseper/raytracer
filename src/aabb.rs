use std::{cmp::Ordering, mem::swap};

use serde::{Deserialize, Serialize};

use crate::{util::{Interval, Point, random_int}, renderable::{Renderable, RenderableList, Object}, sphere::Sphere};
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
            if interval.max <= interval.min {
                return (false, HitRecord::nothing());
            }
        }
        (true, HitRecord::nothing())
    }

    fn handle_zero(direction_cord: f32) -> f32 {
        if f32::abs(direction_cord) < f32::EPSILON {
            if f32::is_sign_negative(direction_cord) {
                return f32::NEG_INFINITY;
            }
            return f32::INFINITY;
        }
        1.0 / direction_cord
    }

    // Optimization of ^, based on code from Andrew Kensler at Pixar
    pub fn hit_pixar_optimization(&self, ray: &Ray, mut interval: Interval) -> (bool, HitRecord) {
        for a in 0..3 {
            let inv_d = Self::handle_zero(ray.direction[a as usize]);
            let orig = ray.origin[a as usize];
            
            let mut t0 = (self.axis(a).min - orig) * inv_d;
            let mut t1 = (self.axis(a).max - orig) * inv_d;

            if inv_d < 0.0 {
                // swap
                swap(&mut t0, &mut t1);
            }

            if t0 > interval.min {
                interval.min = t0;
            }
            if t1 < interval.max {
                interval.min = t1;
            }
            if interval.max <= interval.min {
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

#[derive(Clone)]
pub struct BvhNode {
    pub bbox: Object,
    pub left: Option<Box<BvhNode>>,
    pub right: Option<Box<BvhNode>>
}

impl BvhNode {

    fn root() -> BvhNode {
        BvhNode { bbox: Object::empty(), left: None, right: None }
    }

    pub fn new_from_renderables(list: &Vec<Object>) -> BvhNode {
        let root = Self::new_from_renderables_with_index(list, 0, list.len());
        match root {
            Some(node) => *node,
            None => Self::root()
        }
    }

    fn new_from_renderables_with_index(list: &Vec<Object>, start: usize, end: usize)-> Option<Box<BvhNode>>{
        // eprintln!("start: {} end: {} len {}", start, end, list.len());
        let axis = random_int(0, 2);

        let object_span = end - start;
        // base case - just make two leaf nodes for the same object
        if object_span == 1 {
            return Some(Box::new(Self {
                bbox: list[start],
                left: None,
                right: None,
            }));
        }
        // two, assign accordingly
        else if object_span == 2 {
            // if the first is < the second, build tree as:
            //              root
            //              /  \
            //         first    second
            let first = Some(Box::new(Self {
                bbox: list[start],
                left: None,
                right: None
            }));
            let second = Some(Box::new(Self {
                bbox: list[start + 1],
                left: None,
                right: None,
            }));
            let parent_bbox = AABB::new_from_bbox(list[start].bounding_box(), list[start + 1].bounding_box());
            // TODO: since we only use bbox for checking ANY child node, no need for ordering here.
            return Some(Box::new(Self {
                bbox: Object::AABB(parent_bbox),
                left: first,
                right: second
            }));
        }
        // Copies the entire list of renderables - not the best performance wise, but slicing accordingly gave 
        // weird index OOB errors for SPECIFICALLY when the original list passed in was >= 6 objects ???
        let rec_list = list[..].to_vec();
        let mid = start + (f32::round(object_span as f32 / 2.0)) as usize;
        let left = Self::new_from_renderables_with_index(&rec_list, start, mid);
        let right = Self::new_from_renderables_with_index(&rec_list, mid, end);
        let bbox = match (&left, &right) {
            (Some(node_l), Some(node_r)) => AABB::new_from_bbox(node_l.bounding_box(), node_r.bounding_box()),
            (Some(node_l), None) => node_l.bounding_box(),
            (None, Some(node_r)) => node_r.bounding_box(),
            _ => AABB::empty()
        };
        Some(Box::new(Self {
            bbox: Object::AABB(bbox), left, right
        }))
        
    }

    fn box_compare(a: &Object, b: &Object, axis_index: i32) -> Ordering {
       let result = a.bounding_box().axis(axis_index).min <= b.bounding_box().axis(axis_index).min;
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

    fn hit_aabb(&self, aabb: AABB, ray: &Ray, interval: Interval) -> (bool, HitRecord) {
        let (did_hit, hit_rec) = aabb.hit(ray, interval);
        if !did_hit {
            return (false, HitRecord::nothing());
        }
        let (did_hit_left, left_hit_rec) = match &self.left {
            Some(node) => node.hit(ray, interval),
            None => (did_hit, hit_rec)
        };
        let right_interval_max = if did_hit_left { left_hit_rec.t } else { interval.max };
        let (did_hit_right, right_hit_rec) = match &self.right {
            Some(node) => node.hit(ray, Interval { min: interval.min, max: right_interval_max}),
            None => (did_hit, hit_rec.clone())
        };
        if did_hit_left {
            return (did_hit_left, left_hit_rec);
        }
        (did_hit_right, right_hit_rec)
    }

    fn hit_sphere(&self, sphere: Sphere, ray: &Ray, interval: Interval) -> (bool, HitRecord) {
        let (did_hit_aabb, hit_rec) = sphere.bbox.hit(ray, interval);
        if did_hit_aabb {
            let (hit_sphere, sphere_hit_rec) = sphere.hit(ray, interval);
            return (hit_sphere, sphere_hit_rec);
        }
        (did_hit_aabb, hit_rec)
    }
}

impl Renderable for BvhNode {
    fn hit(&self, ray: &Ray, interval: Interval) -> (bool, HitRecord) {
        let mut final_rec: HitRecord = HitRecord::nothing();
        let mut hit_anything: bool = false;
        let mut closest_yet = interval.max;

        let mut to_check = vec![self];
        while to_check.len() > 0 {
            let mut new_to_check: Vec<&BvhNode> = vec![];
            for node in &to_check[..] {
                let new_interval = Interval {min: interval.min, max: closest_yet};
                match node.bbox {
                    Object::AABB(aabb) => {
                        let (did_hit, _hit_rec) = aabb.hit(ray, new_interval);
                        if did_hit {
                            match &node.left {
                                Some(n) => new_to_check.push(&n),
                                None => eprint!("Error: reached AABB without Sphere child node")
                            }
                            match &node.right {
                                Some(n) => new_to_check.push(&n),
                                None => eprint!("Error: reached AABB without Sphere child node")
                            }
                        }
                    }
                    Object::Sphere(sphere) => {
                        let (did_hit, hit_rec) = sphere.hit(ray, new_interval);
                        if did_hit {
                            closest_yet = hit_rec.t;
                            hit_anything = true;
                            final_rec = hit_rec;
                        }
                    }
                };
            }
            to_check = new_to_check;
        }
        (hit_anything, final_rec)
    }

    fn bounding_box(&self) -> AABB {
        match self.bbox {
            Object::AABB(aabb) => aabb,
            Object::Sphere(sphere) => sphere.bbox
        }
    }
}

// TODO: write tests for this file!
// #[cfg(test)]
// mod tests {
//     use crate::{ray::Ray, util::{Point, Color}, sphere::Sphere, material::{LambertianMaterial, RenderableMaterial}};
//     use super::*;

//     #[test]
//     fn should_hit_aabb_of_sphere() {
//         let r = Ray::new(Point::new(0.0, 0.0, 0.0), Point::new(10.0, 0.0, 0.0));
//         let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
//         let bbox = Sphere::new(Point::new(0.0, 5.0, 0.0), 5.0, material ).bbox;
//         let (did_hit, _) = bbox.hit(&r, Interval{min: 0.0, max: 10.0});
//         assert!(did_hit)
//     }

//     #[test]
//     fn should_not_hit_aabb_of_sphere() {
//         let r = Ray::new(Point::new(0.0, 0.0, 0.0), Point::new(10.0, 0.0, 0.0));
//         let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
//         let bbox = Sphere::new(Point::new(5.0, 0.0, 0.0), 1.0, material ).bbox;
//         let (did_hit, _) = bbox.hit(&r, Interval{min: 0.0, max: 10.0});
//         assert!(!did_hit)
//     }

//     #[test]
//     fn creating_bvh_node_from_single_renderable_works() {
//         let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
//         let sphere = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0, material);
//         let renderables = vec![Object::Sphere(sphere)];

//         let root = BvhNode::new_from_renderables(&renderables);
//         let left_exists = match root.left {
//             None => false,
//             Some(_n) => true
//         };
//         let right_exists = match root.right {
//             None => false,
//             Some(_n) => true
//         };
//         assert!(left_exists && right_exists);
//     }

//     #[test]
//     fn creating_bvh_node_from_two_renderables_within_eachother_works() {
//         let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
//         // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
//         // a lower min for an axis = left
//         let sphere1 = Sphere::new(Point::new(0.0, 0.0, 0.0), 3.0, material);
//         let sphere2 = Sphere::new(Point::new(0.0, 0.5, 1.0), 1.0, material);
//         let renderables = vec![Object::Sphere(sphere1), Object::Sphere(sphere2)];

//         let root = BvhNode::new_from_renderables(&renderables);
//         let left_child_bbox = root.left.expect("root left child should be assigned").bounding_box();
//         let right_child_bbox = root.right.expect("root right child should be assigned").bounding_box();
//         let result = left_child_bbox == sphere1.bbox && right_child_bbox == sphere2.bbox;
//         assert!(result)
//     }

//     #[test]
//     fn creating_bvh_node_from_two_renderables_separated_works() {
//         let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
//         // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
//         // a lower min for an axis = left
//         let sphere1 = Sphere::new(Point::new(0.0, 0.0, 0.0), 3.0, material);
//         let sphere2 = Sphere::new(Point::new(1.0, 4.0, 4.0), 1.0, material);
//         let renderables = vec![Object::Sphere(sphere1), Object::Sphere(sphere2)];

//         let root = BvhNode::new_from_renderables(&renderables);
//         let left_child_bbox = root.left.expect("root left child should be assigned").bounding_box();
//         let right_child_bbox = root.right.expect("root right child should be assigned").bounding_box();
//         let result = left_child_bbox == sphere1.bbox && right_child_bbox == sphere2.bbox;
//         assert!(result)
//     }

//     #[test]
//     fn creating_bvh_node_from_multiple_renderables_separated_works_total_bbox() {
//         let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
//         // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
//         // a lower min for an axis = left
//         /*
//          * Scene structire: 
//          * y
//          * y ______
//          * y|      |      __
//          * y|   A  |     |B_| z = 
//          * y|______|     
//          * x/z -----------------------                         
//          * y            _____
//          * y           |  C  |
//          * y           |_____|
//          */  
//         let sphere_a = Sphere::new(Point::new(0.0, 1.0, 0.0), 3.0, material);
//         let sphere_b = Sphere::new(Point::new(3.0, 1.0, 3.0), 1.0, material);
//         let sphere_c = Sphere::new(Point::new(1.0, -2.0, -2.0), 0.5, material);
//         let renderables = vec![Object::Sphere(sphere_a), Object::Sphere(sphere_b), Object::Sphere(sphere_c)];

//         let root = BvhNode::new_from_renderables(&renderables);

//         // sphere bbox are calculated and assigned accordingly
//         // let child_left_bbox = root.left.expect("should be sphere_a").bounding_box();
//         // let child_right = root.right.expect("should be a BvhNode");
//         // let child_right_left_bbox = child_right.left.expect("should be sphere c").bounding_box();
//         // let child_right_right_bbox = child_right.right.expect("should be sphere b").bounding_box();

//         // total bbox is assigned accordingly
//         let expected_total_bbox = AABB::new_from_pts(Point::new(-3.0, -2.5, -3.0), Point::new(4.0, 4.0, 4.0));
//         let actual_total_bbox = root.bounding_box();

//         assert_eq!(expected_total_bbox, actual_total_bbox);
//     }

//     #[test]
//     fn creating_bvh_node_from_multiple_renderables_separated_works_child_bvh_node_bbox() {
//         let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
//         // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
//         // a lower min for an axis = left
//         /*
//          * Scene structire: 
//          * y
//          * y ______
//          * y|      |      __
//          * y|   A  |     |B_| z = 
//          * y|______|     
//          * x/z -----------------------                         
//          * y            _____
//          * y           |  C  |
//          * y           |_____|
//          */  
//         let sphere_a = Sphere::new(Point::new(0.0, 0.0, 0.0), 3.0, material);
//         let sphere_b = Sphere::new(Point::new(3.0, 1.0, 3.0), 1.0, material);
//         let sphere_c = Sphere::new(Point::new(1.0, -2.0, -2.0), 0.5, material);
//         let renderables = vec![Object::Sphere(sphere_a), Object::Sphere(sphere_b), Object::Sphere(sphere_c)];

//         let root = BvhNode::new_from_renderables(&renderables);

//         // sphere bbox are calculated and assigned accordingly
//         // let child_left_bbox = root.left.expect("should be sphere_a").bounding_box();
//         let child_right = root.right.expect("should be a BvhNode");
//         // let child_right_left_bbox = child_right.left.expect("should be sphere c").bounding_box();
//         // let child_right_right_bbox = child_right.right.expect("should be sphere b").bounding_box();

//         // total bbox is assigned accordingly
//         let expected_right_child_bbox = AABB::new_from_pts(Point::new(0.5, -2.5, -2.5), Point::new(4.0, 2.0, 4.0));
//         let actual_right_child_bbox = child_right.bounding_box();

//         assert_eq!(expected_right_child_bbox, actual_right_child_bbox);
//     }

//     #[test]
//     fn creating_bvh_node_from_multiple_renderables_separated_works_left_leaf_child_bbox() {
//         let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
//         // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
//         // a lower min for an axis = left
//         /*
//          * Scene structire: 
//          * y
//          * y ______
//          * y|      |      __
//          * y|   A  |     |B_| z = 
//          * y|______|     
//          * x/z -----------------------                         
//          * y            _____
//          * y           |  C  |
//          * y           |_____|
//          */  
//         let sphere_a = Sphere::new(Point::new(0.0, 0.0, 0.0), 3.0, material);
//         let sphere_b = Sphere::new(Point::new(3.0, 1.0, 3.0), 1.0, material);
//         let sphere_c = Sphere::new(Point::new(1.0, -2.0, -2.0), 0.5, material);
//         let renderables = vec![Object::Sphere(sphere_a), Object::Sphere(sphere_b), Object::Sphere(sphere_c)];

//         let root = BvhNode::new_from_renderables(&renderables);

//         // sphere bbox are calculated and assigned accordingly
//         let child_left_bbox = root.left.expect("should be sphere_a").bounding_box();
//         // let child_right = root.right.expect("should be a BvhNode");
//         // let child_right_left_bbox = child_right.left.expect("should be sphere c").bounding_box();
//         // let child_right_right_bbox = child_right.right.expect("should be sphere b").bounding_box();

//         // total bbox is assigned accordingly
//         let expected_left_child_bbox = AABB::new_from_pts(Point::new(0.0, 0.0, 0.0), Point::new(3.0, 3.0, 3.0));

//         assert_eq!(expected_left_child_bbox, child_left_bbox);
//     }

//     #[test]
//     fn creating_bvh_node_from_multiple_renderables_separated_works_subtree_left_child_bbox() {
//         let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
//         // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
//         // a lower min for an axis = left
//         /*
//          * Scene structire: 
//          * y
//          * y ______
//          * y|      |      __
//          * y|   A  |     |B_| z = 
//          * y|______|     
//          * x/z -----------------------                         
//          * y            _____
//          * y           |  C  |
//          * y           |_____|
//          */  
//         let sphere_a = Sphere::new(Point::new(0.0, 0.0, 0.0), 3.0, material);
//         let sphere_b = Sphere::new(Point::new(3.0, 1.0, 3.0), 1.0, material);
//         let sphere_c = Sphere::new(Point::new(1.0, -2.0, -2.0), 0.5, material);
//         let renderables = vec![Object::Sphere(sphere_a), Object::Sphere(sphere_b), Object::Sphere(sphere_c)];

//         let root = BvhNode::new_from_renderables(&renderables);

//         // sphere bbox are calculated and assigned accordingly
//         // let child_left_bbox = root.left.expect("should be sphere_a").bounding_box();
//         let child_right = root.right.expect("should be a BvhNode");
//         let child_right_left_bbox = child_right.left.expect("should be sphere c").bounding_box();
//         // let child_right_right_bbox = child_right.right.expect("should be sphere b").bounding_box();

//         // total bbox is assigned accordingly
//         let expected_left_child_bbox = sphere_c.bbox;

//         assert_eq!(expected_left_child_bbox, child_right_left_bbox);
//     }

//     #[test]
//     fn creating_bvh_node_from_multiple_renderables_separated_works_subtree_right_child_bbox() {
//         let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
//         // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
//         // a lower min for an axis = left
//         /*
//          * Scene structire: 
//          * y
//          * y ______
//          * y|      |      __
//          * y|   A  |     |B_| z = 
//          * y|______|     
//          * x/z -----------------------                         
//          * y            _____
//          * y           |  C  |
//          * y           |_____|
//          */  
//         let sphere_a = Sphere::new(Point::new(0.0, 0.0, 0.0), 3.0, material);
//         let sphere_b = Sphere::new(Point::new(3.0, 1.0, 3.0), 1.0, material);
//         let sphere_c = Sphere::new(Point::new(1.0, -2.0, -2.0), 0.5, material);
//         let renderables = vec![Object::Sphere(sphere_a), Object::Sphere(sphere_b), Object::Sphere(sphere_c)];

//         let root = BvhNode::new_from_renderables(&renderables);

//         // sphere bbox are calculated and assigned accordingly
//         // let child_left_bbox = root.left.expect("should be sphere_a").bounding_box();
//         let child_right = root.right.expect("should be a BvhNode");
//         // let child_right_left_bbox = child_right.left.expect("should be sphere c").bounding_box();
//         let child_right_right_bbox = child_right.right.expect("should be sphere b").bounding_box();

//         // total bbox is assigned accordingly
//         let expected_right_child_bbox = sphere_b.bbox;

//         assert_eq!(expected_right_child_bbox, child_right_right_bbox);
//     }

//     // TODO: write tests regarding hit detection with BvHNode World
//     #[test]
//     fn bvh_node_should_not_be_hit_when_ray_misses_completely_did_hit() {
//         let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
//         // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
//         // a lower min for an axis = left
//         /*
//          * Scene structire: 
//          * y
//          * y ______
//          * y|      |      __
//          * y|   A  |     |B_| z = 
//          * y|______|     
//          * x/z -----------------------                         
//          * y            _____
//          * y           |  C  |
//          * y           |_____|
//          */  
//         let sphere_a = Sphere::new(Point::new(0.0, 0.0, 0.0), 3.0, material);
//         let sphere_b = Sphere::new(Point::new(3.0, 1.0, 3.0), 1.0, material);
//         let sphere_c = Sphere::new(Point::new(1.0, -2.0, -2.0), 0.5, material);
//         let renderables = vec![Object::Sphere(sphere_a), Object::Sphere(sphere_b), Object::Sphere(sphere_c)];

//         let root = BvhNode::new_from_renderables(&renderables);

//         let r = Ray::new(Point::new(-10.0, 0.0, 0.0), Point::new(0.0, 0.0, 10.0));

//         let (did_hit, _actual_hit_record) = root.hit(&r, Interval{min: -10.0, max:10.0});

//         assert!(!did_hit);
//     }

//     #[test]
//     fn bvh_node_should_not_be_hit_when_ray_misses_completely_hit_record() {
//         let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
//         // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
//         // a lower min for an axis = left
//         /*
//          * Scene structire: 
//          * y
//          * y ______
//          * y|      |      __
//          * y|   A  |     |B_| z = 
//          * y|______|     
//          * x/z -----------------------                         
//          * y            _____
//          * y           |  C  |
//          * y           |_____|
//          */  
//         let sphere_a = Sphere::new(Point::new(0.0, 0.0, 0.0), 3.0, material);
//         let sphere_b = Sphere::new(Point::new(3.0, 1.0, 3.0), 1.0, material);
//         let sphere_c = Sphere::new(Point::new(1.0, -2.0, -2.0), 0.5, material);
//         let renderables = vec![Object::Sphere(sphere_a), Object::Sphere(sphere_b), Object::Sphere(sphere_c)];

//         let root = BvhNode::new_from_renderables(&renderables);

//         let r = Ray::new(Point::new(-10.0, 0.0, 0.0), Point::new(0.0, 0.0, 10.0));

//         let (_did_hit, actual_hit_record) = root.hit(&r, Interval{min: -10.0, max:10.0});

//         let expected_hit_record = HitRecord::nothing();

//         assert_eq!(expected_hit_record, actual_hit_record);
//     }

//     #[test]
//     fn bvh_node_should_be_hit_when_ray_shot_at_root_bvh_node_aabb() {
//         let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
//         // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
//         // a lower min for an axis = left
//         /*
//          * Scene structire: 
//          * y
//          * y ______
//          * y|      |      __
//          * y|   A  |     |B_| z = 
//          * y|______|     
//          * x/z -----------------------                         
//          * y            _____
//          * y           |  C  |
//          * y           |_____|
//          */  
//         let sphere_a = Sphere::new(Point::new(0.0, 0.0, 0.0), 3.0, material);
//         let sphere_b = Sphere::new(Point::new(3.0, 1.0, 3.0), 1.0, material);
//         let sphere_c = Sphere::new(Point::new(1.0, -2.0, -2.0), 0.5, material);
//         let renderables = vec![Object::Sphere(sphere_a), Object::Sphere(sphere_b), Object::Sphere(sphere_c)];

//         let root = BvhNode::new_from_renderables(&renderables);

//         let r = Ray::new(Point::new(2.0, 5.0, 0.0), Point::new(0.0, 0.0, 0.0));

//         let (did_hit, _actual_hit_record) = root.hit(&r, Interval{min: 0.0, max:10.0});

//         assert!(did_hit);
//     }

//     #[test]
//     fn bvh_node_should_hit_when_aabb_is_hit() {
//         let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
//         // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
//         // a lower min for an axis = left
//         /*
//          * Scene structire: 
//          * y
//          * y ______
//          * y|      |      __
//          * y|   A  |     |B_| z = 
//          * y|______|     
//          * x/z -----------------------                         
//          * y            _____
//          * y           |  C  |
//          * y           |_____|
//          */  
//         let sphere_a = Sphere::new(Point::new(0.0, 0.0, 0.0), 3.0, material);
//         let renderables = vec![Object::Sphere(sphere_a)];

//         let root = BvhNode::new_from_renderables(&renderables);

//         let r = Ray::new(Point::new(-2.0, 0.0, 0.0), Point::new(0.0, 0.0, 0.0));

//         let (did_hit, _actual_hit_record) = root.hit(&r, Interval{min: 0.0, max:10.0});

//         assert!(did_hit);
//     }

//     #[test]
//     fn bvh_node_should_not_be_hit_when_aabb_is_missed() {
//         let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
//         // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
//         // a lower min for an axis = left
//         /*
//          * Scene structire: 
//          * y
//          * y ______
//          * y|      |      __
//          * y|   A  |     |B_| z = 
//          * y|______|     
//          * x/z -----------------------                         
//          * y            _____
//          * y           |  C  |
//          * y           |_____|
//          */  
//         let sphere_a = Sphere::new(Point::new(0.0, 0.0, 0.0), 3.0, material);
//         let renderables = vec![Object::Sphere(sphere_a)];

//         let root = BvhNode::new_from_renderables(&renderables);

//         let r = Ray::new(Point::new(-5.0, 0.0, 0.0), Point::new(0.0, 10.0, 0.0));

//         let (did_hit, _actual_hit_record) = root.hit(&r, Interval{min: 0.0, max:10.0});

//         assert!(!did_hit);
//     }

//     #[test]
//     fn bvh_node_root_should_record_hit_when_child_bvh_node_hit() {
//         let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
//         // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
//         // a lower min for an axis = left
//         /*
//          * Scene structire: 
//          * y
//          * y ______
//          * y|      |      __
//          * y|   A  |     |B_| z = 
//          * y|______|     
//          * x/z -----------------------                         
//          * y            _____
//          * y           |  C  |
//          * y           |_____|
//          */  
//         let sphere_a = Sphere::new(Point::new(0.0, 3.0, 3.0), 3.0, material);
//         let sphere_b = Sphere::new(Point::new(0.0, -3.0, -3.0), 3.0, material);
//         let renderables = vec![Object::Sphere(sphere_a), Object::Sphere(sphere_b)];

//         let root = BvhNode::new_from_renderables(&renderables);

//         let r = Ray::new(Point::new(-5.0, 0.0, 0.0), Point::new(0.0, -3.0, -3.0));

//         let (did_hit, _actual_hit_record) = root.hit(&r, Interval{min: -10.0, max:00.0});

//         assert!(did_hit);
//     }

//     #[test]
//     fn bvh_node_should_be_hit_when_ray_shot_at_child_bvh_node_aabb_negative_axis() {
//         let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
//         // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
//         // a lower min for an axis = left
//         /*
//          * Scene structire: 
//          * y
//          * y ______
//          * y|      |      __
//          * y|   A  |     |B_| z = 
//          * y|______|     
//          * x/z -----------------------                         
//          * y            _____
//          * y           |  C  |
//          * y           |_____|
//          */  
//         let sphere_a = Sphere::new(Point::new(-5.0, -5.0, 0.0), 2.0, material);
//         let sphere_b = Sphere::new(Point::new(0.0, 0.0, 0.0), 2.0, material);
//         let sphere_c = Sphere::new(Point::new(5.0, 5.0, 0.0), 2.0, material);
//         let renderables = vec![Object::Sphere(sphere_a), Object::Sphere(sphere_b), Object::Sphere(sphere_c)];

//         let root = BvhNode::new_from_renderables(&renderables);

//         let r = Ray::new(Point::new(-5.0, -5.0, -5.0), Point::new(-5.0, -5.0, 0.0));

//         let (did_hit, _actual_hit_record) = root.hit(&r, Interval{min: 0.0, max:10.0});

//         assert!(did_hit);
//     }

//     #[test]
//     fn bvh_node_should_be_hit_when_ray_shot_at_child_bvh_node_aabb_neutral_axis() {
//         let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
//         // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
//         // a lower min for an axis = left
//         /*
//          * Scene structire: 
//          * y
//          * y ______
//          * y|      |      __
//          * y|   A  |     |B_| z = 
//          * y|______|     
//          * x/z -----------------------                         
//          * y            _____
//          * y           |  C  |
//          * y           |_____|
//          */  
//         let sphere_a = Sphere::new(Point::new(-5.0, -5.0, 0.0), 2.0, material);
//         let sphere_b = Sphere::new(Point::new(0.0, 0.0, 0.0), 2.0, material);
//         let sphere_c = Sphere::new(Point::new(5.0, 5.0, 0.0), 2.0, material);
//         let renderables = vec![Object::Sphere(sphere_a), Object::Sphere(sphere_b), Object::Sphere(sphere_c)];

//         let root = BvhNode::new_from_renderables(&renderables);

//         let r = Ray::new(Point::new(0.0, 0.0, -5.0), Point::new(0.0, 0.0, 0.0));

//         let (did_hit, _actual_hit_record) = root.hit(&r, Interval{min: 0.0, max:10.0});

//         assert!(did_hit);
//     }

//     #[test]
//     fn bvh_node_should_be_hit_when_ray_shot_at_child_bvh_node_aabb_positive_axis() {
//         let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
//         // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
//         // a lower min for an axis = left
//         /*
//          * Scene structire: 
//          * y
//          * y ______
//          * y|      |      __
//          * y|   A  |     |B_| z = 
//          * y|______|     
//          * x/z -----------------------                         
//          * y            _____
//          * y           |  C  |
//          * y           |_____|
//          */  
//         let sphere_a = Sphere::new(Point::new(-5.0, -5.0, 0.0), 2.0, material);
//         let sphere_b = Sphere::new(Point::new(0.0, 0.0, 0.0), 2.0, material);
//         let sphere_c = Sphere::new(Point::new(5.0, 5.0, 0.0), 2.0, material);
//         let renderables = vec![Object::Sphere(sphere_a), Object::Sphere(sphere_b), Object::Sphere(sphere_c)];

//         let root = BvhNode::new_from_renderables(&renderables);

//         let r = Ray::new(Point::new(5.0, 5.0, -5.0), Point::new(5.0, 5.0, 0.0));

//         let (did_hit, _actual_hit_record) = root.hit(&r, Interval{min: 0.0, max:10.0});

//         assert!(did_hit);
//     }

//     #[ignore]
//     #[test]
//     fn bvh_node_hit_point_should_be_on_correct_renderable_when_hit_positive_axis() {
//         let material: RenderableMaterial = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
//         // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
//         // a lower min for an axis = left
//         /*
//          * Scene structire: 
//          * y
//          * y ______
//          * y|      |      __
//          * y|   A  |     |B_| z = 
//          * y|______|     
//          * x/z -----------------------                         
//          * y            _____
//          * y           |  C  |
//          * y           |_____|
//          */  
//         let sphere_a = Sphere::new(Point::new(-5.0, -5.0, 0.0), 2.0, material);
//         let sphere_b = Sphere::new(Point::new(0.0, 0.0, 0.0), 2.0, material);
//         let sphere_c = Sphere::new(Point::new(5.0, 5.0, 0.0), 2.0, material);
//         let renderables = vec![Object::Sphere(sphere_a), Object::Sphere(sphere_b), Object::Sphere(sphere_c)];

//         let root = BvhNode::new_from_renderables(&renderables);

//         let r = Ray::new(Point::new(-5.0, -5.0, -5.0), Point::new(-5.0, -5.0, 0.0));

//         let (did_hit, actual_hit_record) = root.hit(&r, Interval{min: 0.0, max:10.0});

//         let actual_hit_point = actual_hit_record.point;
//         let expected_hit_point = Point::new(-5.0, -5.0, -3.0);

//         assert_eq!(actual_hit_point, expected_hit_point);
//     }

    // #[test]
    // fn bvh_node_should_be_hit_when_ray_shot_at_child_bvh_node_aabb_neutral_axis() {
    //     let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
    //     // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
    //     // a lower min for an axis = left
    //     /*
    //      * Scene structire: 
    //      * y
    //      * y ______
    //      * y|      |      __
    //      * y|   A  |     |B_| z = 
    //      * y|______|     
    //      * x/z -----------------------                         
    //      * y            _____
    //      * y           |  C  |
    //      * y           |_____|
    //      */  
    //     let sphere_a = Sphere::new(Point::new(-5.0, -5.0, 0.0), 2.0, material);
    //     let sphere_b = Sphere::new(Point::new(0.0, 0.0, 0.0), 2.0, material);
    //     let sphere_c = Sphere::new(Point::new(5.0, 5.0, 0.0), 2.0, material);
    //     let renderables = vec![Object::Sphere(sphere_a), Object::Sphere(sphere_b), Object::Sphere(sphere_c)];

    //     let root = BvhNode::new_from_renderables(&renderables);

    //     let r = Ray::new(Point::new(0.0, 0.0, -5.0), Point::new(0.0, 0.0, 0.0));

    //     let (did_hit, _actual_hit_record) = root.hit(&r, Interval{min: 0.0, max:10.0});

    //     assert!(did_hit);
    // }

    // #[test]
    // fn bvh_node_should_be_hit_when_ray_shot_at_child_bvh_node_aabb_positive_axis() {
    //     let material = RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0)));
    //     // because of how we randomly assign left vs. right children by random axis choice, sphere2 is contained completely in sphere 1 for testability
    //     // a lower min for an axis = left
    //     /*
    //      * Scene structire: 
    //      * y
    //      * y ______
    //      * y|      |      __
    //      * y|   A  |     |B_| z = 
    //      * y|______|     
    //      * x/z -----------------------                         
    //      * y            _____
    //      * y           |  C  |
    //      * y           |_____|
    //      */  
    //     let sphere_a = Sphere::new(Point::new(-5.0, -5.0, 0.0), 2.0, material);
    //     let sphere_b = Sphere::new(Point::new(0.0, 0.0, 0.0), 2.0, material);
    //     let sphere_c = Sphere::new(Point::new(5.0, 5.0, 0.0), 2.0, material);
    //     let renderables = vec![Object::Sphere(sphere_a), Object::Sphere(sphere_b), Object::Sphere(sphere_c)];

    //     let root = BvhNode::new_from_renderables(&renderables);

    //     let r = Ray::new(Point::new(5.0, 5.0, -5.0), Point::new(5.0, 5.0, 0.0));

    //     let (did_hit, _actual_hit_record) = root.hit(&r, Interval{min: 0.0, max:10.0});

    //     assert!(did_hit);
    // }

// }
