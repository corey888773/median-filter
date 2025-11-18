mod shared;
mod sequential;

use clap::Parser;
use csv::Writer;
use serde::Serialize;
use std::fs::{create_dir_all, OpenOptions};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(name = "median-filter")]
#[command(about = "Apply median filter to images with various methods", long_about = None)]
struct Args {
    /// Input image path
    #[arg(short, long)]
    input: PathBuf,

    /// Output image path
    #[arg(short, long)]
    output: PathBuf,

    /// Noise level (0.0 to 1.0)
    #[arg(short, long, default_value = "0.0")]
    noise: f32,

    /// Method: seq, par, dist, gpu
    #[arg(short, long, default_value = "seq")]
    method: String,

    /// Kernel size (3 or 5)
    #[arg(short, long, default_value = "3")]
    kernel: usize,
}

#[derive(Serialize)]
struct Measurement {
    timestamp: String,
    image: String,
    kernel_size: usize,
    noise_level: f32,
    processing_time_ms: f64,
    method: String,
}

fn validate_args(args: &Args) -> Result<(), String> {
    if args.kernel != 3 && args.kernel != 5 {
        return Err("Kernel size must be 3 or 5".to_string());
    }

    if args.noise < 0.0 || args.noise > 1.0 {
        return Err("Noise level must be between 0.0 and 1.0".to_string());
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    // Validate arguments
    if let Err(e) = validate_args(&args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    println!("Loading image: {:?}", args.input);
    let mut img = shared::Image::load(&args.input).expect("Failed to load image");

    // Add noise if requested
    if args.noise > 0.0 {
        println!("Adding {}% noise...", args.noise * 100.0);
        shared::add_noise(&mut img, args.noise);
    }

    // Apply median filter based on method
    println!("Applying median filter (method: {}, kernel: {}x{})...",
             args.method, args.kernel, args.kernel);

    let start = Instant::now();
    let filtered = match args.method.as_str() {
        "seq" => sequential::apply_median_filter(&img, args.kernel),
        _ => {
            eprintln!("Error: Unknown method '{}'. Available: seq", args.method);
            std::process::exit(1);
        }
    };
    let duration = start.elapsed();
    let processing_time_ms = duration.as_secs_f64() * 1000.0;

    println!("Processing time: {:.2} ms", processing_time_ms);

    // Save output image
    println!("Saving output: {:?}", args.output);
    filtered.save(&args.output).expect("Failed to save output image");

    // Save measurement to CSV
    save_measurement(&args, processing_time_ms).expect("Failed to save measurement");

    println!("Done!");
}

fn save_measurement(args: &Args, processing_time_ms: f64) -> Result<(), Box<dyn std::error::Error>> {
    // Create results directory if it doesn't exist
    create_dir_all("results")?;

    let csv_path = format!("results/{}.csv", args.method);

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&csv_path)?;

    let mut wtr = Writer::from_writer(file);

    let measurement = Measurement {
        timestamp: chrono::Local::now().to_rfc3339(),
        image: args.input.to_string_lossy().to_string(),
        kernel_size: args.kernel,
        noise_level: args.noise,
        processing_time_ms,
        method: args.method.clone(),
    };

    wtr.serialize(measurement)?;
    wtr.flush()?;

    println!("Measurement saved to: {}", csv_path);

    Ok(())
}
