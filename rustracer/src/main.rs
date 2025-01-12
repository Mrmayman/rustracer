use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use rst_render::{Geometry, LookDirection, Material, ShaderConfig, Triangle};
use winit::{event_loop::EventLoop, keyboard::KeyCode, window::WindowBuilder};

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

async fn run(event_loop: EventLoop<()>, window: Arc<winit::window::Window>) {
    println!("Started application");
    let (materials, objects) = read_model();

    let renderer = rst_render::Renderer::new(
        window.clone(),
        &materials,
        ShaderConfig {
            samples: 2,
            bounces: 6,
            antialiasing: true,
            motion_blur: true,
        },
    )
    .await;

    let mut keys_pressed = HashSet::<KeyCode>::new();

    renderer.run(event_loop, |events, frame| {
        for event in events {
            process_event(event, frame, &window, &mut keys_pressed);
        }

        move_camera(frame, &keys_pressed);

        vec![objects.clone()]
    });
}

fn process_event(
    event: &winit::event::WindowEvent,
    frame: &mut rst_render::FrameState,
    window: &winit::window::Window,
    keys_pressed: &mut HashSet<KeyCode>,
) {
    match event {
        winit::event::WindowEvent::CursorMoved { position, .. } => {
            if frame.is_mouse_locked {
                let window_size = window.inner_size();
                let dirx = position.x - window_size.width as f64 / 2.0;
                let diry = position.y - window_size.height as f64 / 2.0;

                let sensitivity = std::cmp::max(window_size.width, window_size.height) as f32;

                if dirx != 0.0 || diry != 0.0 {
                    if let LookDirection::InDirection(x, y) = &mut frame.camera_dir {
                        *x -= diry as f32 / sensitivity;
                        *y += dirx as f32 / sensitivity;

                        *x = (*x).clamp(
                            -std::f32::consts::FRAC_PI_2 * 0.99,
                            std::f32::consts::FRAC_PI_2 * 0.99,
                        );
                    }
                }
            }
        }
        winit::event::WindowEvent::KeyboardInput { event, .. } => {
            if let winit::keyboard::PhysicalKey::Code(code) = event.physical_key {
                if event.state.is_pressed() {
                    keys_pressed.insert(code);
                } else {
                    keys_pressed.remove(&code);
                }
            }
        }
        winit::event::WindowEvent::MouseInput { state, button, .. } => {
            if state.is_pressed() && matches!(button, winit::event::MouseButton::Left) {
                frame.is_mouse_locked = !frame.is_mouse_locked;
            }
        }
        _ => {}
    }
}

fn move_camera(frame: &mut rst_render::FrameState, keys_pressed: &HashSet<KeyCode>) {
    let movement_speed: f32 = frame.delta_time as f32 / 8.0;
    if let LookDirection::InDirection(_, roty) = &frame.camera_dir {
        if keys_pressed.contains(&winit::keyboard::KeyCode::KeyW) {
            frame.camera_pos[0] += movement_speed * roty.cos();
            frame.camera_pos[2] += movement_speed * roty.sin();
        }
        if keys_pressed.contains(&winit::keyboard::KeyCode::KeyS) {
            frame.camera_pos[0] -= movement_speed * roty.cos();
            frame.camera_pos[2] -= movement_speed * roty.sin();
        }
        if keys_pressed.contains(&winit::keyboard::KeyCode::KeyA) {
            frame.camera_pos[0] += movement_speed * roty.sin();
            frame.camera_pos[2] -= movement_speed * roty.cos();
        }
        if keys_pressed.contains(&winit::keyboard::KeyCode::KeyD) {
            frame.camera_pos[0] -= movement_speed * roty.sin();
            frame.camera_pos[2] += movement_speed * roty.cos();
        }
        if keys_pressed.contains(&winit::keyboard::KeyCode::Space) {
            frame.camera_pos[1] += movement_speed;
        }
        if keys_pressed.contains(&winit::keyboard::KeyCode::ShiftLeft) {
            frame.camera_pos[1] -= movement_speed;
        }
    }
}

fn read_model() -> (Vec<Material>, Vec<Triangle>) {
    let mtl = wavefront_obj::mtl::parse(include_str!("../../assets/mazda_rx7_low.mtl")).unwrap();

    let mut materials_map = HashMap::new();
    for (i, mtl) in mtl.materials.iter().enumerate() {
        println!("mtl read: {}", mtl.name);
        materials_map.insert(mtl.name.clone(), (i as u32, mtl));
    }

    let mut objects_list = Vec::new();

    let obj = wavefront_obj::obj::parse(include_str!("../../assets/mazda_rx7_low.obj")).unwrap();
    for obj in obj.objects {
        for geom in &obj.geometry {
            println!("mtl: {:?}", geom.material_name);
            let mtl = materials_map
                .get(geom.material_name.as_ref().unwrap())
                .unwrap()
                .0;
            for shape in &geom.shapes {
                create_triangle(shape, &obj, &mut objects_list, mtl);
            }
        }
    }

    let mut materials = Vec::new();

    for i in 0..materials_map.len() as u32 {
        if let Some((name, (_, mtl))) = materials_map.iter().find(|(_name, (m_i, _m))| *m_i == i) {
            println!("loaded {name}");
            materials.push(Material::Metal {
                albedo: [
                    mtl.color_diffuse.r as f32,
                    mtl.color_diffuse.g as f32,
                    mtl.color_diffuse.b as f32,
                    1.0,
                ],
                fuzziness: 0.3,
                _padding: Default::default(),
            });
        }
    }

    (materials, objects_list)
}

fn create_triangle(
    shape: &wavefront_obj::obj::Shape,
    obj: &wavefront_obj::obj::Object,
    objects_list: &mut Vec<Triangle>,
    mtl: u32,
) {
    match shape.primitive {
        wavefront_obj::obj::Primitive::Point(_) => todo!(),
        wavefront_obj::obj::Primitive::Line(_, _) => todo!(),
        wavefront_obj::obj::Primitive::Triangle((a, _, _), (b, _, _), (c, _, _)) => {
            let a = obj.vertices.get(a).unwrap();
            let b = obj.vertices.get(b).unwrap();
            let c = obj.vertices.get(c).unwrap();
            objects_list.push(Triangle {
                material: mtl,
                geometry: Geometry {
                    ax: a.x as f32,
                    ay: a.y as f32,
                    az: a.z as f32,
                    bx: b.x as f32,
                    by: b.y as f32,
                    bz: b.z as f32,
                    cx: c.x as f32,
                    cy: c.y as f32,
                    cz: c.z as f32,
                },
                _padding: Default::default(),
            });
        }
    }
}
