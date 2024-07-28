use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use application::Application;
use objects::Object;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod application;
mod objects;

async fn run(event_loop: EventLoop<()>, window: Arc<winit::window::Window>) {
    let mut app = Application::new(window.clone()).await;

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
                        app.tick();

                        let time_elapsed = app.last_frame_time.elapsed().as_secs_f64();
                        let remaining_time = (1.0 / 60.0) - time_elapsed;
                        println!("sleeping {remaining_time}");
                        if remaining_time > 0.0 {
                            std::thread::sleep(Duration::from_secs_f64(remaining_time));
                        }

                        app.last_frame_time = Instant::now();

                        app.data_buffer.time_elapsed = app.start_time.elapsed().as_secs_f32();
                        app.update_data();
                        window.request_redraw();
                    }
                    _ => {}
                },
                _ => {}
            }
        })
        .unwrap();
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Some GPU application")
        .build(&event_loop)
        .unwrap();

    pollster::block_on(run(event_loop, Arc::new(window)));
}
