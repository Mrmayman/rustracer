use crate::vector::Vec3;
use image::GenericImageView;
use std::path::Path;
use std::sync::Arc;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
    fn clone_custom(&self) -> Box<dyn Texture>;
}

pub struct SolidColor {
    color_value: Vec3,
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.color_value
    }

    fn clone_custom(&self) -> Box<dyn Texture> {
        Box::new(SolidColor::new(&self.color_value))
    }
}

impl SolidColor {
    pub fn new(color: &Vec3) -> SolidColor {
        SolidColor {
            color_value: *color,
        }
    }
}

pub struct CheckerTexture {
    scale: f64,
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        // let x = (self.scale * p.x()).floor() as i64;
        // let y = (self.scale * p.y()).floor() as i64;
        // let z = (self.scale * p.z()).floor() as i64;

        // let is_even: bool = (x + y + z) % 2 == 0;

        // let is_even: bool = (((u * 100.0) + (v * 100.0)) % 2.0) as i64 == 0;
        // println!("{}, {}", u*100.0, v*100.0);

        let is_even: bool = ((u * 100.0) % 2.0) as i64 == ((v * 100.0) % 2.0) as i64;

        if is_even {
            return self.even.value(u, v, p);
        } else {
            return self.odd.value(u, v, p);
        }
    }

    fn clone_custom(&self) -> Box<dyn Texture> {
        Box::new(CheckerTexture::new(
            self.scale,
            self.even.clone_custom(),
            self.odd.clone_custom(),
        ))
    }
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Box<dyn Texture>, odd: Box<dyn Texture>) -> CheckerTexture {
        CheckerTexture {
            scale: scale,
            even: even,
            odd: odd,
        }
    }

    pub fn new_color(scale: f64, even: &Vec3, odd: &Vec3) -> CheckerTexture {
        CheckerTexture {
            scale: scale,
            even: Box::new(SolidColor::new(even)),
            odd: Box::new(SolidColor::new(odd)),
        }
    }
}

pub struct ImageTexture {
    data: Arc<[u8]>, 
    width: usize,
    height: usize,
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.read(
            (u * (self.width as f64)) as usize,
            ((1.0 - v) * (self.height as f64)) as usize,
        )
        .expect("Could not access pixel in image texture")
        // Vec3::new(0.0, 1.0, 0.0)
    }

    fn clone_custom(&self) -> Box<dyn Texture> {
        Box::new(ImageTexture {
            data: self.data.clone(),
            width: self.width,
            height: self.height,
        })
    }
}

impl ImageTexture {
    pub fn new(file_path: &str) -> Result<ImageTexture, String> {
        println!("Started loading image");
        // Load the image from file
        let image = match image::open(&Path::new(file_path)) {
            Ok(image) => image,
            Err(err) => return Err(format!("Failed to open image: {}", err)),
        };

        // Get the dimensions of the image
        let (width, height) = image.dimensions();

        // Convert the image into RGBA raw data
        let rgba_data: Vec<u8> = image.to_rgba8().to_vec();

        // Convert the Vec<u8> into an Arc reference
        let arc_data = Arc::from(rgba_data);

        println!("Finished loading image");

        Ok(ImageTexture {
            data: arc_data,
            width: width as usize,
            height: height as usize,
        })
    }

    // Function to read RGB values at a specific (x, y) coordinate
    pub fn read(&self, x: usize, y: usize) -> Option<Vec3> {
        if x < self.width && y < self.height {
            let pixel_offset = (y * self.width + x) * 4; // Each pixel has 4 components (RGBA)
            let r = self.data[pixel_offset] as f64 / 255.0;
            let g = self.data[pixel_offset + 1] as f64 / 255.0;
            let b = self.data[pixel_offset + 2] as f64 / 255.0;

            Some(Vec3::new(r, g, b))
        } else {
            None
        }
    }
}