pub mod util;
pub mod ray;
pub mod renderable;
pub mod sphere;

use util::{Color, Point, Vec3};
use ray::Ray;
use renderable::{RenderableList, Renderable};
use std::f32::INFINITY;
use sphere::Sphere;

const FACTOR: f32 = 255.999;

fn write_color_to_output(color: Color) {
    println!("{} {} {}", (color.x() * FACTOR) as i32, (color.y() * FACTOR) as i32, (color.z() * FACTOR) as i32)
}

fn ray_color(ray: &Ray, world: &RenderableList) -> Color {
    let (did_hit, hit_rec) = world.hit(ray, 0.0, INFINITY);
    if did_hit {
        return (hit_rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_direction : Vec3 = ray.direction.unit_vector();
    let t : f32 = 0.5*(unit_direction.y() + 1.0);
    return Color::new(1.0, 1.0, 1.0)* (1.0-t) + Color::new(0.5, 0.7, 1.0) * t;
}

fn main() {

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f32)/ aspect_ratio) as i32;

    // WORLD

    let mut world: RenderableList = RenderableList::new();
    world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    // currently stretches this a ton lol
    // world.add(Box::new(Sphere::new(Point::new(-0.5, 0.0, -0.5), 0.15)));
    // ground
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    // actually render the image

    println!("P3\n{image_width} {image_height}\n255\n");

    for j in (0..image_height).rev() {
        // eprintln!("\rScanlines remaining: {j}");
        for i in 0..image_width {
            let u = (i as f32) / (image_width - 1) as f32;
            let v = (j as f32) / (image_height - 1) as f32;
            let r: Ray = Ray::new(origin, lower_left_corner + (horizontal * u) + (vertical * v) - origin);
            let pixel_color: Color = ray_color(&r, &world);
            write_color_to_output(pixel_color);
        }
    }
}
