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
pub use update::WORKGROUP_SIZE;

use crate::objects::{material::Material, BoundingBox, ObjectList, Triangle};

#[derive(Clone)]
pub enum LookDirection {
    AtPoint(f32, f32, f32),
    InDirection(f32, f32),
}

pub struct Application<'a> {
    pub surface: Surface<'a>,
    pub surface_config: SurfaceConfiguration,
    pub device: Device,
    pub data_buffer_object: Buffer,
    pub data_buffer: DataBuffer,

    pub compute_bind_group: BindGroup,
    pub compute_bind_group_layout: BindGroupLayout,
    pub compute_pipeline: ComputePipeline,

    pub texture_bind_group: BindGroup,
    pub texture_bind_group_layout: BindGroupLayout,
    pub render_pipeline: RenderPipeline,

    pub denoise_bind_group: BindGroup,
    pub denoise_bind_group_layout: BindGroupLayout,
    pub denoise_pipeline: ComputePipeline,

    pub queue: Queue,

    pub denoise_texture: Texture,
    pub denoise_texture_view: TextureView,
    pub denoise_sampler: Sampler,

    pub texture: Texture,
    pub texture_view: TextureView,

    pub previous_texture: Texture,
    pub previous_texture_view: TextureView,

    pub last_frame_time: Instant,
    pub start_time: Instant,
    pub time_elapsed: f64,

    pub objects_list: ObjectList<Triangle>,
    pub materials_list: ObjectList<Material>,
    pub bbox_list: ObjectList<BoundingBox>,

    pub camera_pos: [f32; 3],
    pub camera_dir: LookDirection,
    pub scale_factor: f32,
    pub is_mouse_locked: bool,
}
