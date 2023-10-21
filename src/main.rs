/*
Massive credit to the wonderful authors behind:
[_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
This wouldn't have been made without them.
*/

extern crate sdl2;

use bvh::BvhNode;
use camera::Camera;
use hittable::{HittableList, Sphere};
use material::{Dielectric, Lambertian, Material, Metal};
use profiler::Profiler;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use texture::{CheckerTexture, SolidColor, Texture};
// use std::fmt;
use crate::{hittable::Quad, texture::ImageTexture, vector::Vec3, material::DiffuseLight};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::time::Instant;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

mod aabb;
mod bvh;
mod camera;
mod currentpath;
mod hittable;
mod interval;
mod material;
mod pixelbuffer;
mod profiler;
mod random;
mod ray;
mod texture;
mod vector;

fn main() {
    let current_path = currentpath::get_executable_directory();
    println!("{}", current_path);
    // Set up renderer.
    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
    let video_subsystem: sdl2::VideoSubsystem = sdl_context.video().unwrap();
    let window: Window = video_subsystem
        .window("Rustracer", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();
    let texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext> =
        canvas.texture_creator();
    let mut texture: sdl2::render::Texture<'_> = texture_creator
        .create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::RGBA32,
            pixelbuffer::WIDTH as u32,
            pixelbuffer::HEIGHT as u32,
        )
        .unwrap();
    let mut event_pump: sdl2::EventPump = sdl_context.event_pump().unwrap();
    let mut quit: bool = false;

    let start_time: Instant = Instant::now();

    let mut rng = XorShiftRng::from_seed([
        0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
        0x88,
    ]);

    // Set up raytracer.
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Quad::new(
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Box::new(DiffuseLight::new_color(Vec3::new(1.0, 1.0, 1.0)))
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(&Vec3::new(0.0, 0.3, 0.7), 0.0)),
    )));
    /*world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new_texture(Box::new(
            ImageTexture::new(&(current_path + "earthmap.png")).expect("Could not load image")
        ))),
    )));*/
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new_texture(Box::new(
            ImageTexture::new(&(current_path + "earthmap.png")).expect("Could not load image"),
        ))),
    )));

    world = HittableList::new_add(Box::new(BvhNode::new_list(&world, &mut rng)));

    let mut camera: Camera = Camera::new();

    camera.lookfrom = Vec3::new(-2.0, 2.0, 1.0);
    camera.lookat = Vec3::new(0.0, 0.0, -1.0);

    let mut delta_start_time: Instant = Instant::now();

    let mut camera_rotation: (f64, f64) = (180.0, 2.71);

    let mut mouse_locked = false;
    let mut mouse_x = 0;
    let mut mouse_y = 0;

    let mut prof: Profiler = Profiler::new();

    println!("Starting render");

    // Main loop.
    while !quit {
        prof.start("Main loop");
        prof.start("SDL2 Input events");
        mouse_x = 0;
        mouse_y = 0;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => quit = true,
                Event::MouseButtonDown { mouse_btn, .. } => {
                    // Lock the mouse when any mouse button is clicked
                    mouse_locked = true;
                    sdl_context.mouse().set_relative_mouse_mode(true);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    // Unlock the mouse when the Escape key is pressed
                    mouse_locked = false;
                    sdl_context.mouse().set_relative_mouse_mode(false);
                }
                Event::MouseMotion {
                    x, y, xrel, yrel, ..
                } => {
                    mouse_x = xrel;
                    mouse_y = yrel;
                }
                _ => {}
            }
        }

        if !mouse_locked {
            mouse_x = 0;
            mouse_y = 0;
        }

        prof.end("SDL2 Input events");

        // println!("{}, {}", mouse_x, mouse_y);

        // Main rendering
        prof.start("Rendering");
        let elapsed_time = start_time.elapsed().as_secs_f32();
        // println!("Started frame");
        camera.render(&mut world, &mut rng);
        // println!("Finished frame");
        prof.end("Rendering");

        // println!("Rendered frame. Second : {}", elapsed_time);

        prof.start("Screen Update");
        update_screen(&mut texture, &camera, &mut canvas);
        prof.end("Screen Update");

        prof.start("Movement");
        let delta_time: f64 = delta_start_time.elapsed().as_millis() as f64 / 16.0;
        delta_start_time = Instant::now();

        move_player(
            &mut camera_rotation,
            &event_pump,
            &mut camera,
            delta_time,
            &mouse_x,
            &mouse_y,
        );

        prof.end("Movement");

        println!("{} FPS", 62.5 / delta_time);
        prof.end("Main loop");
        prof.finish();
        // quit = true;
    }
}

fn move_player(
    camera_rotation: &mut (f64, f64),
    event_pump: &sdl2::EventPump,
    camera: &mut Camera,
    delta_time: f64,
    mouse_x: &i32,
    mouse_y: &i32,
) {
    const MOVE_SPEED: f64 = 0.1;

    camera_rotation.0 += delta_time * (*mouse_x as f64) / 5.0;
    camera_rotation.1 += delta_time * (*mouse_y as f64) / 5.0;

    let camera_rot_x: f64 = camera_rotation.0 * (std::f64::consts::PI / 180.0);
    let camera_rot_y: f64 = camera_rotation.1 * (std::f64::consts::PI / 180.0);
    // println!("{}, {}", camera_rotation.0, camera_rot_x);

    if event_pump
        .keyboard_state()
        .is_scancode_pressed(sdl2::keyboard::Scancode::W)
    {
        camera
            .lookfrom
            .setz(camera.lookfrom.z() - (MOVE_SPEED * delta_time * camera_rot_x.cos()));
        camera
            .lookfrom
            .setx(camera.lookfrom.x() + (MOVE_SPEED * delta_time * camera_rot_x.sin()));
    }
    if event_pump
        .keyboard_state()
        .is_scancode_pressed(sdl2::keyboard::Scancode::S)
    {
        camera
            .lookfrom
            .setz(camera.lookfrom.z() + (MOVE_SPEED * delta_time * camera_rot_x.cos()));
        camera
            .lookfrom
            .setx(camera.lookfrom.x() - (MOVE_SPEED * delta_time * camera_rot_x.sin()));
    }
    if event_pump
        .keyboard_state()
        .is_scancode_pressed(sdl2::keyboard::Scancode::A)
    {
        camera
            .lookfrom
            .setz(camera.lookfrom.z() - (MOVE_SPEED * delta_time * camera_rot_x.sin()));
        camera
            .lookfrom
            .setx(camera.lookfrom.x() - (MOVE_SPEED * delta_time * camera_rot_x.cos()));
    }
    if event_pump
        .keyboard_state()
        .is_scancode_pressed(sdl2::keyboard::Scancode::D)
    {
        camera
            .lookfrom
            .setz(camera.lookfrom.z() + (MOVE_SPEED * delta_time * camera_rot_x.sin()));
        camera
            .lookfrom
            .setx(camera.lookfrom.x() + (MOVE_SPEED * delta_time * camera_rot_x.cos()));
    }
    if event_pump
        .keyboard_state()
        .is_scancode_pressed(sdl2::keyboard::Scancode::Space)
    {
        camera
            .lookfrom
            .sety(camera.lookfrom.y() + (MOVE_SPEED * delta_time));
    }
    if event_pump
        .keyboard_state()
        .is_scancode_pressed(sdl2::keyboard::Scancode::LShift)
    {
        camera
            .lookfrom
            .sety(camera.lookfrom.y() - (MOVE_SPEED * delta_time));
    }

    /*if(keyboard_state[SDL_SCANCODE_A]) {
        cameraZ -= moveSpeed * delta * sin(rotX);
        cameraX -= moveSpeed * delta * cos(rotX);
    }
    if(keyboard_state[SDL_SCANCODE_D]) {
        cameraZ += moveSpeed * delta * sin(rotX);
        cameraX += moveSpeed * delta * cos(rotX);
    }*/

    let look_x: f64 = camera.lookfrom.x() + (camera_rot_x.sin() * camera_rot_y.cos());
    let look_y: f64 = camera.lookfrom.y() - (camera_rot_y.sin() * 1.5);
    let look_z: f64 = camera.lookfrom.z() - (camera_rot_x.cos() * camera_rot_y.cos());

    camera.lookat = Vec3::new(look_x, look_y, look_z);
}

fn update_screen(
    texture: &mut sdl2::render::Texture<'_>,
    camera: &Camera,
    canvas: &mut Canvas<Window>,
) {
    // Update the existing texture with the pixel buffer
    texture
        .update(
            None,
            &(*pixelbuffer::get_pixels(&camera.pixel_buffer)),
            pixelbuffer::WIDTH as usize * 4,
        )
        .expect("Could not convert viewbuffer into texture");

    // Clear the canvas
    canvas.clear();

    // Copy the texture to the canvas
    canvas
        .copy(
            &*texture,
            None,
            Some(Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT)),
        )
        .unwrap();

    // Update the existing texture with the pixel buffer
    /*texture
        .update(
            None,
            &(*pixelbuffer::get_pixels(&camera.pixel_buffer_2)),
            pixelbuffer::WIDTH as usize * 4,
        )
        .expect("Could not convert viewbuffer into texture");

    canvas
        .copy(
            &*texture,
            None,
            Some(Rect::new(
                0,
                (SCREEN_HEIGHT / 2) as i32,
                SCREEN_WIDTH,
                (SCREEN_HEIGHT / 2) as u32,
            )),
        )
        .unwrap();*/

    // Present the canvas
    canvas.present();
}
