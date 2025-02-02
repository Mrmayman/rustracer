use std::{sync::Arc, time::Instant};

use wgpu::{util::DeviceExt, PipelineCompilationOptions};

use crate::{
    error::Error,
    objects::{material::Material, ObjectList, Triangle},
    ShaderConfig,
};

use super::{data_buffer::DataBuffer, Application};

impl<'a> Application<'a> {
    pub async fn new(
        window: Arc<winit::window::Window>,
        materials: &[Material],
        shader_config: ShaderConfig,
    ) -> Result<Self, Error> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = instance.create_surface(window)?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(Error::AdapterNotFound)?;
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
            .await?;

        let objects_list: ObjectList<Triangle> = ObjectList::new(&device, "Object".to_owned());

        let bbox_list = ObjectList::new(&device, "Bounding Box".to_owned());

        let materials_list =
            ObjectList::from_vec(&device, materials.to_owned(), "Materials".to_owned());

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities.formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![surface_format],
            desired_maximum_frame_latency: 1,
        };
        surface.configure(&device, &config);

        let mut compute_shader_txt = include_str!("../shaders/raytracer/uniforms.wgsl").to_owned();
        compute_shader_txt.push_str(include_str!("../shaders/data.wgsl"));
        compute_shader_txt.push_str(include_str!("../shaders/raytracer/rng.wgsl"));
        compute_shader_txt.push_str(include_str!("../shaders/raytracer/interval.wgsl"));
        compute_shader_txt.push_str(include_str!("../shaders/raytracer/vec.wgsl"));
        compute_shader_txt.push_str(
            &include_str!("../shaders/compute_shader.wgsl")
                .replace("CONF_SAMPLES", &shader_config.samples.to_string())
                .replace("CONF_BOUNCES", &shader_config.bounces.to_string())
                .replace(
                    "CONF_ANTIALIASING",
                    if shader_config.antialiasing { "1" } else { "0" },
                )
                .replace(
                    "CONF_MOTION_BLUR",
                    if shader_config.motion_blur { "1" } else { "0" },
                ),
        );

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
                    wgpu::BindGroupLayoutEntry {
                        binding: 7,
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
                        binding: 8,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(0 as wgpu::BufferAddress),
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
            compilation_options: PipelineCompilationOptions {
                zero_initialize_workgroup_memory: false,
                ..Default::default()
            },
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
                width: (config.width as f32 / shader_config.downscale) as u32,
                height: (config.height as f32 / shader_config.downscale) as u32,
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
            scale_factor: shader_config.downscale,
            time_elapsed: start_time.elapsed().as_secs_f32(),
            frame_number: 0,
            camx: 0.0,
            camy: 0.0,
            camz: 0.0,
            lookx: 0.0,
            looky: 0.0,
            lookz: 0.0,
            fov: shader_config.fov,
            sky_color_top: shader_config.sky_color_top,
            sky_color_bottom: shader_config.sky_color_bottom,
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
                wgpu::BindGroupEntry {
                    binding: 7,
                    resource: bbox_list.object_len.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 8,
                    resource: bbox_list.object_buffer.as_entire_binding(),
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

        Ok(Self {
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
            scale_factor: shader_config.downscale,
            texture_view,
            bbox_list,
            compute_bind_group_layout,
            texture_bind_group_layout,
            last_frame_time: Instant::now(),
            start_time,
            data_buffer,
            objects_list,
            materials_list,
            camera_pos: [0.0, 0.0, 0.0],
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
        })
    }
}
