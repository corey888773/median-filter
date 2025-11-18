use crate::shared::{collect_neighborhood, median_rgb, Image};
use mpi::traits::*;
use std::path::Path;

pub fn apply_median_filter_mpi(
    input_path: &Path,
    noise_level: f32,
    kernel_size: usize,
) -> (Image, i32) {
    let universe = mpi::initialize().expect("Failed to initialize MPI");
    let world = universe.world();
    let rank = world.rank();
    let _size = world.size();

    // Only root process loads and processes the image
    if rank == 0 {
        let mut img = Image::load(input_path).expect("Failed to load image");
        if noise_level > 0.0 {
            crate::shared::add_noise(&mut img, noise_level);
        }

        // For now, just do sequential processing on root
        // TODO: Implement proper distributed processing
        let output = process_sequential(&img, kernel_size);
        (output, rank)
    } else {
        // Worker processes just wait and return dummy
        (Image::new_empty(1, 1), rank)
    }
}

fn process_sequential(img: &Image, kernel_size: usize) -> Image {
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

