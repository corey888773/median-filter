use crate::shared::{collect_neighborhood, median_rgb, Image};
use image::Rgb;
use mpi::traits::*;
use std::path::Path;

pub fn apply_median_filter_mpi(
    input_path: &Path,
    noise_level: f32,
    kernel_size: usize,
) -> (Image, i32, i32) {
    let universe = mpi::initialize().expect("Failed to initialize MPI");
    let world = universe.world();
    let rank = world.rank();
    let size = world.size();

    let half_kernel = (kernel_size / 2) as i32;

    // Root process loads image and broadcasts dimensions
    if rank == 0 {
        let mut img = Image::load(input_path).expect("Failed to load image");
        if noise_level > 0.0 {
            crate::shared::add_noise(&mut img, noise_level);
        }



        // Broadcast dimensions to all processes
        for dest in 1..size {
            world.process_at_rank(dest).send(&img.width);
            world.process_at_rank(dest).send(&img.height);
        }

        // Distribute work
        let rows_per_process = (img.height as i32 + size - 1) / size;
        let mut results = Vec::new();

        for proc in 0..size {
            let start_row = proc * rows_per_process;
            let end_row = ((proc + 1) * rows_per_process).min(img.height as i32);

            if start_row >= img.height as i32 {
                continue;
            }

            // Calculate ghost region
            let ghost_start = (start_row - half_kernel).max(0);
            let ghost_end = (end_row + half_kernel).min(img.height as i32);

            if proc == 0 {
                // Process 0 processes its own chunk
                let chunk = extract_chunk(&img, ghost_start, ghost_end);
                let processed = process_chunk(&chunk, start_row - ghost_start, end_row - ghost_start, kernel_size);
                results.push((start_row, end_row, processed));
            } else {
                // Send chunk to worker process
                world.process_at_rank(proc).send(&start_row);
                world.process_at_rank(proc).send(&end_row);
                world.process_at_rank(proc).send(&ghost_start);
                world.process_at_rank(proc).send(&ghost_end);

                let chunk_data = serialize_chunk(&img, ghost_start, ghost_end);
                world.process_at_rank(proc).send(&chunk_data[..]);
            }
        }

        // Receive results from workers
        for proc in 1..size {
            let start_row = proc * rows_per_process;
            let end_row = ((proc + 1) * rows_per_process).min(img.height as i32);

            if start_row >= img.height as i32 {
                continue;
            }

            let chunk_height = (end_row - start_row) as usize;
            let mut buffer = vec![0u8; img.width as usize * chunk_height * 3];
            world.process_at_rank(proc).receive_into(&mut buffer[..]);

            let processed = deserialize_chunk(&buffer, img.width, chunk_height as u32);
            results.push((start_row, end_row, processed));
        }

        // Assemble final image
        let mut output = Image::new_empty(img.width, img.height);
        for (start_row, end_row, chunk) in results {
            for y in start_row..end_row {
                for x in 0..img.width {
                    let pixel = chunk.get_pixel(x, (y - start_row) as u32);
                    output.put_pixel(x, y as u32, *pixel);
                }
            }
        }

        (output, rank, size)
    } else {
        // Worker process
        let width: u32 = world.process_at_rank(0).receive().0;
        let _height: u32 = world.process_at_rank(0).receive().0;

        let start_row: i32 = world.process_at_rank(0).receive().0;
        let end_row: i32 = world.process_at_rank(0).receive().0;
        let ghost_start: i32 = world.process_at_rank(0).receive().0;
        let ghost_end: i32 = world.process_at_rank(0).receive().0;

        let ghost_height = (ghost_end - ghost_start) as usize;
        let mut chunk_data = vec![0u8; width as usize * ghost_height * 3];
        world.process_at_rank(0).receive_into(&mut chunk_data[..]);

        let chunk = deserialize_chunk(&chunk_data, width, ghost_height as u32);
        let processed = process_chunk(&chunk, start_row - ghost_start, end_row - ghost_start, kernel_size);

        // Send result back
        let result_data = serialize_chunk(&processed, 0, processed.height as i32);
        world.process_at_rank(0).send(&result_data[..]);

        (Image::new_empty(1, 1), rank, size)
    }
}

fn extract_chunk(img: &Image, start_row: i32, end_row: i32) -> Image {
    let height = (end_row - start_row) as u32;
    let mut chunk = Image::new_empty(img.width, height);

    for y in start_row..end_row {
        for x in 0..img.width {
            let pixel = img.get_pixel(x, y as u32);
            chunk.put_pixel(x, (y - start_row) as u32, *pixel);
        }
    }

    chunk
}

fn serialize_chunk(img: &Image, start_row: i32, end_row: i32) -> Vec<u8> {
    let mut data = Vec::new();
    for y in start_row..end_row {
        for x in 0..img.width {
            let pixel = img.get_pixel(x, y as u32);
            data.push(pixel[0]);
            data.push(pixel[1]);
            data.push(pixel[2]);
        }
    }
    data
}

fn deserialize_chunk(data: &[u8], width: u32, height: u32) -> Image {
    let mut chunk = Image::new_empty(width, height);
    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 3) as usize;
            let pixel = Rgb([data[idx], data[idx + 1], data[idx + 2]]);
            chunk.put_pixel(x, y, pixel);
        }
    }
    chunk
}

fn process_chunk(chunk: &Image, start_row: i32, end_row: i32, kernel_size: usize) -> Image {
    let output_height = (end_row - start_row) as u32;
    let mut output = Image::new_empty(chunk.width, output_height);

    for y in start_row..end_row {
        for x in 0..chunk.width {
            let neighborhood = collect_neighborhood(chunk, x, y as u32, kernel_size);
            let median_pixel = median_rgb(&neighborhood);
            output.put_pixel(x, (y - start_row) as u32, median_pixel);
        }
    }

    output
}

