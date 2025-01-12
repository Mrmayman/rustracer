use super::Application;

pub const WORKGROUP_SIZE: f32 = 8.0;

impl<'a> Application<'a> {
    pub fn render(&mut self) {
        puffin::profile_function!();
        self.run_compute_shader();
        self.copy_texture_to_screen();
    }

    fn copy_texture_to_screen(&mut self) {
        puffin::profile_function!();

        let (frame, mut encoder) = {
            puffin::profile_scope!("create frame, encoder");

            let frame = {
                puffin::profile_scope!("create frame");
                self.surface
                    .get_current_texture()
                    .expect("Failed to acquire next swap chain texture")
            };

            // Create a command encoder for the graphics pass
            let encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
            (frame, encoder)
        };

        {
            puffin::profile_scope!("render pass");
            let view = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
            render_pass.draw(0..6, 0..1);
        }

        // Submit the command buffer
        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    fn run_compute_shader(&mut self) {
        puffin::profile_function!();
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Encoder"),
            });

        {
            puffin::profile_scope!("compute pass");
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute Pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &self.compute_bind_group, &[]);

            compute_pass.dispatch_workgroups(
                (self.surface_config.width as f32 / (self.scale_factor * WORKGROUP_SIZE)).ceil()
                    as u32,
                (self.surface_config.height as f32 / (self.scale_factor * WORKGROUP_SIZE)).ceil()
                    as u32,
                1,
            );
        }

        {
            puffin::profile_scope!("denoise pass");
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Denoise Pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.denoise_pipeline);
            compute_pass.set_bind_group(0, &self.denoise_bind_group, &[]);

            compute_pass.dispatch_workgroups(
                (self.surface_config.width as f32 / (self.scale_factor * WORKGROUP_SIZE)).ceil()
                    as u32,
                (self.surface_config.height as f32 / (self.scale_factor * WORKGROUP_SIZE)).ceil()
                    as u32,
                1,
            );
        }

        // Submit the command buffer
        self.queue.submit(Some(encoder.finish()));
    }
}
