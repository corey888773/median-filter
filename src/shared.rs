use image::{Rgb, RgbImage};
use rand::Rng;
use std::path::Path;

/// Wrapper around image data for easier manipulation
pub struct Image {
    pub data: RgbImage,
    pub width: u32,
    pub height: u32,
}

impl Image {
    /// Load image from file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, image::ImageError> {
        let img = image::open(path)?;
        let rgb_img = img.to_rgb8();
        let (width, height) = rgb_img.dimensions();
        
        Ok(Image {
            data: rgb_img,
            width,
            height,
        })
    }

    /// Save image to file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), image::ImageError> {
        self.data.save(path)
    }

    /// Create a new image with the same dimensions
    pub fn new_empty(width: u32, height: u32) -> Self {
        Image {
            data: RgbImage::new(width, height),
            width,
            height,
        }
    }

    /// Get pixel at (x, y)
    pub fn get_pixel(&self, x: u32, y: u32) -> &Rgb<u8> {
        self.data.get_pixel(x, y)
    }

    /// Set pixel at (x, y)
    pub fn put_pixel(&mut self, x: u32, y: u32, pixel: Rgb<u8>) {
        self.data.put_pixel(x, y, pixel);
    }

    /// Get pixel with mirror padding for out-of-bounds coordinates
    pub fn get_pixel_padded(&self, x: i32, y: i32) -> Rgb<u8> {
        let px = if x < 0 {
            (-x) as u32
        } else if x >= self.width as i32 {
            (2 * self.width as i32 - x - 2) as u32
        } else {
            x as u32
        };

        let py = if y < 0 {
            (-y) as u32
        } else if y >= self.height as i32 {
            (2 * self.height as i32 - y - 2) as u32
        } else {
            y as u32
        };

        let px = px.min(self.width - 1);
        let py = py.min(self.height - 1);

        *self.get_pixel(px, py)
    }
}

/// Add salt-and-pepper noise to the image
/// noise_level: 0.0 to 1.0 (percentage of pixels to corrupt)
pub fn add_noise(img: &mut Image, noise_level: f32) {
    let mut rng = rand::thread_rng();
    let total_pixels = (img.width * img.height) as f32;
    let pixels_to_corrupt = (total_pixels * noise_level) as u32;

    for _ in 0..pixels_to_corrupt {
        let x = rng.gen_range(0..img.width);
        let y = rng.gen_range(0..img.height);
        
        // Randomly choose salt (white) or pepper (black)
        let value = if rng.gen_bool(0.5) { 255 } else { 0 };
        let pixel = Rgb([value, value, value]);
        
        img.put_pixel(x, y, pixel);
    }
}

/// Calculate median of a slice of values
pub fn median(values: &mut [u8]) -> u8 {
    values.sort_unstable();
    values[values.len() / 2]
}

/// Calculate median for RGB pixel (per channel)
pub fn median_rgb(pixels: &[Rgb<u8>]) -> Rgb<u8> {
    let mut r_values: Vec<u8> = pixels.iter().map(|p| p[0]).collect();
    let mut g_values: Vec<u8> = pixels.iter().map(|p| p[1]).collect();
    let mut b_values: Vec<u8> = pixels.iter().map(|p| p[2]).collect();

    Rgb([
        median(&mut r_values),
        median(&mut g_values),
        median(&mut b_values),
    ])
}

/// Collect neighborhood pixels for median filtering
pub fn collect_neighborhood(img: &Image, x: u32, y: u32, kernel_size: usize) -> Vec<Rgb<u8>> {
    let radius = (kernel_size / 2) as i32;
    let mut pixels = Vec::with_capacity(kernel_size * kernel_size);

    for dy in -radius..=radius {
        for dx in -radius..=radius {
            let px = x as i32 + dx;
            let py = y as i32 + dy;
            pixels.push(img.get_pixel_padded(px, py));
        }
    }

    pixels
}

