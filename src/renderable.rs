use std::fmt;
// use std::rc::Rc;

use crate::aabb::AABB;
use crate::material::{LambertianMaterial, RenderableMaterial};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::util::{Color, Point, Vec3, Interval};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f32,
    pub material_ptr: RenderableMaterial,

    pub front_face: bool,
}

impl HitRecord {
    pub fn nothing() -> Self {
        Self {
            point: Point::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: true,
            material_ptr: RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(
                1.0, 1.0, 1.0,
            ))),
        }
    }

    pub fn new(
        point: Point,
        normal: Vec3,
        t: f32,
        front_face: bool,
        material_ptr: RenderableMaterial,
    ) -> Self {
        Self {
            point,
            normal,
            t,
            front_face,
            material_ptr,
        }
    }

    pub fn get_front_face(&self, ray: &Ray, outward_normal: &Vec3) -> bool {
        ray.direction.dot(*outward_normal) < 0.0
    }
    // need to do this in order to distinguish hitting the outside of a volume vs the inside of a volume
    // as they always point out, we need to know what side the ray was on when it actually intersected
    // i.e are we hitting the front or back of this surface?
    // as we're choosing to have the normals always point OUT of the surface, we can determine if the ray is hitting
    // the inside or outside via the result of dotting it with the normal vector. If it's positive, that means
    // the normal vector and ray are pointing in the same direction (and so, the ray must have hit it from the INSIDE)
    // if dotting it wit hthe normal is negative, then the ray must have hit it from the OUTSIDE
    // (if it hits it tangentially, then do either, it doesn't really matter)
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(*outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

impl PartialEq for HitRecord {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point && self.normal == other.normal && self.t == other.t && self.front_face == other.front_face
    }
}

impl Eq for HitRecord {}

impl Copy for HitRecord {}

impl Clone for HitRecord {
    fn clone(&self) -> Self {
        HitRecord { point: self.point, normal: self.normal, t: self.t, material_ptr: self.material_ptr, front_face: self.front_face }
    }
}

pub trait Renderable {
    fn hit(&self, ray: &Ray, interval: Interval) -> (bool, HitRecord);
    fn bounding_box(&self) -> AABB {
        AABB::empty()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "type")] // will expect { type: "Sphere", ... } in JSON format
pub enum Object {
    Sphere(Sphere),
    AABB(AABB),
}

impl Renderable for Object {
    fn hit(&self, ray: &Ray, interval: Interval) -> (bool, HitRecord) {
        match self {
            Object::Sphere(s) => s.hit(ray, interval),
            Object::AABB(aabb) => aabb.hit(ray, interval)
        }
    }

    fn bounding_box(&self) -> AABB {
        match self {
            Object::Sphere(s) => s.bounding_box(),
            Object::AABB(aabb) => *aabb
        }
    }
}

impl Object {
    pub fn empty() -> Object {
        Object::AABB(AABB::empty())
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::AABB(aabb_1), Object::AABB(aabb_2)) => aabb_1 == aabb_2,
            (Object::Sphere(sphere_1), Object::Sphere(sphere_2)) => sphere_1 == sphere_2,
            _ => false
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderableList {
    pub objects: Vec<Object>,
    pub bbox: AABB
}

impl RenderableList {
    pub fn new() -> Self {
        Self { objects: vec![], bbox: AABB::empty()}
    }

    pub fn add(&mut self, to_render: Object) {
        self.objects.push(to_render);
        self.bbox = AABB::new_from_bbox(self.bbox, to_render.bounding_box())
    }
}

impl Renderable for RenderableList {
    fn hit(&self, ray: &Ray, interval: Interval) -> (bool, HitRecord) {
        let mut final_rec: HitRecord = HitRecord::nothing();
        let mut hit_anything: bool = false;
        let mut closest_yet = interval.max;

        for object in &(self.objects) {
            let new_interval = Interval {min: interval.min, max: closest_yet};
            let (object_did_hit, hit_record) = object.hit(ray, new_interval);
            if object_did_hit {
                hit_anything = true;
                closest_yet = hit_record.t;
                final_rec = hit_record;
            }
        }

        (hit_anything, final_rec)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

// impl fmt::Display for RenderableList {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let objects_str = self.objects.iter()
//         .map(|renderable| renderable.to_string())
//         .collect::<Vec<String>>()
//         .join("\n");
//         write!(f, "WORLD\n{}", objects_str)
//     }
// }
