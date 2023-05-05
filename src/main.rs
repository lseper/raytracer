pub mod util;
pub mod ray;
pub mod renderable;
pub mod sphere;
pub mod camera;
pub mod material;

use util::{Color, Point, Vec3, clamp, random_between_0_1};
use ray::Ray;
use renderable::{RenderableList, Renderable};
use std::{f32::INFINITY, rc::Rc};
use sphere::Sphere;
use camera::Camera;
use crate::material::{Metal, LambertianMaterial, Dielectric};
use std::f32::consts::PI;

const FACTOR: f32 = 255.999;
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
    return (Color::new(1.0, 1.0, 1.0)* (1.0-t)) + (Color::new(0.5, 0.7, 1.0) * t);
}

fn main() {

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f32)/ aspect_ratio) as i32;
    let samples_per_pixel = 100;

    // WORLD

    // MATERIAL DEFINITIONS
    // let ground_material = Rc::new(LambertianMaterial::new(Color::new(0.8, 0.8, 0.0)));
    // let front_material = Rc::new(Dielectric::new(Some(1.3)));
    // let left_material = Rc::new(Dielectric::new(Some(1.3)));
    // // let left_material = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), Some(0.3)));
    // let right_material = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), None ));

    // let mut world: RenderableList = RenderableList::new();
    // world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, front_material)));
    // world.add(Box::new(Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, right_material)));
    // world.add(Box::new(Sphere::new(Point::new(-1.0, 0.0, -1.0), -0.5, left_material)));
    // // ground
    // world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, ground_material)));

    // let R = f32::cos(PI / 4.0);
    let mut world : RenderableList = RenderableList::new();
    // let material_a = Rc::new(LambertianMaterial::new(Color::new(1.0, 0.0, 0.0)));
    // let material_b = Rc::new(LambertianMaterial::new(Color::new(0.0, 0.0, 1.0)));
    // world.add(Box::new(Sphere::new(Point::new(-R, 0.0, -1.0), R, material_a)));
    // world.add(Box::new(Sphere::new(Point::new(R, 0.0, -1.0), R, material_b)));

    // let cam = Camera::new(Point::new(0.0, 0.0, 1.0), Point::new(0.0, 0.15, -1.0), Vec3::new(0.0, 1.0, 0.0), 90.0, aspect_ratio);

    let material_ground = Rc::new(LambertianMaterial::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(LambertianMaterial::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(Some(1.5)));
    let material_left_2 = Rc::new(Dielectric::new(Some(1.5)));
    let material_right  = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), Some(0.0)));

    world.add(Box::new(Sphere::new(Point::new( 0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Point::new( 0.0,    0.0, -1.0),   0.5, material_center)));
    world.add(Box::new(Sphere::new(Point::new(-1.0,    0.0, -1.0),   0.5, material_left)));
    world.add(Box::new(Sphere::new(Point::new(-1.0,    0.0, -1.0), -0.45, material_left_2)));
    world.add(Box::new(Sphere::new(Point::new( 1.0,    0.0, -1.0),   0.5, material_right)));

    let cam = Camera::new(Point::new(-2.0,2.0,1.0), Point::new(0.0,0.0,-1.0), Vec3::new(0.0,1.0,0.0), 40.0, aspect_ratio);



    // actually render the image

    println!("P3\n{image_width} {image_height}\n255\n");

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {j}");
        for i in 0..image_width {
            // black
            let mut color = Color::zero();
            for _s in 0..samples_per_pixel {
                // random point for ray to shoot at within this pixel
                let u = ((i as f32) + random_between_0_1()) / (image_width as f32);
                let v = ((j as f32) + random_between_0_1()) / (image_height as f32);
                let r = cam.get_ray(u, v);

                color += ray_color(&r, &world, 0);
            }
            write_color_to_output(color, 100);
        }
    }
}
