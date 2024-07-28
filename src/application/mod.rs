use std::time::Instant;

use wgpu::{
    BindGroup, BindGroupLayout, Buffer, ComputePipeline, Device, Queue, RenderPipeline, Sampler,
    Surface, SurfaceConfiguration, Texture, TextureView,
};

mod data_buffer;
mod init;
mod resize;
mod update;

pub use data_buffer::DataBuffer;

use crate::objects::ObjectList;

pub struct Application<'a> {
    pub surface: Surface<'a>,
    pub surface_config: SurfaceConfiguration,
    pub device: Device,
    pub data_buffer_object: Buffer,
    pub data_buffer: DataBuffer,
    pub texture_bind_group: BindGroup,
    pub compute_bind_group: BindGroup,
    pub texture_bind_group_layout: BindGroupLayout,
    pub compute_bind_group_layout: BindGroupLayout,
    pub render_pipeline: RenderPipeline,
    pub compute_pipeline: ComputePipeline,
    pub queue: Queue,
    pub texture: Texture,
    pub scale_factor: f32,
    pub texture_view: TextureView,
    pub sampler: Sampler,
    pub last_frame_time: Instant,
    pub start_time: Instant,

    pub objects_list: ObjectList,
}
