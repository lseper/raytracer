use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use std::fs;

// use std::io::Write;
// use std::rc::Rc;

use crate::camera::Camera;
use crate::material::{Dielectric, LambertianMaterial, Metal, RenderableMaterial};
use crate::renderable::{Object, RenderableList};
use crate::sphere::Sphere;
use crate::util::{random_between_0_1, random_in_range, Color, Point, Vec3};

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
    let DEFAULT_LOOK_FROM: Point = Point::new(13.0, 2.0, 3.0);
    let DEFAULT_LOOK_AT: Point = Point::new(0.0, 0.0, -1.0);
    let DEFAULT_FOCAL_LENGTH: f32 = 10.0; // (look_from - look_at).len() for focusing at the point we're aiming for

    let DEFAULT_APERATURE: f32 = 0.1;

    let DEFAULT_CAMERA: Camera = Camera::new(
        DEFAULT_LOOK_FROM,
        DEFAULT_LOOK_AT,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        1.0,
        DEFAULT_APERATURE,
        DEFAULT_FOCAL_LENGTH,
    );

    let DEFAULT_MATERIAL: RenderableMaterial =
        RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.4, 0.2, 0.1)));
    let DEFAULT_SPHERE: Sphere = Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, DEFAULT_MATERIAL);
    let DEFAULT_WORLD: RenderableList = RenderableList {
        objects: vec![Object::Sphere(DEFAULT_SPHERE)],
    };

    let DEFAULT_SCENE: Scene = Scene {
        aspect_ratio: 1.0,
        image_width: 400,
        image_height: 400,
        samples_per_pixel: 100,
        camera: DEFAULT_CAMERA,
        world: DEFAULT_WORLD,
    };
    DEFAULT_SCENE
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

pub fn random_scene() -> RenderableList {
    let mut world: RenderableList = RenderableList::new();

    let material_ground =
        RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.5, 0.5, 0.5)));
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
                    let albedo: Color = Color::random(0.0, 1.0);
                    let sphere_material =
                        RenderableMaterial::Lambertian(LambertianMaterial::new(albedo));
                    world.add(Object::Sphere(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0);
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
        RenderableMaterial::Lambertian(LambertianMaterial::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Object::Sphere(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = RenderableMaterial::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), Some(0.0)));
    world.add(Object::Sphere(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    world
}
