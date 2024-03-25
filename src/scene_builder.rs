use crate::scene::{random_scene, random_scene_checker, save_scene, SceneMetaData};
use crate::camera::Camera;
use crate::util::{Point, Vec3};

pub fn create_checker_test(destination_file_name: &str) {
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 300;
    const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 20;
    // WORLD

    let world = random_scene_checker();

    let look_from = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let focus_length = 10.0; // (look_from - look_at).len() for focusing at the point we're aiming for

    let aperature = 0.1;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        ASPECT_RATIO,
        aperature,
        focus_length,
    );

    let scene_metadata = SceneMetaData {
        file_name: String::from(format!("./scenes/{}", destination_file_name)),
        aspect_ratio: ASPECT_RATIO,
        image_width: IMAGE_WIDTH,
        image_height: IMAGE_HEIGHT,
        samples_per_pixel: SAMPLES_PER_PIXEL,
    };

    save_scene(scene_metadata, cam, world);
}

pub fn create_test(destination_file_name: &str) {
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 300;
    const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 20;
    // WORLD

    let world = random_scene();

    let look_from = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let focus_length = 10.0; // (look_from - look_at).len() for focusing at the point we're aiming for

    let aperature = 0.1;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        ASPECT_RATIO,
        aperature,
        focus_length,
    );

    let scene_metadata = SceneMetaData {
        file_name: String::from(format!("./scenes/{}", destination_file_name)),
        aspect_ratio: ASPECT_RATIO,
        image_width: IMAGE_WIDTH,
        image_height: IMAGE_HEIGHT,
        samples_per_pixel: SAMPLES_PER_PIXEL,
    };

    save_scene(scene_metadata, cam, world);
}

// TODO: add support for BVH
pub fn create(destination_file_name: &str) {
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 300;
    const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 20;
    // WORLD

    let world = random_scene();

    let look_from = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let focus_length = 10.0; // (look_from - look_at).len() for focusing at the point we're aiming for

    let aperature = 0.1;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        ASPECT_RATIO,
        aperature,
        focus_length,
    );

    let scene_metadata = SceneMetaData {
        file_name: String::from(format!("./scenes/{}", destination_file_name)),
        aspect_ratio: ASPECT_RATIO,
        image_width: IMAGE_WIDTH,
        image_height: IMAGE_HEIGHT,
        samples_per_pixel: SAMPLES_PER_PIXEL,
    };

    save_scene(scene_metadata, cam, world);
}
