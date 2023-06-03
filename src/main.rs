pub mod util;
pub mod ray;
pub mod renderable;
pub mod sphere;
pub mod camera;
pub mod material;
pub mod scene;

use scene::SceneMetaData;
use util::{Color, Point, Vec3, clamp};
use ray::Ray;
use renderable::{RenderableList, Renderable};
use std::{f32::INFINITY};
use sphere::Sphere;
use camera::{Camera};
use crate::scene::{save_scene, random_scene};


const COLOR_LIM: i32 = 256;
const BOUNCE_DEPTH: i32 = 50;

fn write_color_to_output(color: Color, samples_per_pixel: i32) {
    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    let scale = 1.0 / (samples_per_pixel as f32);

    // approximation of gamma correction, as image viewers for some reason scale by ^2
    r = f32::sqrt(scale * r);
    g = f32::sqrt(scale * g);
    b = f32::sqrt(scale * b);

    let r_out = ((COLOR_LIM as f32) * clamp(r, 0.0, 0.999)) as i32;
    let g_out = ((COLOR_LIM as f32) * clamp(g, 0.0, 0.999)) as i32;
    let b_out = ((COLOR_LIM as f32) * clamp(b, 0.0, 0.999)) as i32;
    println!("{} {} {}", r_out, g_out, b_out);
}

fn ray_color(ray: &Ray, world: &RenderableList, call_depth: i32) -> Color {
    if call_depth >= BOUNCE_DEPTH {
        return Color::zero();
    }
    let (did_hit, hit_rec) = world.hit(ray, 0.001, INFINITY);
    if did_hit {
        // if we hit something, determing how this ray scatters (if at all)
        let (did_scatter, scatter_color, scatter_ray) = hit_rec.material_ptr.scatter(ray, &hit_rec);
        if did_scatter {
            // if we do scatter, 
            return scatter_color * ray_color(&scatter_ray, world, call_depth + 1);
        }
    }
    let unit_direction : Vec3 = ray.direction.unit_vector();
    let t : f32 = 0.5*(unit_direction.y() + 1.0);
    let skybox = (Color::new(1.0, 1.0, 1.0)* (1.0-t)) + (Color::new(0.5, 0.7, 1.0) * t);
    skybox
}

fn render() {
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = ((image_width as f32)/ aspect_ratio) as i32;
    let samples_per_pixel = 500;

    // WORLD

    let world = random_scene();

    let look_from = Point::new(13.0,2.0,3.0);
    let look_at = Point::new(0.0,0.0,-1.0);
    let focus_length = 10.0; // (look_from - look_at).len() for focusing at the point we're aiming for

    let aperature = 0.1;

    let cam = Camera::new(look_from, look_at, Vec3::new(0.0,1.0,0.0), 20.0, aspect_ratio, aperature, focus_length);


    // actually render the image

    let scene_metadata = SceneMetaData::new(String::from("./scenes/test.scene"), aspect_ratio, image_width, image_height, samples_per_pixel);

    save_scene(scene_metadata, cam, &world);

    // println!("P3\n{image_width} {image_height}\n255\n");

    // for j in (0..image_height).rev() {
    //     eprintln!("\rScanlines remaining: {j}");
    //     for i in 0..image_width {
    //         // black
    //         let mut color = Color::zero();
    //         for _s in 0..samples_per_pixel {
    //             // random point for ray to shoot at within this pixel
    //             let u = ((i as f32) + random_between_0_1()) / (image_width as f32);
    //             let v = ((j as f32) + random_between_0_1()) / (image_height as f32);
    //             let r = cam.get_ray(u, v);
    //             color += ray_color(&r, &world, 0);
    //         }
    //         write_color_to_output(color, samples_per_pixel);
    //     }
    // }
}

fn main() {
    render();
}
