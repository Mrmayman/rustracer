/*
Massive credit to the wonderful authors behind:
[_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
This wouldn't have been made without them.
*/

extern crate sdl2;

use bvh::BvhNode;
use camera::Camera;
use hittable::{HittableList, Sphere};
use material::{};
use sdl2::event::{self, Event};
use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
// use std::fmt;
use crate::{hittable::Quad, material::DiffuseLight, texture::ImageTexture, vector::Vec3};
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use std::collections::HashSet;
use std::{rc::Rc, time::Instant};

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
mod worldconstructor;

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

    // let start_time: Instant = Instant::now();

    let mut rng = XorShiftRng::from_seed([
        0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
        0x88,
    ]);

    let mut world = init_world(current_path, &mut rng);

    let mut camera: Camera = Camera::new();

    camera.lookfrom = Vec3::new(0.0, 2.0, 0.0);
    camera.lookat = Vec3::new(0.0, 0.0, -1.0);

    let mut delta_start_time: Instant = Instant::now();

    let mut car_rotation: f64 = 180.0;

    let mut mouse_locked = false;
    let mut mouse_x = 0;
    let mut mouse_y = 0;

    // let mut prof: Profiler = Profiler::new();

    println!("Starting render");

    let mut car_pos: (f64, f64, f64) = (4.0, 0.0, 5.0);

    let mut forward_velocity = 0.0;

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

        camera.lookat = Vec3::new(car_pos.0, car_pos.1 + 1.0, car_pos.2);

        camera.lookfrom = Vec3::new(
            car_pos.0 + 5.0 * (car_rotation * (std::f64::consts::PI / 180.0)).sin(),
            car_pos.1 + 3.0,
            car_pos.2 + 5.0 * (car_rotation * (std::f64::consts::PI / 180.0)).cos(),
        );

        camera.render(&mut world, &mut rng);

        update_screen(&mut texture, &camera, &mut canvas);

        let delta_time = calculate_delta_time(&mut delta_start_time);

        move_player(
            &mut car_rotation,
            &event_pump,
            &mut camera,
            delta_time,
            &mouse_x,
            &mouse_y,
            &mut world,
            &mut car_pos,
            &mut forward_velocity
        );

        // camera_rotation += 1.0 * delta_time;

        println!("{} FPS", 62.5 / delta_time);
    }
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
            Event::MouseButtonDown { mouse_btn, .. } => {
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
                x, y, xrel, yrel, ..
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

fn init_world(current_path: String, rng: &mut XorShiftRng) -> HittableList {
    // Add stuff to world
    let mut world: HittableList = HittableList::new();

    worldconstructor::construct_world(current_path, &mut world);

    world = HittableList::new_add(Box::new(BvhNode::new_list(&world, rng)));

    worldconstructor::construct_car(&mut world);

    world
}

fn move_player(
    camera_rotation: &mut f64,
    event_pump: &sdl2::EventPump,
    camera: &mut Camera,
    delta_time: f64,
    mouse_x: &i32,
    mouse_y: &i32,
    world: &mut HittableList,
    car_pos: &mut (f64, f64, f64),
    forward_velocity: &mut f64,
) {
    const MOVE_SPEED: f64 = 0.1;

    // camera_rotation.0 += delta_time * (*mouse_x as f64) / 5.0;
    // camera_rotation.1 += delta_time * (*mouse_y as f64) / 5.0;

    let mut camera_rot_x: f64 = *camera_rotation * (std::f64::consts::PI / 180.0);
    // let camera_rot_y: f64 = camera_rotation.1 * (std::f64::consts::PI / 180.0);

    let pressed: HashSet<Keycode> = event_pump
        .keyboard_state()
        .pressed_scancodes()
        .filter_map(Keycode::from_scancode)
        .collect();

    if pressed.contains(&sdl2::keyboard::Keycode::W) {
        *forward_velocity = 0.2;
    } else {
        *forward_velocity = 0.0;
    }
    println!("{}", forward_velocity);
    car_pos.2 -= (*forward_velocity * delta_time * camera_rot_x.cos());
    car_pos.0 -= (*forward_velocity * delta_time * camera_rot_x.sin());
    /*if pressed.contains(&sdl2::keyboard::Keycode::S) {
        camera
            .lookfrom
            .setz(camera.lookfrom.z() + (MOVE_SPEED * delta_time * camera_rot_x.cos()));
        camera
            .lookfrom
            .setx(camera.lookfrom.x() - (MOVE_SPEED * delta_time * camera_rot_x.sin()));
    }*/
    if pressed.contains(&sdl2::keyboard::Keycode::A) {
        *camera_rotation += 4.0;
        camera_rot_x = *camera_rotation * (std::f64::consts::PI / 180.0);
    }
    if pressed.contains(&sdl2::keyboard::Keycode::D) {
        *camera_rotation -= 4.0;
        camera_rot_x = *camera_rotation * (std::f64::consts::PI / 180.0);
    }
    /*if pressed.contains(&sdl2::keyboard::Keycode::Space) {
        camera
            .lookfrom
            .sety(camera.lookfrom.y() + (MOVE_SPEED * delta_time));
    }
    if pressed.contains(&sdl2::keyboard::Keycode::LShift) {
        camera
            .lookfrom
            .sety(camera.lookfrom.y() - (MOVE_SPEED * delta_time));
    }*/

    /*if(keyboard_state[SDL_SCANCODE_A]) {
        cameraZ -= moveSpeed * delta * sin(rotX);
        cameraX -= moveSpeed * delta * cos(rotX);
    }
    if(keyboard_state[SDL_SCANCODE_D]) {
        cameraZ += moveSpeed * delta * sin(rotX);
        cameraX += moveSpeed * delta * cos(rotX);
    }*/

    for i in 1..12 {
        world.objects[i].set_translate(*car_pos);
    }

    // let look_x: f64 = camera.lookfrom.x() + (camera_rot_x.sin() * camera_rot_y.cos());
    // let look_y: f64 = camera.lookfrom.y() - (camera_rot_y.sin() * 1.5);
    // let look_z: f64 = camera.lookfrom.z() - (camera_rot_x.cos() * camera_rot_y.cos());

    // camera.lookat = Vec3::new(look_x, look_y, look_z);
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
