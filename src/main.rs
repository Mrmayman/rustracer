use std::sync::Arc;

use application::{Application, LookDirection, WORKGROUP_SIZE};

use winit::{
    dpi::{PhysicalPosition, Position},
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod application;
mod objects;

/// This indicates how much to downscale the image.
/// 4.0 means it will reduce resolution by 4x (example: from 1080p to 270p).
/// Higher values are faster but lower resolution.
const SCALE_FACTOR: f32 = 4.0;

async fn run(event_loop: EventLoop<()>, window: Arc<winit::window::Window>) {
    let mut app = Application::new(window.clone()).await;

    println!("Started application");

    event_loop
        .run(move |event, target| {
            target.set_control_flow(ControlFlow::Poll);
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => target.exit(),
                    WindowEvent::Resized(new_size) => {
                        app.resize_window(new_size);
                    }
                    WindowEvent::RedrawRequested => {
                        puffin::GlobalProfiler::lock().new_frame();
                        puffin::profile_scope!("redraw");
                        app.tick();

                        let time_elapsed = app.last_frame_time.elapsed().as_secs_f64();
                        let remaining_time = (1.0 / 60.0) - time_elapsed;
                        println!(
                            "{:.2} FPS, Resolution: {} x {} ({} x {} workgroups)",
                            1.0 / time_elapsed,
                            (app.surface_config.width as f32 / app.scale_factor).ceil(),
                            (app.surface_config.height as f32 / app.scale_factor).ceil(),
                            (app.surface_config.width as f32 / (app.scale_factor * WORKGROUP_SIZE))
                                .ceil(),
                            (app.surface_config.height as f32
                                / (app.scale_factor * WORKGROUP_SIZE))
                                .ceil()
                        );
                        if remaining_time > 0.0 {
                            // std::thread::sleep(std::time::Duration::from_secs_f64(remaining_time));
                        }

                        let movement_speed: f32 = time_elapsed as f32 * 2.0;
                        if let LookDirection::InDirection(_, roty) = &app.camera_dir {
                            if app.keys_pressed.contains(&winit::keyboard::KeyCode::KeyW) {
                                app.camera_pos[0] += movement_speed * roty.cos();
                                app.camera_pos[2] += movement_speed * roty.sin();
                            }
                            if app.keys_pressed.contains(&winit::keyboard::KeyCode::KeyS) {
                                app.camera_pos[0] -= movement_speed * roty.cos();
                                app.camera_pos[2] -= movement_speed * roty.sin();
                            }
                            if app.keys_pressed.contains(&winit::keyboard::KeyCode::KeyA) {
                                app.camera_pos[0] += movement_speed * roty.sin();
                                app.camera_pos[2] -= movement_speed * roty.cos();
                            }
                            if app.keys_pressed.contains(&winit::keyboard::KeyCode::KeyD) {
                                app.camera_pos[0] -= movement_speed * roty.sin();
                                app.camera_pos[2] += movement_speed * roty.cos();
                            }
                            if app.keys_pressed.contains(&winit::keyboard::KeyCode::Space) {
                                app.camera_pos[1] += movement_speed;
                            }
                            if app
                                .keys_pressed
                                .contains(&winit::keyboard::KeyCode::ShiftLeft)
                            {
                                app.camera_pos[1] -= movement_speed;
                            }
                        }

                        update_mouse_lock(&app, &window);

                        app.update_camera();
                        app.update_data_buffer();
                        window.request_redraw();
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        if let winit::keyboard::PhysicalKey::Code(code) = event.physical_key {
                            if event.state.is_pressed() {
                                app.keys_pressed.insert(code);
                            } else {
                                app.keys_pressed.remove(&code);
                            }
                        }
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        if state.is_pressed() && matches!(button, MouseButton::Left) {
                            app.is_mouse_locked = !app.is_mouse_locked;
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        if app.is_mouse_locked {
                            let window_size = window.inner_size();
                            let dirx = position.x - window_size.width as f64 / 2.0;
                            let diry = position.y - window_size.height as f64 / 2.0;

                            let sensitivity =
                                std::cmp::max(window_size.width, window_size.height) as f32;

                            if dirx != 0.0 || diry != 0.0 {
                                if let LookDirection::InDirection(x, y) = &mut app.camera_dir {
                                    *x -= diry as f32 / sensitivity;
                                    *y += dirx as f32 / sensitivity;

                                    *x = (*x).clamp(
                                        -std::f32::consts::FRAC_PI_2,
                                        std::f32::consts::FRAC_PI_2,
                                    );
                                }
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        })
        .unwrap();
}

fn update_mouse_lock(app: &Application<'_>, window: &Arc<winit::window::Window>) {
    if app.is_mouse_locked {
        if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::Locked) {
            eprintln!("warning (cursor grab): {err}");
        }
        let window_size = window.inner_size();
        if let Err(err) = window.set_cursor_position(Position::Physical(PhysicalPosition::new(
            window_size.width as i32 / 2,
            window_size.height as i32 / 2,
        ))) {
            eprintln!("warning (cursor move): {err}");
        }
    } else {
        if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::None) {
            eprintln!("warning (cursor grab): {err}");
        }
    }
    window.set_cursor_visible(!app.is_mouse_locked);
}

fn main() {
    let server_addr = format!("0.0.0.0:{}", puffin_http::DEFAULT_PORT);
    let _puffin_server = puffin_http::Server::new(&server_addr).unwrap();
    eprintln!("Serving demo profile data on {server_addr}. Run `puffin_viewer` to view it.");

    puffin::set_scopes_on(true);

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Rustracer")
        .with_inner_size(winit::dpi::PhysicalSize {
            width: 800,
            height: 600,
        })
        .build(&event_loop)
        .unwrap();

    pollster::block_on(run(event_loop, Arc::new(window)));
}
