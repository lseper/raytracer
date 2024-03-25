pub mod camera;
pub mod material;
pub mod ray;
pub mod renderable;
pub mod scene;
pub mod sphere;
pub mod util;
pub mod aabb;
pub mod scene_builder;
pub mod texture;

#[macro_use]
extern crate fstrings;

use std::env;

use crate::{
    aabb::AABB, scene::{default_scene, load_scene, random_scene, save_scene, test_scene, Scene, SceneMetaData}, scene_builder::{create, create_checker_test, create_test}, util::random_between_0_1
};
use aabb::BvhNode;
use camera::Camera;
use material::Material;
use ray::Ray;
use renderable::{Renderable, RenderableList};
use texture::Texture;
use std::{f32::INFINITY};
use util::{clamp, Color, Point, Vec3, Interval};

use std::sync::mpsc;
use std::thread;
use std::time::{Instant};

const COLOR_LIM: i32 = 256;
const BOUNCE_DEPTH: i32 = 50;

struct RenderOptions {
    use_bvh_node: bool
}

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

fn ray_color_world(ray: &Ray, world: &RenderableList, call_depth: i32) -> Color {
    if call_depth >= BOUNCE_DEPTH {
        return Color::zero();
    }
    let (did_hit, hit_rec) = world.hit(ray, Interval{min: 0.001, max: INFINITY});
    if did_hit {
        // if we hit something, determing how this ray scatters (if at all)
        let (did_scatter, scatter_color, scatter_ray) = hit_rec.material_ptr.scatter(ray, &hit_rec);
        if did_scatter {
            // if we do scatter,
            return scatter_color.value(0.0, 0.0, &hit_rec.point) * ray_color_world(&scatter_ray, world, call_depth + 1);
        }
    }
    let unit_direction: Vec3 = ray.direction.unit_vector();
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    let skybox = (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t);
    skybox
}

fn ray_color_bvh_node(ray: &Ray, root: &BvhNode, call_depth: i32) -> Color {
    if call_depth >= BOUNCE_DEPTH {
        return Color::zero();
    }
    let (did_hit, hit_rec) = root.hit(ray, Interval{min: 0.001, max: INFINITY});
    if did_hit {
        // if we hit something, determing how this ray scatters (if at all)
        let (did_scatter, scatter_color, scatter_ray) = hit_rec.material_ptr.scatter(ray, &hit_rec);
        if did_scatter {
            // if we do scatter,
            return scatter_color.value(0.0, 0.0, &hit_rec.point) * ray_color_bvh_node(&scatter_ray, root, call_depth + 1);
        }
    }
    let unit_direction: Vec3 = ray.direction.unit_vector();
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    let skybox = (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t);
    skybox
}

fn render(scene_path: &str) {
    let scene: Scene = load_scene(scene_path);

    // progress printing
    let mut scanlines_remaining = scene.image_height;

    // initialize as all black
    println_f!("P3\n{scene.image_width} {scene.image_height}\n255\n");

    let mut pixels =
        vec![vec![Color::zero(); scene.image_width as usize]; scene.image_height as usize];

    for j in 0..scene.image_height {
        for i in 0..scene.image_width {
            // spawn threads
            for _s in 0..scene.samples_per_pixel {
                // random point for ray to shoot at within this pixel
                let u = ((i as f32) + random_between_0_1()) / (scene.image_width as f32);
                let v = ((j as f32) + random_between_0_1()) / (scene.image_height as f32);
                let r = scene.camera.get_ray(u, v);
                // starting call depth is 0 as increases until hitting bounce depth
                pixels[j as usize][i as usize] += ray_color_world(&r, &scene.world, 0);
            }
        }
        scanlines_remaining -= 1;
        eprintln_f!("Scanlines remaining: {scanlines_remaining}");
    }

    // write final colors to output
    for y in (0..pixels.len()).rev() {
        for x in 0..pixels[y].len() {
            write_color_to_output(pixels[y][x], scene.samples_per_pixel);
        }
    }
}

fn render_bvh(scene_path: &str) {
    let scene: Scene = load_scene(scene_path);

    // progress printing
    let mut scanlines_remaining = scene.image_height;

    // initialize as all black
    println_f!("P3\n{scene.image_width} {scene.image_height}\n255\n");

    let mut pixels =
        vec![vec![Color::zero(); scene.image_width as usize]; scene.image_height as usize];
    let root = BvhNode::new_from_renderables(&scene.world.objects);
    for j in 0..scene.image_height {
        for i in 0..scene.image_width {
            // spawn threads
            for _s in 0..scene.samples_per_pixel {
                // random point for ray to shoot at within this pixel
                let u = ((i as f32) + random_between_0_1()) / (scene.image_width as f32);
                let v = ((j as f32) + random_between_0_1()) / (scene.image_height as f32);
                let r = scene.camera.get_ray(u, v);
                // starting call depth is 0 as increases until hitting bounce depth
                pixels[j as usize][i as usize] += ray_color_bvh_node(&r, &root, 0);
            }
        }
        scanlines_remaining -= 1;
        eprintln_f!("Scanlines remaining: {scanlines_remaining}");
    }

    // write final colors to output
    for y in (0..pixels.len()).rev() {
        for x in 0..pixels[y].len() {
            write_color_to_output(pixels[y][x], scene.samples_per_pixel);
        }
    }
}

fn render_multi_threaded(scene_path: &str, mut num_threads: i32) {
    let scene: Scene = load_scene(scene_path);

    let samples_per_pixel_per_thread =
        f32::round(scene.samples_per_pixel as f32 / num_threads as f32) as i32;

    let mut handles = vec![];
    // let num_pixels: usize= (scene.image_height * scene.image_width) as usize;
    let mut final_pixels =
        vec![vec![Color::zero(); scene.image_width as usize]; scene.image_height as usize];

    // progress printing
    let mut scanline_completion_array = vec![0; scene.image_height as usize];
    let mut scanlines_remaining = scene.image_height;

    // initialize as all black
    println_f!("P3\n{scene.image_width} {scene.image_height}\n255\n");

    // maybe only use channels? aggregate color fully within thread, then once pixel is finished
    // send fractionally sampled pixel to awating main thread, which will collect and aggregate them
    // to build final output color.
    let (pixel_value_transmitter, pixel_value_receiver) = mpsc::channel();
    let (scanline_completion_transmitter, scanline_completion_receiver) = mpsc::channel();

    for thread in 0..num_threads {
        let objects = scene.world.objects.clone();
        let mut thread_world = RenderableList::new();
        for obj in objects {
            thread_world.add(obj)
        }
        let pixel_value_transmitter = pixel_value_transmitter.clone();
        let scanline_completion_transmitter = scanline_completion_transmitter.clone();
        let mut pixels =
            vec![vec![Color::zero(); scene.image_width as usize]; scene.image_height as usize];
        let handle = thread::spawn(move || {
            for j in 0..scene.image_height {
                for i in 0..scene.image_width {
                    // spawn threads
                    for _s in 0..samples_per_pixel_per_thread {
                        // random point for ray to shoot at within this pixel
                        let u = ((i as f32) + random_between_0_1()) / (scene.image_width as f32);
                        let v = ((j as f32) + random_between_0_1()) / (scene.image_height as f32);
                        let r = scene.camera.get_ray(u, v);
                        // starting call depth is 0 as increases until hitting bounce depth
                        pixels[j as usize][i as usize] += ray_color_world(&r, &thread_world, 0);
                    }
                }
                // eprintln_f!("Completed Scanline {j}");
                // notify that this thread has completed this scanline
                scanline_completion_transmitter.send(j).unwrap();
            }
            // send completed partial image once finished
            pixel_value_transmitter.send(pixels).unwrap();
            drop(pixel_value_transmitter);
            drop(scanline_completion_transmitter);
            eprintln_f!("Thread {thread} completed");
        });
        handles.push(handle);
    }

    // handle printing progress from threads
    for completed_thread_scanline in scanline_completion_receiver {
        scanline_completion_array[completed_thread_scanline as usize] += 1;
        if scanline_completion_array[completed_thread_scanline as usize] == num_threads {
            scanlines_remaining -= 1;
            eprintln_f!("Scanlines Remaining: {scanlines_remaining}");
        }
        if scanlines_remaining == 0 {
            break;
        }
    }

    for partially_rendered_scene in pixel_value_receiver {
        for y in 0..partially_rendered_scene.len() {
            for x in 0..partially_rendered_scene[y].len() {
                final_pixels[y][x] += partially_rendered_scene[y][x];
            }
        }
        num_threads -= 1;
        eprintln_f!("Threads Remaining: {num_threads}");
        if num_threads == 0 {
            break;
        }
    }

    drop(pixel_value_transmitter);
    drop(scanline_completion_transmitter);

    // clean up threads
    for handle in handles {
        handle.join().unwrap();
        eprintln_f!("Cleaning up threads");
    }

    // write final colors to output
    for y in (0..final_pixels.len()).rev() {
        for x in 0..final_pixels[y].len() {
            write_color_to_output(final_pixels[y][x], scene.samples_per_pixel);
        }
    }
}

fn render_multi_threaded_bvh(scene_path: &str, mut num_threads: i32) {
    let scene: Scene = load_scene(scene_path);

    let samples_per_pixel_per_thread =
        f32::round(scene.samples_per_pixel as f32 / num_threads as f32) as i32;

    let mut handles = vec![];
    // let num_pixels: usize= (scene.image_height * scene.image_width) as usize;
    let mut final_pixels =
        vec![vec![Color::zero(); scene.image_width as usize]; scene.image_height as usize];

    // progress printing
    let mut scanline_completion_array = vec![0; scene.image_height as usize];
    let mut scanlines_remaining = scene.image_height;

    // initialize as all black
    println_f!("P3\n{scene.image_width} {scene.image_height}\n255\n");

    // maybe only use channels? aggregate color fully within thread, then once pixel is finished
    // send fractionally sampled pixel to awating main thread, which will collect and aggregate them
    // to build final output color.
    let (pixel_value_transmitter, pixel_value_receiver) = mpsc::channel();
    let (scanline_completion_transmitter, scanline_completion_receiver) = mpsc::channel();

    for thread in 0..num_threads {
        let objects = scene.world.objects.clone();
        let root = BvhNode::new_from_renderables(&objects);
        let mut thread_world = RenderableList::new();
        for obj in objects {
            thread_world.add(obj)
        }
        let pixel_value_transmitter = pixel_value_transmitter.clone();
        let scanline_completion_transmitter = scanline_completion_transmitter.clone();
        let mut pixels =
            vec![vec![Color::zero(); scene.image_width as usize]; scene.image_height as usize];
        let handle = thread::spawn(move || {
            for j in 0..scene.image_height {
                for i in 0..scene.image_width {
                    // spawn threads
                    for _s in 0..samples_per_pixel_per_thread {
                        // random point for ray to shoot at within this pixel
                        let u = ((i as f32) + random_between_0_1()) / (scene.image_width as f32);
                        let v = ((j as f32) + random_between_0_1()) / (scene.image_height as f32);
                        let r = scene.camera.get_ray(u, v);
                        // starting call depth is 0 as increases until hitting bounce depth
                        pixels[j as usize][i as usize] += ray_color_bvh_node(&r, &root, 0);
                    }
                }
                // eprintln_f!("Completed Scanline {j}");
                // notify that this thread has completed this scanline
                scanline_completion_transmitter.send(j).unwrap();
            }
            // send completed partial image once finished
            pixel_value_transmitter.send(pixels).unwrap();
            drop(pixel_value_transmitter);
            drop(scanline_completion_transmitter);
            eprintln_f!("Thread {thread} completed");
        });
        handles.push(handle);
    }

    // handle printing progress from threads
    for completed_thread_scanline in scanline_completion_receiver {
        scanline_completion_array[completed_thread_scanline as usize] += 1;
        if scanline_completion_array[completed_thread_scanline as usize] == num_threads {
            scanlines_remaining -= 1;
            eprintln_f!("Scanlines Remaining: {scanlines_remaining}");
        }
        if scanlines_remaining == 0 {
            break;
        }
    }

    for partially_rendered_scene in pixel_value_receiver {
        for y in 0..partially_rendered_scene.len() {
            for x in 0..partially_rendered_scene[y].len() {
                final_pixels[y][x] += partially_rendered_scene[y][x];
            }
        }
        num_threads -= 1;
        eprintln_f!("Threads Remaining: {num_threads}");
        if num_threads == 0 {
            break;
        }
    }

    drop(pixel_value_transmitter);
    drop(scanline_completion_transmitter);

    // clean up threads
    for handle in handles {
        handle.join().unwrap();
        eprintln_f!("Cleaning up threads");
    }

    // write final colors to output
    for y in (0..final_pixels.len()).rev() {
        for x in 0..final_pixels[y].len() {
            write_color_to_output(final_pixels[y][x], scene.samples_per_pixel);
        }
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    // create_test("./test-texture.json");
    // create("test-texture.json");
    create_checker_test("test-checker-texture.json");
    let start = Instant::now();
    render_multi_threaded("scenes/test-checker-texture.json", 10);
    // render("scenes/test.json");
    let elapsed = start.elapsed().as_secs_f32();
    eprintln_f!("scene rendered in {elapsed}");
    /*
    render : single thread -
    render : multi-thread (5 threads) - 

    render : bvh : single thread - 
    render : bvh : multi-thread (5 threads) - 111.0266s
     */
}
