//! # RuSTracer RENDERer
//!
//! A simple, fast, and easy-to-use ray tracing renderer,
//! capable of running on lower-end hardware (including
//! integrated graphics).
//!
//! ## Features
//! - Real-time ray tracing
//! - Customizable materials (lambertian (matte),
//!   metal, dielectric (glass), emissive (light))
//! - Simple to use but quite flexible
//! - Reasonably high performance (although there is
//!   still room for improvement)
//!
//! ## To-Do
//! - Texture support
//! - Noise texture support
//! - Hybrid rendering (ray tracing + rasterization)
//! - Skyboxes
//! - Sky models (procedural)
//! - Volumetric rendering (fog, smoke, etc.)
//! - Point lights
//!
//! ## Example
//! ```no_run
//! use std::sync::Arc;
//! use winit::keyboard::{KeyCode, PhysicalKey};
//! use winit::{event_loop::EventLoop, window::WindowBuilder};
//!
//! fn main() {
//!     let event_loop = EventLoop::new().unwrap();
//!     let window = WindowBuilder::new()
//!         .with_title("My Application")
//!         .with_inner_size(winit::dpi::PhysicalSize {
//!             width: 800,
//!             height: 600,
//!         })
//!         .build(&event_loop)
//!         .unwrap();
//!     let window = Arc::new(window);
//!     pollster::block_on(run(window.clone(), event_loop));
//! }
//!
//! async fn run(window: Arc<winit::window::Window>, event_loop: EventLoop<()>) {
//!     let materials = vec![rst_render::Material::Lambertian {
//!         albedo: [1.0, 1.0, 0.0],
//!         _padding: Default::default(),
//!     }];
//!
//!     let renderer = rst_render::Renderer::new(
//!         window.clone(),
//!         &materials,
//!         rst_render::ShaderConfig {
//!             samples: 4,
//!             bounces: 4,
//!             antialiasing: false,
//!             motion_blur: true,
//!             downscale: 4.0,
//!         },
//!     )
//!     .await
//!     .unwrap();
//!
//!     renderer.run(event_loop, |events, frame| {
//!         for event in events {
//!             match event {
//!                 winit::event::WindowEvent::KeyboardInput { event, .. } => {
//!                     if let PhysicalKey::Code(KeyCode::Escape) = event.physical_key {
//!                         frame.exit = true;
//!                     }
//!                 }
//!                 _ => {}
//!             }
//!         }
//!
//!         // Render resolution will be 4 times smaller
//!         // than window resolution
//!         frame.set_scale(4.0);
//!
//!         frame.is_mouse_locked = false;
//!
//!         frame.camera_pos = [0.5, 0.0, 1.0];
//!         frame.camera_dir = rst_render::LookDirection::AtPoint(0.5, 0.5, 0.0);
//!
//!         let object = vec![rst_render::Triangle {
//!             material: 0, // Index of the material in the materials list
//!             #[rustfmt::skip]
//!             geometry: rst_render::Geometry {
//!                 ax: 0.0, ay: 0.0, az: 0.0,
//!                 bx: 1.0, by: 0.0, bz: 0.0,
//!                 cx: 0.5, cy: 1.0, cz: 0.0,
//!             },
//!             _padding: Default::default(),
//!         }];
//!
//!         let objects = vec![object];
//!
//!         // Send an object at the end of every frame.
//!         // You can store it wherever you want, but
//!         // just return it here every frame.
//!         // (clone or convert if necessary)
//!         objects
//!     });
//! }
//! ```

use std::sync::Arc;

use application::{Application, WORKGROUP_SIZE};
use objects::BoundingBox;
use winit::{
    dpi::{PhysicalPosition, Position},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

mod application;
mod error;
mod objects;
pub use application::LookDirection;
pub use error::Error;
pub use objects::{material::Material, Geometry, Triangle};

/// The configuration for the shader.
pub struct ShaderConfig {
    /// The number of samples per pixel.
    ///
    /// Higher is slower, but less noisy.
    ///
    /// Lower is faster, but more noisy.
    ///
    /// 2 for performance, 4 for a good balance, 8+ for quality.
    pub samples: u32,
    /// The number of bounces for the ray.
    ///
    /// Higher is slower, but more realistic.
    ///
    /// 3 for performance, 4 for a good balance, 6+ for quality.
    pub bounces: u32,
    /// Whether to enable antialiasing.
    ///
    /// Not recommended with motion blur (or in general)
    /// as it makes things look blurry.
    pub antialiasing: bool,
    /// Whether to enable motion blur.
    ///
    /// Makes moving objects look blurry.
    /// Reduces noise by half so you can
    /// cut the samples in half with this on.
    pub motion_blur: bool,
    /// The downscale factor.
    ///
    /// The scale determines the rendering resolution.
    ///
    /// Higher is faster, lower is more detailed.
    pub downscale: f32,
}

/// The user-modifiable state of the frame.
///
/// Just edit the fields you need to change,
/// and the renderer will take care of the rest.
pub struct FrameState {
    /// Whether the mouse is locked
    /// (if it is, the cursor will be hidden and locked to the center of the window)
    pub is_mouse_locked: bool,
    /// The position of the camera
    pub camera_pos: [f32; 3],
    /// The direction the camera is looking at
    pub camera_dir: LookDirection,
    /// The time since the last frame (1.0 = 60 FPS)
    pub delta_time: f64,
    /// Whether to exit the application
    pub exit: bool,
    scale: f32,
    scale_changed: bool,
}

impl FrameState {
    /// Set the scale of the window.
    ///
    /// The scale determines the rendering resolution.
    ///
    /// Higher is faster, lower is more detailed.
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
        self.scale_changed = true;
    }

    /// Get the scale of the window.
    ///
    /// The scale determines the rendering resolution.
    ///
    /// Higher is faster, lower is more detailed.
    pub fn get_scale(&self) -> f32 {
        self.scale
    }
}

/// The main renderer.
///
/// This is the main struct you will interact with.
pub struct Renderer<'a> {
    app: Application<'a>,
    refresh_rate: f64,
    window: Arc<winit::window::Window>,
    window_event_buf: Vec<WindowEvent>,
}

impl Renderer<'_> {
    /// Create a new renderer.
    pub async fn new(
        window: Arc<winit::window::Window>,
        materials: &[Material],
        config: ShaderConfig,
    ) -> Result<Self, Error> {
        let refresh_rate = window
            .primary_monitor()
            .map(|monitor| monitor.refresh_rate_millihertz().map(|n| n as f64 / 1000.0))
            .flatten()
            .unwrap_or(60.0);

        Ok(Self {
            app: Application::new(window.clone(), materials, config).await?,
            refresh_rate,
            window,
            window_event_buf: Vec::new(),
        })
    }

    /// Runs the renderer. This will block the current thread
    /// and run the renderer until the window is closed.
    ///
    /// Takes a closure that will be called every frame.
    ///
    /// Every frame in the closure, you should return a
    /// `Vec<Vec<Triangle>>` where each `Vec<Triangle>`
    /// is a separate object.
    pub fn run<F: FnMut(&[WindowEvent], &mut FrameState) -> Vec<Vec<Triangle>>>(
        mut self,
        event_loop: EventLoop<()>,
        mut f: F,
    ) {
        event_loop
            .run(move |event, target| {
                target.set_control_flow(ControlFlow::Poll);
                match event {
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => target.exit(),
                        WindowEvent::Resized(new_size) => {
                            self.app.resize_window(new_size);
                        }
                        WindowEvent::RedrawRequested => {
                            let mut frame_state = FrameState {
                                is_mouse_locked: self.app.is_mouse_locked,
                                camera_pos: self.app.camera_pos,
                                camera_dir: self.app.camera_dir.clone(),
                                delta_time: self.app.time_elapsed * (1000.0 / 60.0),
                                exit: false,
                                scale: self.app.scale_factor,
                                scale_changed: false,
                            };

                            let objects = f(&self.window_event_buf, &mut frame_state);

                            self.app.is_mouse_locked = frame_state.is_mouse_locked;
                            self.app.camera_pos = frame_state.camera_pos;
                            self.app.camera_dir = frame_state.camera_dir;
                            self.app.scale_factor = frame_state.scale;
                            if frame_state.scale_changed {
                                self.app.resize_window(self.window.inner_size());
                            }
                            if frame_state.exit {
                                target.exit();
                            }
                            self.tick_frame(objects);
                        }
                        event => {
                            self.window_event_buf.push(event);
                        }
                    },
                    _ => {}
                }
            })
            .unwrap()
    }

    fn tick_frame(&mut self, objects: Vec<Vec<Triangle>>) {
        puffin::GlobalProfiler::lock().new_frame();
        puffin::profile_scope!("redraw");

        self.window_event_buf.clear();

        self.app.objects_list.objects.clear();
        self.app.bbox_list.objects.clear();
        for object in objects {
            let start_idx = self.app.objects_list.objects.len();
            self.app.objects_list.objects.extend_from_slice(&object);
            let stop_idx = self.app.objects_list.objects.len();
            let mut bbox: Option<BoundingBox> = None;
            for tri in &object {
                if let Some(bbox) = &mut bbox {
                    bbox.expand_to_tri(&tri);
                } else {
                    bbox = Some(BoundingBox::wrap(&tri));
                }
            }
            if let Some(mut bbox) = bbox {
                bbox.start_idx = start_idx as u32;
                bbox.end_idx = stop_idx as u32;
                bbox.fix();
                self.app.bbox_list.objects.push(bbox);
            }
        }

        println!(
            "{} {}",
            self.app.bbox_list.existing_len,
            self.app.objects_list.objects.len()
        );
        // std::process::exit(0);

        if self
            .app
            .objects_list
            .update(&self.app.queue, &self.app.device)
        {
            self.app.bbox_list.update(&self.app.queue, &self.app.device);
            self.app.update_compute_bind_group();
        } else if self.app.bbox_list.update(&self.app.queue, &self.app.device) {
            self.app.update_compute_bind_group();
        }

        self.app.render();

        {
            let x = self.app.surface_config.width as f32 / self.app.scale_factor;
            let y = self.app.surface_config.height as f32 / self.app.scale_factor;
            println!(
                "{:.2} FPS, Resolution: {} x {} ({} x {} workgroups)",
                1.0 / self.app.time_elapsed,
                x.ceil(),
                y.ceil(),
                (x / WORKGROUP_SIZE).ceil(),
                (y / WORKGROUP_SIZE).ceil()
            );
        }

        let remaining_time = (1.0 / self.refresh_rate) - self.app.time_elapsed;
        if remaining_time > 0.0 {
            // println!("Sleep: {:.1}", remaining_time * 1000.0);
            // std::thread::sleep(std::time::Duration::from_secs_f64(remaining_time));
            // std::thread::sleep(std::time::Duration::from_secs_f64(0.1));
        }

        update_mouse_lock(&self.app, &self.window);

        self.app.update_camera();
        self.app.update_data_buffer();
        self.window.request_redraw();

        self.app.time_elapsed = self.app.last_frame_time.elapsed().as_secs_f64();
        self.app.last_frame_time = std::time::Instant::now();
    }
}

fn update_mouse_lock(app: &Application<'_>, window: &Arc<winit::window::Window>) {
    if app.is_mouse_locked {
        if let Err(_) = window.set_cursor_grab(winit::window::CursorGrabMode::Locked) {
            let window_size = window.inner_size();
            if let Err(err) = window.set_cursor_position(Position::Physical(PhysicalPosition::new(
                window_size.width as i32 / 2,
                window_size.height as i32 / 2,
            ))) {
                eprintln!("warning (cursor move): {err}");
            }
        }
    } else {
        window
            .set_cursor_grab(winit::window::CursorGrabMode::None)
            .unwrap();
    }
    window.set_cursor_visible(!app.is_mouse_locked);
}
