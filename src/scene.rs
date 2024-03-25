use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use std::fs;
use std::env;

// use std::io::Write;
// use std::rc::Rc;

use crate::aabb::{AABB, BvhNode};
use crate::camera::Camera;
use crate::material::{Dielectric, LambertianMaterial, Metal, RenderableMaterial};
use crate::renderable::{Object, RenderableList, Renderable};
use crate::sphere::Sphere;
use crate::util::{random_between_0_1, random_in_range, Color, Point, Vec3};
use crate::texture::{CheckerTexture, SolidColor, RenderableTexture};

pub struct SceneMetaData {
    pub file_name: String,
    pub aspect_ratio: f32,
    pub image_width: i32,
    pub image_height: i32,
    pub samples_per_pixel: i32,
}

pub fn save_scene(scene_metadata: SceneMetaData, camera: Camera, world: RenderableList) {
    let scene = Scene {
        aspect_ratio: scene_metadata.aspect_ratio,
        image_width: scene_metadata.image_width,
        image_height: scene_metadata.image_height,
        samples_per_pixel: scene_metadata.samples_per_pixel,
        camera,
        world,
    };
    let serialized = serde_json::to_string(&scene).unwrap();

    match env::current_dir() {
        Ok(path) => {
            eprintln_f!("The current directory is {}", path.display());
        }
        Err(e) => {
            eprintln_f!("Couldn't get the current directory: {}", e);
        }
    }

    eprintln_f!("{}", scene_metadata.file_name);
    fs::write(scene_metadata.file_name, serialized).expect("Unable to write to file?");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
    pub aspect_ratio: f32,
    pub image_width: i32,
    pub image_height: i32,
    pub samples_per_pixel: i32,
    pub camera: Camera,
    pub world: RenderableList,
}

pub fn default_scene() -> Scene {
    let default_look_from: Point = Point::new(13.0, 2.0, 3.0);
    let default_look_at: Point = Point::new(0.0, 0.0, -1.0);
    let default_focal_length: f32 = 10.0; // (look_from - look_at).len() for focusing at the point we're aiming for

    let default_aperature: f32 = 0.1;

    let default_camera: Camera = Camera::new(
        default_look_from,
        default_look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        1.0,
        default_aperature,
        default_focal_length,
    );

    let default_material: RenderableMaterial =
        RenderableMaterial::Lambertian(LambertianMaterial::new(RenderableTexture::SolidColor(SolidColor::from_values(0.4, 0.2, 0.1))));
    let default_sphere: Sphere = Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, default_material);
    let default_obj = Object::Sphere(default_sphere);
    let default_world: RenderableList = RenderableList {
        objects: vec![default_obj],
        bbox: AABB::new_from_bbox(AABB::empty(), default_obj.bounding_box())
    };

    let default_scene: Scene = Scene {
        aspect_ratio: 1.0,
        image_width: 400,
        image_height: 400,
        samples_per_pixel: 100,
        camera: default_camera,
        world: default_world,
    };
    default_scene
}

pub fn load_scene<P: AsRef<Path>>(path: P) -> Scene {
    // Open the file in read-only mode with buffer.
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let s = serde_json::from_reader(reader).unwrap_or(default_scene());

    // Return the `Scene`.
    s
}

pub fn test_scene() -> RenderableList {
    let mut world: RenderableList = RenderableList::new();

    let material_ground =
        RenderableMaterial::Lambertian(LambertianMaterial::new(RenderableTexture::SolidColor(SolidColor::from_values(0.5, 0.5, 0.5))));
    let ground = Object::Sphere(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    ));
    world.add(ground);

    let material_1 = RenderableMaterial::Dielectric(Dielectric::new(Some(1.5)));
    world.add(Object::Sphere(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 =
        RenderableMaterial::Lambertian(LambertianMaterial::new(RenderableTexture::SolidColor(SolidColor::from_values(0.4, 0.2, 0.1))));
    world.add(Object::Sphere(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = RenderableMaterial::Metal(Metal::new(RenderableTexture::SolidColor(SolidColor::from_values(0.7, 0.6, 0.5)), Some(0.0)));
    world.add(Object::Sphere(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    world
}

pub fn random_scene() -> RenderableList {
    let mut world: RenderableList = RenderableList::new();

    let material_ground =
        RenderableMaterial::Lambertian(LambertianMaterial::new(RenderableTexture::SolidColor(SolidColor::from_values(0.5, 0.5, 0.5))));
    let ground = Object::Sphere(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    ));
    world.add(ground);

    // generate the spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random_between_0_1();
            let center = Point::new(
                a as f32 + 0.9 * random_between_0_1(),
                0.2,
                b as f32 + 0.9 * random_between_0_1(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_material < 0.8 {
                    // diffuse
                    let albedo: RenderableTexture = RenderableTexture::SolidColor(SolidColor::from_color(Color::random(0.0, 1.0)));
                    let sphere_material =
                        RenderableMaterial::Lambertian(LambertianMaterial::new(albedo));
                    let center2 = center + Vec3::new(0.0, random_in_range(0.0, 0.5), 0.0);
                    world.add(Object::Sphere(Sphere::new_moving(center, 0.2, sphere_material, center2)));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = RenderableTexture::SolidColor(SolidColor::from_color(Color::random(0.5, 1.0)));
                    let fuzz = random_in_range(0.0, 0.5);
                    let sphere_material = RenderableMaterial::Metal(Metal::new(albedo, Some(fuzz)));
                    world.add(Object::Sphere(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material =
                        RenderableMaterial::Dielectric(Dielectric::new(Some(1.5)));
                    world.add(Object::Sphere(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material_1 = RenderableMaterial::Dielectric(Dielectric::new(Some(1.5)));
    world.add(Object::Sphere(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 =
        RenderableMaterial::Lambertian(LambertianMaterial::new(RenderableTexture::SolidColor(SolidColor::from_values(0.4, 0.2, 0.1))));
    world.add(Object::Sphere(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = RenderableMaterial::Metal(Metal::new(RenderableTexture::SolidColor(SolidColor::from_values(0.7, 0.6, 0.5)), Some(0.0)));
    world.add(Object::Sphere(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));
    world
}

pub fn random_scene_checker() -> RenderableList {
    let mut world: RenderableList = RenderableList::new();

    let material_ground = RenderableMaterial::Lambertian(LambertianMaterial::new(RenderableTexture::CheckerTexture(CheckerTexture::new_from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)))));
    let ground = Object::Sphere(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    ));
    world.add(ground);

    // generate the spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random_between_0_1();
            let center = Point::new(
                a as f32 + 0.9 * random_between_0_1(),
                0.2,
                b as f32 + 0.9 * random_between_0_1(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_material < 0.8 {
                    // diffuse
                    let albedo: RenderableTexture = RenderableTexture::SolidColor(SolidColor::from_color(Color::random(0.0, 1.0)));
                    let sphere_material =
                        RenderableMaterial::Lambertian(LambertianMaterial::new(albedo));
                    let center2 = center + Vec3::new(0.0, random_in_range(0.0, 0.5), 0.0);
                    world.add(Object::Sphere(Sphere::new_moving(center, 0.2, sphere_material, center2)));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo: RenderableTexture = RenderableTexture::SolidColor(SolidColor::from_color(Color::random(0.5, 1.0)));
                    let fuzz = random_in_range(0.0, 0.5);
                    let sphere_material = RenderableMaterial::Metal(Metal::new(albedo, Some(fuzz)));
                    world.add(Object::Sphere(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material =
                        RenderableMaterial::Dielectric(Dielectric::new(Some(1.5)));
                    world.add(Object::Sphere(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material_1 = RenderableMaterial::Dielectric(Dielectric::new(Some(1.5)));
    world.add(Object::Sphere(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 =
        RenderableMaterial::Lambertian(LambertianMaterial::new(RenderableTexture::SolidColor(SolidColor::from_values(0.4, 0.2, 0.1))));
    world.add(Object::Sphere(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = RenderableMaterial::Metal(Metal::new(RenderableTexture::SolidColor(SolidColor::from_values(0.7, 0.6, 0.5)), Some(0.0)));
    world.add(Object::Sphere(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    world
}
