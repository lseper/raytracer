use std::fmt;
use std::rc::Rc;

use crate::util::{Point, Vec3, Color};
use crate::ray::Ray;
use crate::material::{Material, LambertianMaterial};

pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f32,
    pub material_ptr: Rc<dyn Material>,

    pub front_face: bool
}

impl HitRecord {
    pub fn nothing() -> Self {
        Self { point: Point::zero(), normal: Vec3::zero(), t: 0.0, front_face: true, material_ptr: Rc::new(LambertianMaterial::new(Color::new(1.0, 1.0, 1.0)))}
    }

    pub fn new(point: Point, normal: Vec3, t: f32, front_face: bool, material_ptr: Rc<dyn Material>) -> Self {
        Self { point, normal, t, front_face, material_ptr}
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
        self.normal = if self.front_face {*outward_normal} else {-*outward_normal};
    }
}

pub trait Renderable: fmt::Display {
    fn hit (&self, ray: &Ray, t_min: f32, t_max: f32) -> (bool, HitRecord);
}

pub struct RenderableList {
    objects: Vec<Rc<dyn Renderable>>
}

impl RenderableList {
    pub fn new() -> Self {
        Self { objects: vec![]}
    }

    pub fn add(&mut self, to_render: Rc<dyn Renderable>) {
        self.objects.push(to_render);
    }
}

impl Renderable for RenderableList {
    fn hit (&self, ray: &Ray, t_min: f32, t_max: f32) -> (bool, HitRecord) {
        let mut final_rec: HitRecord = HitRecord::nothing();
        let mut hit_anything: bool = false;
        let mut closest_yet = t_max;

        for object in &(self.objects) {
            let (object_did_hit, hit_record) = object.hit(ray, t_min, closest_yet);
            if object_did_hit {
                hit_anything = true;
                closest_yet = hit_record.t;
                final_rec = hit_record;
            }
        }

        (hit_anything, final_rec)
    }
}

impl fmt::Display for RenderableList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let objects_str = self.objects.iter()
        .map(|renderable| renderable.to_string())
        .collect::<Vec<String>>()
        .join("\n");
        write!(f, "world\n{}", objects_str)
    }
}