use std::sync::Arc;

use application::{Application, WORKGROUP_SIZE};
use objects::BoundingBox;
use winit::{
    dpi::{PhysicalPosition, Position},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

mod application;
mod objects;
pub use application::LookDirection;
pub use objects::{material::Material, Geometry, Triangle};

pub struct ShaderConfig {
    pub samples: u32,
    pub bounces: u32,
    pub antialiasing: bool,
    pub motion_blur: bool,
    pub downscale: f32,
}

pub struct FrameState {
    pub is_mouse_locked: bool,
    pub camera_pos: [f32; 3],
    pub camera_dir: LookDirection,
    pub delta_time: f64,
    scale: f32,
    scale_changed: bool,
}

impl FrameState {
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
        self.scale_changed = true;
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }
}

pub struct Renderer<'a> {
    app: Application<'a>,
    refresh_rate: f64,
    window: Arc<winit::window::Window>,
    window_event_buf: Vec<WindowEvent>,
}

impl Renderer<'_> {
    pub async fn new(
        window: Arc<winit::window::Window>,
        materials: &[Material],
        config: ShaderConfig,
    ) -> Self {
        let refresh_rate = window
            .primary_monitor()
            .map(|monitor| monitor.refresh_rate_millihertz().map(|n| n as f64 / 1000.0))
            .flatten()
            .unwrap_or(60.0);

        Self {
            app: Application::new(window.clone(), materials, config).await,
            refresh_rate,
            window,
            window_event_buf: Vec::new(),
        }
    }

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

        // println!("{:#?}", self.app.bbox_list.objects);
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
