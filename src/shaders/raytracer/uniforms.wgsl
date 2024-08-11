struct Data {
    width: f32,
    height: f32,
    scale_factor: f32,
    time_elapsed: f32,
    frame_number: u32,
};

@group(0) @binding(0) var output_image: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(1) var<uniform> data: Data;

@group(0) @binding(2) var<uniform> objects_len: u32;
@group(0) @binding(3) var<storage, read> objects: array<Object>;

@group(0) @binding(4) var<uniform> materials_len: u32;
@group(0) @binding(5) var<storage, read> materials: array<Material>;

const pi: f32 = 3.1415926535897932385;