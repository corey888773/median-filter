use crate::shared::{collect_neighborhood, median_rgb, Image};

/// Apply median filter sequentially
/// 
/// # Arguments
/// * `img` - Input image
/// * `kernel_size` - Size of the kernel (3 or 5)
/// 
/// # Returns
/// Filtered image
pub fn apply_median_filter(img: &Image, kernel_size: usize) -> Image {
    let mut output = Image::new_empty(img.width, img.height);

    for y in 0..img.height {
        for x in 0..img.width {
            let neighborhood = collect_neighborhood(img, x, y, kernel_size);
            let median_pixel = median_rgb(&neighborhood);
            output.put_pixel(x, y, median_pixel);
        }
    }

    output
}

