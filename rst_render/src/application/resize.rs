use winit::dpi::PhysicalSize;

use super::Application;

impl<'a> Application<'a> {
    pub fn resize_window(&mut self, new_size: PhysicalSize<u32>) {
        puffin::profile_function!();
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        self.surface.configure(&self.device, &self.surface_config);

        self.rescale_window();
    }

    pub fn rescale_window(&mut self) {
        puffin::profile_function!();
        let scaled_width = (self.surface_config.width as f32 / self.scale_factor) as u32;
        let scaled_height = (self.surface_config.height as f32 / self.scale_factor) as u32;

        if scaled_height != 0 && scaled_width != 0 {
            self.texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Output Texture"),
                size: wgpu::Extent3d {
                    width: scaled_width,
                    height: scaled_height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
            });

            self.previous_texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Previous Output Texture"),
                size: wgpu::Extent3d {
                    width: scaled_width,
                    height: scaled_height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
            });

            self.denoise_texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Denoise Texture"),
                size: wgpu::Extent3d {
                    width: scaled_width,
                    height: scaled_height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
            });
        }

        self.texture_view = self
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.previous_texture_view = self
            .previous_texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.denoise_texture_view = self
            .denoise_texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.update_compute_bind_group();

        self.texture_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.denoise_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.denoise_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &self.data_buffer_object,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&self.previous_texture_view),
                },
            ],
        });

        self.denoise_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Denoise Bind Group"),
            layout: &self.denoise_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.denoise_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: self.data_buffer_object.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&self.texture_view),
                },
            ],
        });

        self.data_buffer.width = self.surface_config.width as f32;
        self.data_buffer.height = self.surface_config.height as f32;
        self.update_data_buffer();
    }

    pub fn update_compute_bind_group(&mut self) {
        self.compute_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Compute Bind Group"),
            layout: &self.compute_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: self.data_buffer_object.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: self.objects_list.object_len.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: self.objects_list.object_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: self.materials_list.object_len.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: self.materials_list.object_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: wgpu::BindingResource::TextureView(&self.previous_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 7,
                    resource: self.bbox_list.object_len.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 8,
                    resource: self.bbox_list.object_buffer.as_entire_binding(),
                },
            ],
        });
    }

    pub fn update_data_buffer(&mut self) {
        puffin::profile_function!();

        self.data_buffer.time_elapsed = self.start_time.elapsed().as_secs_f32();
        self.data_buffer.frame_number += 1;

        self.queue.write_buffer(
            &self.data_buffer_object,
            0,
            bytemuck::cast_slice(&[self.data_buffer]),
        );
    }

    pub fn update_camera(&mut self) {
        puffin::profile_function!();
        self.data_buffer.camx = self.camera_pos[0];
        self.data_buffer.camy = self.camera_pos[1];
        self.data_buffer.camz = self.camera_pos[2];

        match self.camera_dir {
            crate::application::LookDirection::AtPoint(x, y, z) => {
                self.data_buffer.lookx = x;
                self.data_buffer.looky = y;
                self.data_buffer.lookz = z;
            }
            crate::application::LookDirection::InDirection(rotx, roty) => {
                self.data_buffer.lookx = self.camera_pos[0] + roty.cos() * rotx.cos();
                self.data_buffer.looky = self.camera_pos[1] + rotx.sin();
                self.data_buffer.lookz = self.camera_pos[2] + roty.sin() * rotx.cos();
            }
        }
    }
}
