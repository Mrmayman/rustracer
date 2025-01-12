use wgpu::{util::DeviceExt, Buffer};

pub mod material;

#[repr(C)]
#[derive(Default, Debug)]
pub struct BoundingBox {
    pub start_idx: u32,
    pub end_idx: u32,
    pub minx: f32,
    pub miny: f32,
    pub minz: f32,
    pub maxx: f32,
    pub maxy: f32,
    pub maxz: f32,
}

pub fn min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

pub fn max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

fn min3(a: f32, b: f32, c: f32) -> f32 {
    if a < b {
        if a < c {
            a
        } else {
            c
        }
    } else {
        if b < c {
            b
        } else {
            c
        }
    }
}

fn max3(a: f32, b: f32, c: f32) -> f32 {
    if a > b {
        if a > c {
            a
        } else {
            c
        }
    } else {
        if b > c {
            b
        } else {
            c
        }
    }
}

impl BoundingBox {
    pub fn contains(&self, x: f32, y: f32, z: f32) -> bool {
        (x > self.minx && x < self.maxx)
            && (y > self.miny && y < self.maxy)
            && (z > self.minz && z < self.maxz)
    }

    pub fn wrap(tri: &Triangle) -> Self {
        let ax = tri.geometry.ax;
        let ay = tri.geometry.ay;
        let az = tri.geometry.az;

        let bx = tri.geometry.bx;
        let by = tri.geometry.by;
        let bz = tri.geometry.bz;

        let cx = tri.geometry.cx;
        let cy = tri.geometry.cy;
        let cz = tri.geometry.cz;

        let minx = min3(ax, bx, cx);
        let miny = min3(ay, by, cy);
        let minz = min3(az, bz, cz);

        let maxx = max3(ax, bx, cx);
        let maxy = max3(ay, by, cy);
        let maxz = max3(az, bz, cz);

        Self {
            start_idx: 0,
            end_idx: 0,
            minx,
            miny,
            minz,
            maxx,
            maxy,
            maxz,
        }
    }

    pub fn expand_to(&mut self, x: f32, y: f32, z: f32) {
        if self.contains(x, y, z) {
            return;
        }

        self.maxx = max(self.maxx, x);
        self.maxy = max(self.maxy, y);
        self.maxz = max(self.maxz, z);

        self.minx = min(self.minx, x);
        self.miny = min(self.miny, y);
        self.minz = min(self.minz, z);
    }

    pub fn expand_to_tri(&mut self, tri: &Triangle) {
        self.expand_to(tri.geometry.ax, tri.geometry.ay, tri.geometry.az);
        self.expand_to(tri.geometry.bx, tri.geometry.by, tri.geometry.bz);
        self.expand_to(tri.geometry.cx, tri.geometry.cy, tri.geometry.cz);
    }

    pub fn fix(&mut self) {
        if self.minx == self.maxx {
            self.minx -= 0.1;
            self.maxx += 0.1;
        }
        if self.miny == self.maxy {
            self.miny -= 0.1;
            self.maxy += 0.1;
        }
        if self.minz == self.maxz {
            self.minz -= 0.1;
            self.maxz += 0.1;
        }
    }
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct Triangle {
    pub material: u32,
    pub geometry: Geometry,
    pub _padding: [u32; 16 - (1 + (std::mem::size_of::<Geometry>() / 4))],
}

impl std::fmt::Debug for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Triangle")
            .field("material", &self.material)
            .field("geometry", &self.geometry)
            .finish()
    }
}

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
#[derive(Default, Clone)]
pub struct Geometry {
    pub ax: f32,
    pub ay: f32,
    pub az: f32,
    pub bx: f32,
    pub by: f32,
    pub bz: f32,
    pub cx: f32,
    pub cy: f32,
    pub cz: f32,
}

impl std::fmt::Debug for Geometry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Geometry")
            .field("a", &format!("({}, {}, {})", self.ax, self.ay, self.az))
            .field("b", &format!("({}, {}, {})", self.bx, self.by, self.bz))
            .field("c", &format!("({}, {}, {})", self.cx, self.cy, self.cz))
            .finish()
    }
}

pub struct ObjectList<O: Sized + Default> {
    pub objects: Vec<O>,
    pub object_buffer: Buffer,
    pub object_len: Buffer,
    pub name: String,
    pub existing_len: usize,
}

impl<O: Default> ObjectList<O> {
    pub fn new(device: &wgpu::Device, name: String) -> Self {
        let objects = vec![O::default()];
        let object_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Buffer", name)),
            contents: to_bytes_unsized(
                objects.as_slice(),
                objects.len() * std::mem::size_of::<O>(),
            ),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let object_len = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Length", name)),
            contents: to_bytes(&(objects.len() as u32)),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let existing_len = objects.len();
        Self {
            objects,
            object_buffer,
            object_len,
            name,
            existing_len,
        }
    }

    pub fn from_vec(device: &wgpu::Device, objects: Vec<O>, name: String) -> Self {
        let object_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Buffer", name)),
            contents: to_bytes_unsized(
                objects.as_slice(),
                objects.len() * std::mem::size_of::<O>(),
            ),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let object_len = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Length", name)),
            contents: to_bytes(&(objects.len() as u32)),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let existing_len = objects.len();
        Self {
            objects,
            object_buffer,
            object_len,
            name,
            existing_len,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, device: &wgpu::Device) -> bool {
        let has_changed = self.objects.len() != self.existing_len;
        if has_changed {
            self.object_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{} Buffer", self.name)),
                contents: to_bytes_unsized(
                    self.objects.as_slice(),
                    self.objects.len() * std::mem::size_of::<O>(),
                ),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            });
        } else {
            queue.write_buffer(
                &self.object_buffer,
                0,
                to_bytes_unsized(
                    self.objects.as_slice(),
                    self.objects.len() * std::mem::size_of::<O>(),
                ),
            );
        }

        self.existing_len = self.objects.len();
        queue.write_buffer(&self.object_len, 0, to_bytes(&(self.objects.len() as u32)));

        has_changed
    }
}
