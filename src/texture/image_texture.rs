use crate::vector::Vec3;
use image::GenericImageView;
use std::path::Path;
use std::sync::Arc;

use super::base::Texture;

pub struct ImageTexture {
    data: Arc<[u8]>, 
    width: usize,
    height: usize,
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.read(
            (u * (self.width as f64)) as usize,
            ((1.0 - v) * (self.height as f64)) as usize,
        )
        .expect("Could not access pixel in image texture")
        // Vec3::new(0.0, 1.0, 0.0)
    }
}

impl ImageTexture {
    pub fn new(file_path: &str) -> Result<ImageTexture, String> {
        println!("Started loading image");
        // Load the image from file
        let image = match image::open(Path::new(file_path)) {
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
            println!("{}, {}", x, y);
            None
        }
    }
}