use crate::vector::Vec3;

pub const WIDTH: usize = 200; // Replace with your desired width
pub const HEIGHT: usize = 150; // Replace with your desired height;

pub fn new() -> Box<[u8]> {
    let size = WIDTH * HEIGHT * 4;
    vec![0; size].into_boxed_slice()
}

pub fn set(p: &mut Box<[u8]>, x: usize, y: usize, r: u8, g: u8, b: u8) {
    let offset = (y * WIDTH + x) * 4;
    p[offset] = r;
    p[offset + 1] = g;
    p[offset + 2] = b;
    p[offset + 3] = 255; // Assuming alpha = opaque
}

pub fn set_color(p: &mut Box<[u8]>, x: usize, y: usize, color: &Vec3) {
    set(p, x, y, (color.x() * 256.0) as u8, (color.y() * 256.0) as u8, (color.z() * 256.0) as u8)
}

pub fn get_pixels(p: &Box<[u8]>) -> &[u8] {
    &p
}