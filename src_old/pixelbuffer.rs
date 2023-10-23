use crate::vector::Vec3;

pub const WIDTH: usize = 160; // Replace with your desired width
pub const HEIGHT: usize = 120; // Replace with your desired height;

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

fn calculate_contrast(rgb1: (u8, u8, u8), rgb2: (u8, u8, u8)) -> f64 {
    let r1 = rgb1.0 as f64 / 255.0;
    let g1 = rgb1.1 as f64 / 255.0;
    let b1 = rgb1.2 as f64 / 255.0;
    let r2 = rgb2.0 as f64 / 255.0;
    let g2 = rgb2.1 as f64 / 255.0;
    let b2 = rgb2.2 as f64 / 255.0;
    let r_contrast = (r1 - r2).abs();
    let g_contrast = (g1 - g2).abs();
    let b_contrast = (b1 - b2).abs();

    f64::max(r_contrast, f64::max(g_contrast, b_contrast))
}

pub fn get_contrast(p: &Box<[u8]>, x1: usize, y1: usize, x2: usize, y2: usize) -> f64 {
    if x1 > WIDTH || x2 > WIDTH {
        return 1.0;
    }
    calculate_contrast(get_pixel(p, x1, y1), get_pixel(p, x2, y2))
}

pub fn get_pixel(p: &Box<[u8]>, x: usize, y: usize) -> (u8, u8, u8) {
    if y > HEIGHT {
        panic!("Tried to get pixel outside bottom screen edge");
    }
    if x > WIDTH {
        panic!("Tried to get pixel outside right screen edge");
    }
    let offset = (y * WIDTH + x) * 4;
    (p[offset], p[offset + 1], p[offset + 2])
}

pub fn set_color(p: &mut Box<[u8]>, x: usize, y: usize, color: &Vec3) {
    set(
        p,
        x,
        y,
        (color.x() * 256.0) as u8,
        (color.y() * 256.0) as u8,
        (color.z() * 256.0) as u8,
    )
}

pub fn get_pixels(p: &Box<[u8]>) -> &[u8] {
    &p
}

pub fn average(
    pix_r1: u16,
    pix_g1: u16,
    pix_b1: u16,
    pix_r2: u16,
    pix_g2: u16,
    pix_b2: u16,
) -> (u8, u8, u8) {
    (
        ((pix_r1 + pix_r2) / 2) as u8,
        ((pix_g1 + pix_g2) / 2) as u8,
        ((pix_b1 + pix_b2) / 2) as u8,
    )
}

pub fn average_tuple(pix1: (u8, u8, u8), pix2: (u8, u8, u8)) -> (u8, u8, u8) {
    average(
        pix1.0 as u16,
        pix1.1 as u16,
        pix1.2 as u16,
        pix2.0 as u16,
        pix2.1 as u16,
        pix2.2 as u16,
    )
}
