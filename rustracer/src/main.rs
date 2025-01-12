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
            samples: 4,
            bounces: 4,
            antialiasing: false,
            motion_blur: true,
            downscale: 4.0,
        },
    )
    .await
    .unwrap();

    let mut keys_pressed = HashSet::<KeyCode>::new();

    renderer.run(event_loop, |events, frame| {
        for event in events {
            process_event(event, frame, &window, &mut keys_pressed);
        }

        move_camera(frame, &keys_pressed);

        objects.clone()
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

                let sensitivity = std::cmp::max(window_size.width, window_size.height) as f32
                    * frame.delta_time as f32
                    * 8.0;

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
    println!("{:?}, {:?}", frame.camera_pos, frame.camera_dir);

    // frame.camera_pos = [-1.9376986, 1.7867299, 4.8897142];
    // frame.camera_dir = LookDirection::InDirection(-0.569412, -0.709585);

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

fn read_model() -> (Vec<Material>, Vec<Vec<Triangle>>) {
    let mut materials_map = HashMap::new();

    let mut mtl_i = 0;
    let mtl = wavefront_obj::mtl::parse(include_str!("../../assets/mazda_rx7_low.mtl")).unwrap();
    for mtl in &mtl.materials {
        materials_map.insert(mtl.name.clone(), (mtl_i as u32, mtl));
        mtl_i += 1;
    }
    let mtl = wavefront_obj::mtl::parse(include_str!("../../assets/roadbasic.mtl")).unwrap();
    for mtl in &mtl.materials {
        materials_map.insert(mtl.name.clone(), (mtl_i as u32, mtl));
        mtl_i += 1;
    }

    let mut objects_list = Vec::new();

    add_obj(
        include_str!("../../assets/mazda_rx7_low.obj"),
        &materials_map,
        &mut objects_list,
    );
    add_obj(
        include_str!("../../assets/roadbasic.obj"),
        &materials_map,
        &mut objects_list,
    );
    let mut materials = Vec::new();

    for i in 0..materials_map.len() as u32 {
        if let Some((_, (_, mtl))) = materials_map.iter().find(|(_name, (m_i, _m))| *m_i == i) {
            if let Some(emissive) = mtl.color_emissive {
                if emissive.r > 0.0 || emissive.g > 0.0 || emissive.b > 0.0 {
                    materials.push(Material::Emissive {
                        color: [emissive.r as f32, emissive.g as f32, emissive.b as f32],
                        _padding: Default::default(),
                    });
                    continue;
                }
            }

            if mtl.alpha < 1.0 {
                materials.push(Material::Dielectric {
                    refraction_index: 1.05,
                    tint: [
                        mtl.color_diffuse.r as f32,
                        mtl.color_diffuse.g as f32,
                        mtl.color_diffuse.b as f32,
                    ],
                    tint_alpha: mtl.alpha as f32,
                    _padding: Default::default(),
                });
                continue;
            }

            materials.push(Material::Metal {
                albedo: [
                    mtl.color_diffuse.r as f32,
                    mtl.color_diffuse.g as f32,
                    mtl.color_diffuse.b as f32,
                ],
                fuzziness: mtl.specular_coefficient as f32 / 1000.0,
                _padding: Default::default(),
            });
        }
    }

    (materials, objects_list)
}

fn add_obj(
    input: &str,
    materials_map: &HashMap<String, (u32, &wavefront_obj::mtl::Material)>,
    objects_list: &mut Vec<Vec<Triangle>>,
) {
    let obj = wavefront_obj::obj::parse(input).unwrap();
    for obj in obj.objects {
        // println!("object: {}", obj.name);
        let mut list = Vec::new();
        for geom in &obj.geometry {
            let mtl = materials_map
                .get(&geom.material_name.clone().unwrap_or_default())
                .map(|n| n.0)
                .unwrap_or(0);
            for shape in &geom.shapes {
                create_triangle(shape, &obj, &mut list, mtl);
            }
            if list.len() > 10 {
                // println!("list: {}", list.len());
                objects_list.push(list.clone());
                list.clear();
            }
        }

        if !list.is_empty() {
            objects_list.push(list);
        }
    }
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
