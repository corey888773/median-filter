use crate::shared::{collect_neighborhood, median_rgb, Image};
use rayon::prelude::*;

/// Apply median filter in parallel using Rayon
/// 
/// # Arguments
/// * `img` - Input image
/// * `kernel_size` - Size of the kernel (3 or 5)
/// 
/// # Returns
/// Filtered image
pub fn apply_median_filter(img: &Image, kernel_size: usize) -> Image {
    let mut output = Image::new_empty(img.width, img.height);

    // Process rows in parallel
    let rows: Vec<_> = (0..img.height)
        .into_par_iter()
        .map(|y| {
            let mut row_pixels = Vec::with_capacity(img.width as usize);
            for x in 0..img.width {
                let neighborhood = collect_neighborhood(img, x, y, kernel_size);
                let median_pixel = median_rgb(&neighborhood);
                row_pixels.push(median_pixel);
            }
            (y, row_pixels)
        })
        .collect();

    // Write results back to output image
    for (y, row_pixels) in rows {
        for (x, pixel) in row_pixels.into_iter().enumerate() {
            output.put_pixel(x as u32, y, pixel);
        }
    }

    output
}

