use crate::shared::Image;
use image::Rgb;
use wgpu::util::DeviceExt;

const SHADER_SOURCE: &str = r#"
struct Params {
    width: u32,
    height: u32,
    kernel_size: u32,
}

@group(0) @binding(0) var<storage, read> input: array<u32>;
@group(0) @binding(1) var<storage, read_write> output: array<u32>;
@group(0) @binding(2) var<uniform> params: Params;

fn median9(values: array<u32, 9>) -> u32 {
    var v = values;
    for (var i = 0u; i < 9u; i++) {
        for (var j = i + 1u; j < 9u; j++) {
            if (v[i] > v[j]) {
                let tmp = v[i];
                v[i] = v[j];
                v[j] = tmp;
            }
        }
    }
    return v[4];
}

fn median25(values: array<u32, 25>) -> u32 {
    var v = values;
    for (var i = 0u; i < 25u; i++) {
        for (var j = i + 1u; j < 25u; j++) {
            if (v[i] > v[j]) {
                let tmp = v[i];
                v[i] = v[j];
                v[j] = tmp;
            }
        }
    }
    return v[12];
}

fn get_pixel_channel(x: i32, y: i32, channel: u32) -> u32 {
    // Mirror padding logic
    var nx = x;
    var ny = y;
    
    if (nx < 0) { nx = -nx; }
    if (nx >= i32(params.width)) { nx = 2 * i32(params.width) - nx - 2; }
    if (ny < 0) { ny = -ny; }
    if (ny >= i32(params.height)) { ny = 2 * i32(params.height) - ny - 2; }

    let pixel_idx = u32(ny) * params.width + u32(nx);
    let packed = input[pixel_idx];

    if (channel == 0u) {
        return (packed >> 16u) & 0xFFu;
    } else if (channel == 1u) {
        return (packed >> 8u) & 0xFFu;
    } else {
        return packed & 0xFFu;
    }
}

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = global_id.x;
    let y = global_id.y;

    if (x >= params.width || y >= params.height) {
        return;
    }

    let half = i32(params.kernel_size) / 2;
    var result_packed = 0u;

    // Process each channel separately
    for (var channel = 0u; channel < 3u; channel++) {
        var median_val = 0u;

        if (params.kernel_size == 3u) {
            var values: array<u32, 9>;
            var idx = 0u;
            for (var dy = -half; dy <= half; dy++) {
                for (var dx = -half; dx <= half; dx++) {
                    values[idx] = get_pixel_channel(i32(x) + dx, i32(y) + dy, channel);
                    idx++;
                }
            }
            median_val = median9(values);
        } else { // kernel_size == 5
            var values: array<u32, 25>;
            var idx = 0u;
            for (var dy = -half; dy <= half; dy++) {
                for (var dx = -half; dx <= half; dx++) {
                    values[idx] = get_pixel_channel(i32(x) + dx, i32(y) + dy, channel);
                    idx++;
                }
            }
            median_val = median25(values);
        }

        // Pack result
        if (channel == 0u) {
            result_packed |= (median_val << 16u);
        } else if (channel == 1u) {
            result_packed |= (median_val << 8u);
        } else {
            result_packed |= median_val;
        }
    }
    
    let out_idx = y * params.width + x;
    output[out_idx] = result_packed;
}
"#;

pub fn apply_median_filter(img: &Image, kernel_size: usize) -> Image {
    // Initialize WGPU
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    }))
    .expect("Failed to find GPU adapter");

    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        label: None,
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::default(),
        memory_hints: wgpu::MemoryHints::default(),
        trace: wgpu::Trace::Off,
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
    }))
    .expect("Failed to create device");

    // Compile shader
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Median Filter Shader"),
        source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
    });

    // Pack RGB pixels into u32 (R << 16 | G << 8 | B)
    let input_data: Vec<u32> = img
        .data
        .pixels()
        .map(|p| {
            let r = p[0] as u32;
            let g = p[1] as u32;
            let b = p[2] as u32;
            (r << 16) | (g << 8) | b
        })
        .collect();

    let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Input Buffer"),
        contents: bytemuck::cast_slice(&input_data),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    });

    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output Buffer"),
        size: (img.width * img.height * 4) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let params = [img.width, img.height, kernel_size as u32];
    let params_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Params Buffer"),
        contents: bytemuck::cast_slice(&params),
        usage: wgpu::BufferUsages::UNIFORM,
    });

    // Create bind group layout
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: input_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: output_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: params_buffer.as_entire_binding(),
            },
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Median Filter Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: Some("main"),
        compilation_options: Default::default(),
        cache: None,
    });

    // Create staging buffer for reading results
    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Staging Buffer"),
        size: (img.width * img.height * 4) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // Execute compute shader
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Command Encoder"),
    });

    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute Pass"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);

        let workgroup_size = 8;
        let dispatch_x = (img.width + workgroup_size - 1) / workgroup_size;
        let dispatch_y = (img.height + workgroup_size - 1) / workgroup_size;
        compute_pass.dispatch_workgroups(dispatch_x, dispatch_y, 1);
    }

    encoder.copy_buffer_to_buffer(
        &output_buffer,
        0,
        &staging_buffer,
        0,
        (img.width * img.height * 4) as u64,
    );

    let submission_index = queue.submit(Some(encoder.finish()));

    // Read results
    let buffer_slice = staging_buffer.slice(..);
    let (sender, receiver) = std::sync::mpsc::channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
        sender.send(result).unwrap();
    });

    let _ = device.poll(wgpu::PollType::Wait {
        submission_index: Some(submission_index),
        timeout: None,
    });
    receiver.recv().unwrap().expect("Failed to map buffer");

    let data = buffer_slice.get_mapped_range();
    let output_data: Vec<u32> = bytemuck::cast_slice(&data).to_vec();
    drop(data);
    staging_buffer.unmap();

    // Unpack u32 back to RGB
    let mut output = Image::new_empty(img.width, img.height);
    for (i, &packed) in output_data.iter().enumerate() {
        let x = (i as u32) % img.width;
        let y = (i as u32) / img.width;

        let r = ((packed >> 16) & 0xFF) as u8;
        let g = ((packed >> 8) & 0xFF) as u8;
        let b = (packed & 0xFF) as u8;

        output.put_pixel(x, y, Rgb([r, g, b]));
    }

    output
}
