/*
Massive credit to the wonderful authors behind:
[_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
This wouldn't have been made without them.
*/

extern crate sdl2;

use hittable::base::{HitRecord, Hittable};
use hittable::hittable_list::HittableList;
use hittable::sphere::Sphere;
use interval::Interval;
use material::lambertian::Lambertian;
use material::metal::Metal;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use ray::Ray;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
// use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use utils::{degrees_to_radians, random_double};
use vector::Vec3;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

const SAMPLES: i32 = 20;
const MAX_DEPTH: i32 = 50;

const THREAD_ROWS: usize = 12;

mod interval;
mod pixel_buffer;
mod ray;
mod utils;
mod vector;

mod hittable {
    pub mod base;
    pub mod hittable_list;
    pub mod sphere;
}

mod material {
    pub mod base;
    pub mod lambertian;
    pub mod metal;
}

fn main() {
    // Set up renderer.
    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
    let video_subsystem: sdl2::VideoSubsystem = sdl_context.video().unwrap();
    let window: Window = video_subsystem
        .window("Rustracer", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas: Canvas<Window> = window.into_canvas().present_vsync().build().unwrap();
    let texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext> =
        canvas.texture_creator();

    let mut textures: Vec<sdl2::render::Texture> = vec![];
    for _i in 0..THREAD_ROWS {
        textures.push(
            texture_creator
                .create_texture_streaming(
                    PixelFormatEnum::ABGR8888,
                    pixel_buffer::WIDTH as u32,
                    (pixel_buffer::HEIGHT / THREAD_ROWS) as u32,
                )
                .unwrap(),
        );
    }

    let mut event_pump: sdl2::EventPump = sdl_context.event_pump().unwrap();
    let mut quit: bool = false;

    let mut delta_start_time: Instant = Instant::now();

    let mut mouse_locked = false;
    let mut mouse_x = 0;
    let mut mouse_y = 0;

    let size = (pixel_buffer::WIDTH * pixel_buffer::HEIGHT * 4) / THREAD_ROWS as usize;
    let mut buffers: Vec<Mutex<Box<[u8]>>> = vec![];
    for _i in 0..THREAD_ROWS {
        buffers.push(Mutex::new(vec![0; size].into_boxed_slice()));
    }

    // Camera stuff.
    let mut pixel_delta_u: Vec3 = Vec3::new_default();
    let mut pixel_delta_v: Vec3 = Vec3::new_default();
    let mut camera_center: Vec3 = Vec3::new_default();
    let mut pixel00_loc: Vec3 = Vec3::new_default();

    initialize_camera(
        &mut camera_center,
        &mut pixel_delta_u,
        &mut pixel_delta_v,
        &mut pixel00_loc,
        90.0,
    );

    // Thread stuff.

    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::new(
        &Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(&Vec3::new(0.8, 0.6, 0.2), 0.3)),
    )));
    world.add(Box::new(Sphere::new(
        &Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new(&Vec3::new(0.8, 0.8, 0.0))),
    )));

    let shared_vec = Arc::new(buffers);
    let shared_world = Arc::new(world);

    for i in 0..THREAD_ROWS {
        let arc_clone = Arc::clone(&shared_vec);
        let world_clone = Arc::clone(&shared_world);

        std::thread::spawn(move || {
            render_loop(
                i,
                pixel00_loc,
                pixel_delta_u,
                pixel_delta_v,
                camera_center,
                world_clone,
                arc_clone,
            )
        });
    }

    // Main loop.
    while !quit {
        handle_events(
            &mut mouse_x,
            &mut mouse_y,
            &mut event_pump,
            &mut quit,
            &mut mouse_locked,
            &sdl_context,
        );

        update_screen(
            &mut textures,
            &shared_vec,
            &mut canvas,
            &mut camera_center,
            &mut pixel_delta_u,
            &mut pixel_delta_v,
            &mut pixel00_loc,
            90.0,
        );

        // let delta_time = calculate_delta_time(&mut delta_start_time);
    }
}

fn initialize_camera(
    camera_center: &mut Vec3,
    pixel_delta_u: &mut Vec3,
    pixel_delta_v: &mut Vec3,
    pixel00_loc: &mut Vec3,
    vfov: f64,
) {
    let theta = degrees_to_radians(vfov);
    let h = (theta / 2.0).tan();
    let viewport_height = 2.0 * h * 1.0;
    let viewport_width: f64 =
        viewport_height * (pixel_buffer::WIDTH as f64 / pixel_buffer::HEIGHT as f64);

    let focal_length = 1.0;
    *camera_center = Vec3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    *pixel_delta_u = viewport_u / (pixel_buffer::WIDTH as f64);
    *pixel_delta_v = viewport_v / (pixel_buffer::HEIGHT as f64);

    let viewport_upper_left =
        *camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    *pixel00_loc = viewport_upper_left + 0.5 * (*pixel_delta_u + *pixel_delta_v);
}

fn render_loop(
    i: usize,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    camera_center: Vec3,
    world_clone: Arc<HittableList>,
    arc_clone: Arc<Vec<Mutex<Box<[u8]>>>>,
) -> ! {
    // Initialize random number generator.
    // Random seed from ChatGPT.
    let mut rng = XorShiftRng::from_seed([
        0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
        0x88,
    ]);

    // let start = Instant::now();
    loop {
        for y in (i * (pixel_buffer::HEIGHT / THREAD_ROWS))
            ..(i + 1) * (pixel_buffer::HEIGHT / THREAD_ROWS)
        {
            for x in 0..pixel_buffer::WIDTH {
                let mut pixel_color: Vec3 = Vec3::new_default();
                for _j in 0..SAMPLES {
                    let ray = get_ray(
                        pixel00_loc,
                        x,
                        pixel_delta_u,
                        y,
                        pixel_delta_v,
                        camera_center,
                        &mut rng,
                    );
                    pixel_color = pixel_color + ray_color(&ray, MAX_DEPTH, &*world_clone, &mut rng);
                }
                write_color(&arc_clone, x, y, pixel_color, SAMPLES, &i);
            }
        }
    }
}

fn get_ray(
    pixel00_loc: Vec3,
    x: usize,
    pixel_delta_u: Vec3,
    y: usize,
    pixel_delta_v: Vec3,
    camera_center: Vec3,
    rng: &mut XorShiftRng,
) -> Ray {
    let pixel_center = pixel00_loc + ((x as f64) * pixel_delta_u) + ((y as f64) * pixel_delta_v);
    let pixel_sample = pixel_center + pixel_sample_square(rng, &pixel_delta_u, &pixel_delta_v);

    let ray_origin = camera_center;
    let ray_direction = pixel_sample - ray_origin;

    Ray::new(&camera_center, &ray_direction)
}

fn pixel_sample_square(rng: &mut XorShiftRng, pixel_delta_u: &Vec3, pixel_delta_v: &Vec3) -> Vec3 {
    let px = -0.5 + random_double(rng);
    let py = -0.5 + random_double(rng);
    return (px * *pixel_delta_u) + (py * *pixel_delta_v);
}

fn ray_color(r: &Ray, depth: i32, world: &HittableList, rng: &mut XorShiftRng) -> Vec3 {
    let mut hit_record: HitRecord = HitRecord::new();

    if depth <= 0 {
        return Vec3::new_default();
    }

    if world.hit(
        r,
        &Interval::new(0.001, std::f64::INFINITY),
        &mut hit_record,
    ) {
        let mut scattered: Ray = Ray::new(&Vec3::new_default(), &Vec3::new_default());
        let mut attenuation: Vec3 = Vec3::new_default();
        if hit_record
            .mat
            .scatter(r, &hit_record, &mut attenuation, &mut scattered, rng)
        {
            return attenuation * ray_color(&scattered, depth - 1, world, rng);
        }
        return Vec3::new(1.0, 0.0, 1.0);
    }

    let sky_bottom_color: Vec3 = Vec3::new(1.0, 1.0, 1.0);
    let sky_top_color: Vec3 = Vec3::new(0.5, 0.7, 1.0);

    let unit_direction: Vec3 = r.direction.unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);

    // Linearly interpolate between bottom and top color
    (1.0 - a) * sky_bottom_color + a * sky_top_color
}

fn write_color(
    arc_clone: &Arc<Vec<Mutex<Box<[u8]>>>>,
    x: usize,
    y: usize,
    pixel_color: Vec3,
    samples_per_pixel: i32,
    i: &usize,
) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    // Apply the linear to gamma transform.
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    pixel_buffer::set(
        &mut *arc_clone[*i].lock().expect("Could not lock screen mutex"),
        x as usize,
        y as usize % (pixel_buffer::HEIGHT / THREAD_ROWS),
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
    );
}

fn linear_to_gamma(r: f64) -> f64 {
    r.sqrt()
}

fn handle_events(
    mouse_x: &mut i32,
    mouse_y: &mut i32,
    event_pump: &mut sdl2::EventPump,
    quit: &mut bool,
    mouse_locked: &mut bool,
    sdl_context: &sdl2::Sdl,
) {
    *mouse_x = 0;
    *mouse_y = 0;
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => *quit = true,
            Event::MouseButtonDown { /*mouse_btn,*/ .. } => {
                // Lock the mouse when any mouse button is clicked
                *mouse_locked = true;
                sdl_context.mouse().set_relative_mouse_mode(true);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                // Unlock the mouse when the Escape key is pressed
                *mouse_locked = false;
                sdl_context.mouse().set_relative_mouse_mode(false);
            }
            Event::MouseMotion {
                xrel, yrel, ..
            } => {
                *mouse_x = xrel;
                *mouse_y = yrel;
            }
            _ => {}
        }
    }

    if !*mouse_locked {
        *mouse_x = 0;
        *mouse_y = 0;
    }
}

fn calculate_delta_time(delta_start_time: &mut Instant) -> f64 {
    let delta_time: f64 = delta_start_time.elapsed().as_millis() as f64 / 16.0;
    *delta_start_time = Instant::now();
    delta_time
}

fn update_screen(
    texture: &mut Vec<sdl2::render::Texture<'_>>,
    camera: &Vec<Mutex<Box<[u8]>>>,
    canvas: &mut Canvas<Window>,
    camera_center: &mut Vec3,
    pixel_delta_u: &mut Vec3,
    pixel_delta_v: &mut Vec3,
    pixel00_loc: &mut Vec3,
    vfov: f64,
) {
    // Clear the canvas
    canvas.clear();
    for i in 0..THREAD_ROWS {
        // Update the existing texture with the pixel buffer
        texture[i]
            .update(
                None,
                &(*pixel_buffer::get_pixels(
                    &camera[i]
                        .lock()
                        .expect("Could not lock mutex for updating texture"),
                )),
                pixel_buffer::WIDTH as usize * 4,
            )
            .expect("Could not convert viewbuffer into texture");

        // Copy the texture to the canvas
        canvas
            .copy(
                &texture[i],
                None,
                Some(Rect::new(
                    0,
                    (i * (SCREEN_HEIGHT as usize / THREAD_ROWS)) as i32,
                    SCREEN_WIDTH,
                    SCREEN_HEIGHT / THREAD_ROWS as u32,
                )),
            )
            .unwrap();
    }

    // Present the canvas
    canvas.present();

    initialize_camera(
        camera_center,
        pixel_delta_u,
        pixel_delta_v,
        pixel00_loc,
        vfov,
    );
}
