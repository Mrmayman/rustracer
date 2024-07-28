use wgpu::{util::DeviceExt, Buffer};

#[repr(C)]
pub struct Object {
    pub geometry: Geometry,
}

// impl Object {
//     pub fn debug_print() {
//         let object = Object {
//             geometry: Geometry::Sphere {
//                 centre_x: 1.0,
//                 centre_y: 2.0,
//                 centre_z: 3.0,
//                 radius: 4.0,
//             },
//         };

//         let bytes: &[u8] = struct_to_bytes(&object);

//         print!("bytes: ");
//         for byte in bytes {
//             print!("{byte:X}, ");
//         }
//         println!()
//     }
// }

pub fn to_bytes<T: Sized>(s: &T) -> &[u8] {
    let ptr = s as *const T as *const u8;
    let size = std::mem::size_of::<T>();
    unsafe { std::slice::from_raw_parts(ptr, size) }
}

pub fn to_bytes_unsized<T: ?Sized>(s: &T, size: usize) -> &[u8] {
    let ptr = s as *const T as *const u8;
    unsafe { std::slice::from_raw_parts(ptr, size) }
}

#[repr(C)]
pub enum Geometry {
    Sphere {
        centre_x: f32,
        centre_y: f32,
        centre_z: f32,
        radius: f32,
        _1: f32,
        _2: f32,
        _3: f32,
    },
}

pub struct ObjectList {
    pub objects: Vec<Object>,
    pub object_buffer: Buffer,
    pub object_len: Buffer,
}

impl ObjectList {
    pub fn new(device: &wgpu::Device) -> Self {
        let objects = vec![];
        let object_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Objects Buffer"),
            contents: to_bytes_unsized(
                objects.as_slice(),
                objects.len() * std::mem::size_of::<Object>(),
            ),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let object_len = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Objects Buffer"),
            contents: to_bytes(&(objects.len() as u32)),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            objects,
            object_buffer,
            object_len,
        }
    }

    pub fn from_vec(device: &wgpu::Device, objects: Vec<Object>) -> Self {
        let object_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Objects Buffer"),
            contents: to_bytes_unsized(
                objects.as_slice(),
                objects.len() * std::mem::size_of::<Object>(),
            ),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let object_len = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Objects Buffer"),
            contents: to_bytes(&(objects.len() as u32)),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            objects,
            object_buffer,
            object_len,
        }
    }

    pub fn push(&mut self, queue: &wgpu::Queue, object: Object) {
        self.objects.push(object);
        self.update(queue);
    }

    pub fn update(&self, queue: &wgpu::Queue) {
        queue.write_buffer(
            &self.object_buffer,
            0,
            to_bytes_unsized(
                self.objects.as_slice(),
                self.objects.len() * std::mem::size_of::<Object>(),
            ),
        );

        queue.write_buffer(&self.object_len, 0, to_bytes(&(self.objects.len() as u32)));
    }
}