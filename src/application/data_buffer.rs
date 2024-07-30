#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DataBuffer {
    pub width: f32,
    pub height: f32,
    pub scale_factor: f32,
    pub time_elapsed: f32,
    pub frame_number: u32,
}
