use std::sync::Arc;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("My Application")
        .with_inner_size(winit::dpi::PhysicalSize {
            width: 800,
            height: 600,
        })
        .build(&event_loop)
        .unwrap();
    let window = Arc::new(window);
    pollster::block_on(run(window.clone(), event_loop));
}

async fn run(window: Arc<winit::window::Window>, event_loop: EventLoop<()>) {
    let materials = vec![rst_render::Material::Lambertian {
        albedo: [1.0, 1.0, 0.0],
        _padding: Default::default(),
    }];

    let renderer = rst_render::Renderer::new(
        window.clone(),
        &materials,
        rst_render::ShaderConfig {
            samples: 4,
            bounces: 4,
            antialiasing: false,
            motion_blur: true,
            downscale: 4.0,
            fov: 90.0,
            sky_color_top: [1.0, 1.0, 1.0],
            sky_color_bottom: [0.03, 0.16, 0.26],
        },
    )
    .await
    .unwrap();

    renderer.run(event_loop, |events, frame| {
        for event in events {
            match event {
                winit::event::WindowEvent::KeyboardInput { event, .. } => {
                    if let PhysicalKey::Code(KeyCode::Escape) = event.physical_key {
                        frame.exit = true;
                    }
                }
                _ => {}
            }
        }

        // Render resolution will be 4 times smaller
        // than window resolution
        frame.set_scale(4.0);

        frame.is_mouse_locked = false;

        // There are a lot of other fields you can change
        // You don't have to change all of them every frame
        // (only the ones that need to be changed)
        // as they will be remembered.

        frame.camera_pos = [0.5, 0.0, 1.0];
        frame.camera_dir = rst_render::LookDirection::AtPoint(0.5, 0.5, 0.0);

        let object = vec![rst_render::Triangle {
            material: 0, // Index of the material in the materials list
            #[rustfmt::skip]
            geometry: rst_render::Geometry {
                ax: 0.0, ay: 0.0, az: 0.0,
                bx: 1.0, by: 0.0, bz: 0.0,
                cx: 0.5, cy: 1.0, cz: 0.0,
            },
            _padding: Default::default(),
        }];

        let objects = vec![object];

        // Send an object at the end of every frame.
        // You can store it wherever you want, but
        // just return it here every frame.
        // (clone or convert if necessary)
        objects
    });
}
