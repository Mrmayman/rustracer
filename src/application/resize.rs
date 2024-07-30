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
        }

        self.texture_view = self
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

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
            ],
        });

        self.texture_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &self.data_buffer_object,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        });

        self.data_buffer.width = self.surface_config.width as f32;
        self.data_buffer.height = self.surface_config.height as f32;
        self.update_data();
    }

    pub fn update_data(&mut self) {
        puffin::profile_function!();
        self.queue.write_buffer(
            &self.data_buffer_object,
            0,
            bytemuck::cast_slice(&[self.data_buffer]),
        );
    }
}
