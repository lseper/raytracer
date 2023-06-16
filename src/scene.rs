use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use std::io::Write;
use std::rc::Rc;

use crate::camera::Camera;
use crate::material::{LambertianMaterial, Dielectric, Metal};
use crate::renderable::{RenderableList};
use crate::sphere::Sphere;
use crate::util::{random_between_0_1, random_in_range, Color, Point, Vec3};

pub struct SceneMetaData {
    pub file_name: String,
    pub aspect_ratio: f32,
    pub image_width: i32,
    pub image_height: i32,
    pub samples_per_pixel: i32,
}

impl SceneMetaData {
    pub fn new(file_name: String, aspect_ratio: f32, image_width: i32, image_height: i32, samples_per_pixel: i32) -> Self {
        Self { file_name, aspect_ratio, image_width, image_height, samples_per_pixel }
    }
}

pub fn save_scene(scene_metadata: SceneMetaData, camera: Camera, world: &RenderableList) {
    let mut file = File::create(scene_metadata.file_name).expect("File should be able to be created");
    // file structure: my_scene.scene
    // 
    writeln!(file, "{}", scene_metadata.aspect_ratio).expect("Error writing aspect ratio");
    writeln!(file, "{}", scene_metadata.image_width).expect("Error writing image width");
    writeln!(file, "{}", scene_metadata.image_height).expect("Error writing aspect ratio");
    writeln!(file, "{}", scene_metadata.samples_per_pixel).expect("Error writing aspect ratio");

    writeln!(file, "{}", camera).expect("Error writing camera");

    writeln!(file, "{}", world).expect("Error writing world");
}

#[derive(Deserialize, Debug)]
struct SerializedMaterial {
    material_type: String
}

#[derive(Deserialize, Debug)]
struct SerializedSphere {
    center: Vec3,
    r: f32,
    material: SerializedMaterial
}

#[derive(Deserialize, Debug)]
struct Scene {
    aspect_ratio: f32,
    image_width: i32,
    image_height: i32,
    samples_per_pixel: i32,
    camera: Camera,
    world: RenderableList,
}

fn load_scene<P: AsRef<Path>>(path: P) -> Result<User, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

pub fn random_scene() -> RenderableList {
    let mut world : RenderableList = RenderableList::new();

    let material_ground = Rc::new(LambertianMaterial::new(Color::new(0.5, 0.5, 0.5)));
    let ground = Rc::new(Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, material_ground));
    world.add(ground);

    // generate the spheres
    for a in 0..11 {
        for b in 0..11 {
            let choose_material = random_between_0_1();
            let center = Point::new(a as f32 + 0.9 * random_between_0_1(), 0.2, b as f32 + 0.9*random_between_0_1());

            if (center - Point::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_material < 0.8 {
                    // diffuse
                    let albedo: Color = Color::random(0.0, 1.0);
                    let sphere_material = Rc::new(LambertianMaterial::new(albedo));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = random_in_range(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, Some(fuzz)));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Rc::new(Dielectric::new(Some(1.5)));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material_1 = Rc::new(Dielectric::new(Some(1.5)));
    world.add(Rc::new(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, material_1)));
    
    let material_2 = Rc::new(LambertianMaterial::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, material_2)));
    
    let material_3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), Some(0.0)));
    world.add(Rc::new(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, material_3)));

    world
}