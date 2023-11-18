use std::{cmp::Ordering};

use serde::{Deserialize, Serialize};

use crate::{util::{Interval, Point, random_int}, renderable::{Renderable, RenderableList, Object}};
use crate::ray::Ray;
use crate::renderable::HitRecord;


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
        let mut root = Self::root();
        root.new_from_renderables_with_index(list, 0, list.len());
        root
    }

    fn new_leaf(object: Object) -> Option<Box<BvhNode>> {
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
            self.left = Self::new_leaf(list[start]);
            self.right = Self::new_leaf(list[start]);
        }
        // two, assign accordingly
        else if object_span == 2 {
            // if the first is < the second, build tree as:
            //              root
            //              /  \
            //         first    second
            if Ordering::is_le(comparator(&list[start], &list[start + 1])) {
                self.left = Self::new_leaf(list[start]);
                self.right = Self::new_leaf(list[start + 1]);
            } else {
                self.right = Self::new_leaf(list[start]);
                self.left = Self::new_leaf(list[start + 1]);
            }
        } else {
            let mut rec_list = list[start..end].to_vec();
            rec_list.sort_by(|a, b| comparator(a, b));
            let mid = start + (object_span / 2);
            self.new_from_renderables_with_index(&rec_list, start, mid);
            self.new_from_renderables_with_index(&rec_list, mid, end);
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