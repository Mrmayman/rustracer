use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::Instant,
};

use wgpu::{util::DeviceExt, PipelineCompilationOptions};

use crate::{
    objects::{material::Material, Geometry, Object, ObjectList},
    SCALE_FACTOR,
};

use super::{data_buffer::DataBuffer, Application};

impl<'a> Application<'a> {
    pub async fn new(window: Arc<winit::window::Window>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = instance.create_surface(window).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .unwrap();

        let mut objects_list = vec![
            // Object {
            //     geometry: Geometry::Sphere {
            //         centre_x: -0.5,
            //         centre_y: 0.0,
            //         centre_z: -1.0,
            //         radius: 0.5,
            //         _padding: Default::default(),
            //     },
            //     material: 1,
            // },
            // Object {
            //     geometry: Geometry::Sphere {
            //         centre_x: 0.5,
            //         centre_y: 0.0,
            //         centre_z: -1.0,
            //         radius: 0.5,
            //         _padding: Default::default(),
            //     },
            //     material: 2,
            // },
            Object {
                geometry: Geometry::Sphere {
                    centre_x: 0.0,
                    centre_y: -100.5,
                    centre_z: -1.0,
                    radius: 100.0,
                    _padding: Default::default(),
                },
                material: 0,
            },
        ];

        if false {
            for i in 0..100 {
                objects_list.push(Object {
                    geometry: Geometry::Sphere {
                        centre_x: (i % 10) as f32,
                        centre_y: 0.0,
                        centre_z: -1.0 - (i / 10) as f32,
                        radius: 0.4,
                        _padding: Default::default(),
                    },
                    material: if i % 2 == 0 { 1 } else { 3 },
                });
                objects_list.push(Object {
                    geometry: Geometry::Triangle {
                        ax: (i % 10) as f32,
                        ay: 2.0,
                        az: (i / 10) as f32 + 1.0,
                        bx: (i % 10) as f32 + 1.0,
                        by: 2.0,
                        bz: (i / 10) as f32,
                        cx: (i % 10) as f32,
                        cy: 2.0,
                        cz: (i / 10) as f32,
                        _padding: Default::default(),
                    },
                    material: if i % 2 == 0 { 1 } else { 3 },
                });
            }
        }

        let mut num_tris = 0;

        let mtl =
            wavefront_obj::mtl::parse(include_str!("../../assets/mazda_rx7_low.mtl")).unwrap();

        let mut materials_map = HashMap::new();
        let mut mtl_i = 1;
        for mtl in mtl.materials {
            println!("mtl read: {}", mtl.name);
            materials_map.insert(mtl.name.clone(), (mtl_i, mtl));
            mtl_i += 1;
        }

        let obj =
            wavefront_obj::obj::parse(include_str!("../../assets/mazda_rx7_low.obj")).unwrap();
        for obj in obj.objects {
            for geom in obj.geometry {
                println!("mtl: {:?}", geom.material_name);
                let mtl = if let Some(mtl) = &geom.material_name {
                    if let Some((mtl_i, _mtl)) = materials_map.get(mtl) {
                        *mtl_i
                    } else {
                        0
                    }
                } else {
                    0
                };
                for shape in geom.shapes {
                    // if num_tris > 300 {
                    //     break;
                    // }
                    match shape.primitive {
                        wavefront_obj::obj::Primitive::Point(_) => todo!(),
                        wavefront_obj::obj::Primitive::Line(_, _) => todo!(),
                        wavefront_obj::obj::Primitive::Triangle(
                            (a, _, _),
                            (b, _, _),
                            (c, _, _),
                        ) => {
                            let a = obj.vertices.get(a).unwrap();
                            let b = obj.vertices.get(b).unwrap();
                            let c = obj.vertices.get(c).unwrap();
                            num_tris += 1;
                            if true {
                                objects_list.push(Object {
                                    material: mtl,
                                    geometry: Geometry::Triangle {
                                        ax: a.x as f32,
                                        ay: a.y as f32,
                                        az: a.z as f32,
                                        bx: b.x as f32,
                                        by: b.y as f32,
                                        bz: b.z as f32,
                                        cx: c.x as f32,
                                        cy: c.y as f32,
                                        cz: c.z as f32,
                                        _padding: Default::default(),
                                    },
                                });
                            }
                        }
                    }
                }
            }
        }

        println!("Num tris: {num_tris}");
        // std::process::exit(0);

        let objects_list = ObjectList::from_vec(&device, objects_list, "Object".to_owned());

        let mut materials = vec![Material::Lambertian {
            albedo: [1.0, 0.5, 0.25, 1.0],
            _padding: Default::default(),
        }];

        for i in 1..mtl_i {
            if let Some((_name, (_mtl_i, mtl))) =
                materials_map.iter().find(|(_name, (m_i, _m))| *m_i == i)
            {
                materials.push(Material::Lambertian {
                    albedo: [
                        mtl.color_diffuse.r as f32,
                        mtl.color_diffuse.g as f32,
                        mtl.color_diffuse.b as f32,
                        1.0,
                    ],
                    _padding: Default::default(),
                });
            }
        }

        let materials_list = ObjectList::from_vec(&device, materials, "Materials".to_owned());

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities.formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![surface_format],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let mut compute_shader_txt = include_str!("../shaders/raytracer/uniforms.wgsl").to_owned();
        compute_shader_txt.push_str(include_str!("../shaders/data.wgsl"));
        compute_shader_txt.push_str(include_str!("../shaders/raytracer/rng.wgsl"));
        compute_shader_txt.push_str(include_str!("../shaders/raytracer/interval.wgsl"));
        compute_shader_txt.push_str(include_str!("../shaders/raytracer/vec.wgsl"));
        compute_shader_txt.push_str(include_str!("../shaders/compute_shader.wgsl"));

        let compute_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(compute_shader_txt.into()),
        });

        let vertex_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/vertex_shader.wgsl").into()),
        });

        let mut denoise_shader_txt = include_str!("../shaders/data.wgsl").to_owned();
        denoise_shader_txt.push_str(include_str!(
            "../shaders/glsl_smart_denoise/glsl_smart_denoise.wgsl"
        ));
        denoise_shader_txt.push_str(include_str!("../shaders/denoise_shader.wgsl"));

        let denoise_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Denoise Shader"),
            source: wgpu::ShaderSource::Wgsl(denoise_shader_txt.into()),
        });

        let fragment_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Fragment Shader"),
            source: wgpu::ShaderSource::Wgsl({
                let mut frag = include_str!("../shaders/fsr/fsr.wgsl").to_owned();
                frag.push_str(include_str!("../shaders/data.wgsl"));
                frag.push_str(include_str!("../shaders/fragment_shader.wgsl"));
                frag.into()
            }),
        });

        let denoise_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Denoise Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture {
                            access: wgpu::StorageTextureAccess::WriteOnly,
                            format: wgpu::TextureFormat::Rgba8Unorm,
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(
                                std::mem::size_of::<DataBuffer>() as wgpu::BufferAddress,
                            ),
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                ],
            });

        let denoise_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Denoise Pipeline Layout"),
                bind_group_layouts: &[&denoise_bind_group_layout],
                push_constant_ranges: &[],
            });

        let denoise_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Denoise Pipeline"),
            layout: Some(&denoise_pipeline_layout),
            module: &denoise_shader,
            entry_point: "main",
            compilation_options: PipelineCompilationOptions::default(),
            cache: None,
        });

        let compute_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Compute Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture {
                            access: wgpu::StorageTextureAccess::WriteOnly,
                            format: wgpu::TextureFormat::Rgba8Unorm,
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(
                                std::mem::size_of::<DataBuffer>() as wgpu::BufferAddress,
                            ),
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(
                                std::mem::size_of::<u32>() as wgpu::BufferAddress
                            ),
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(0 as wgpu::BufferAddress),
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 4,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(
                                std::mem::size_of::<u32>() as wgpu::BufferAddress
                            ),
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 5,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(0 as wgpu::BufferAddress),
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 6,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                ],
            });

        let compute_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Compute Pipeline Layout"),
                bind_group_layouts: &[&compute_bind_group_layout],
                push_constant_ranges: &[],
            });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader,
            entry_point: "main",
            compilation_options: PipelineCompilationOptions::default(),
            cache: None,
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(
                                std::mem::size_of::<DataBuffer>() as wgpu::BufferAddress,
                            ),
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::StorageTexture {
                            access: wgpu::StorageTextureAccess::WriteOnly,
                            format: wgpu::TextureFormat::Rgba8Unorm,
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                ],
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: "main",
                buffers: &[],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader,
                entry_point: "main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Output Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let denoise_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Denoise Texture"),
            size: wgpu::Extent3d {
                width: (config.width as f32 / SCALE_FACTOR) as u32,
                height: (config.height as f32 / SCALE_FACTOR) as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
        });

        let denoise_texture_view =
            denoise_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let denoise_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Denoise Texture Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let previous_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Previous Output Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
        });

        let previous_texture_view =
            previous_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let start_time = Instant::now();

        let data_buffer = DataBuffer {
            width: config.width as f32,
            height: config.height as f32,
            scale_factor: SCALE_FACTOR,
            time_elapsed: start_time.elapsed().as_secs_f32(),
            frame_number: 0,
            camx: 0.0,
            camy: 0.0,
            camz: 0.0,
            lookx: 0.0,
            looky: 0.0,
            lookz: 0.0,
        };

        let data_buffer_object = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Window Size Buffer"),
            contents: bytemuck::cast_slice(&[data_buffer]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let denoise_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Denoise Bind Group"),
            layout: &denoise_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&denoise_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: data_buffer_object.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
            ],
        });

        let compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Compute Bind Group"),
            layout: &compute_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: data_buffer_object.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: objects_list.object_len.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: objects_list.object_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: materials_list.object_len.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: materials_list.object_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: wgpu::BindingResource::TextureView(&previous_texture_view),
                },
            ],
        });

        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&denoise_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&denoise_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &data_buffer_object,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&previous_texture_view),
                },
            ],
        });

        Self {
            surface,
            surface_config: config,
            device,
            data_buffer_object,
            texture_bind_group,
            compute_bind_group,
            render_pipeline,
            compute_pipeline,
            queue,
            texture,
            scale_factor: SCALE_FACTOR,
            texture_view,
            compute_bind_group_layout,
            texture_bind_group_layout,
            last_frame_time: Instant::now(),
            start_time,
            data_buffer,
            objects_list,
            materials_list,
            camera_pos: [0.0, 0.0, 0.0],
            keys_pressed: HashSet::new(),
            camera_dir: super::LookDirection::InDirection(0.0, 0.0),
            is_mouse_locked: true,
            time_elapsed: 1.0,
            previous_texture_view,
            previous_texture,
            denoise_bind_group,
            denoise_bind_group_layout,
            denoise_pipeline,
            denoise_texture,
            denoise_texture_view,
            denoise_sampler,
        }
    }
}
