mod image;

use std::default::Default;
use std::time::Instant;
use wgpu::{BindGroupLayout, Device, InstanceDescriptor};
use wgpu::util::DeviceExt;
use crate::image::Color;

fn create_bind_group_layout(device: &Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }, wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
    })
}

fn create_compute_pipeline(device: &Device, bind_group_layout: &BindGroupLayout) -> wgpu::ComputePipeline {
    let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });
    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });
    device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&compute_pipeline_layout),
        module: &cs_module,
        entry_point: "main",
    })
}

fn buffer_data() -> Vec<f32> {
    vec![ 0f32; 4 ]
}

#[tokio::main]
async fn main() {
    const WIDTH: usize = 512;
    const HEIGHT: usize = 512;

    // Device setup

    let instance = wgpu::Instance::new(InstanceDescriptor::default());
    let adapter = instance.request_adapter(&Default::default()).await.unwrap();
    let features = adapter.features();
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: features & wgpu::Features::TIMESTAMP_QUERY,
                required_limits: Default::default(),
            },
            None,
        )
        .await
        .unwrap();

    // Pipeline setup

    let bind_group_layout = create_bind_group_layout(&device);
    let pipeline = create_compute_pipeline(&device, &bind_group_layout);

    // Buffer setup

    let input_f = vec![0f32; 4 * WIDTH * HEIGHT];
    let input: &[u8] = bytemuck::cast_slice(&input_f);
    let input_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: input,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });

    let output_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: input.len() as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let funcdata_f = buffer_data();
    let funcdata: &[u8] = bytemuck::cast_slice(&funcdata_f);
    let funcdata_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: funcdata,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });

    // Bind group setup

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: input_buf.as_entire_binding(),
        }, wgpu::BindGroupEntry {
            binding: 1,
            resource: funcdata_buf.as_entire_binding(),
        }],
    });

    // Render call

    let mut start_instant = Instant::now();
    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let mut cpass = encoder.begin_compute_pass(&Default::default());
        cpass.set_pipeline(&pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.dispatch_workgroups(WIDTH as u32, HEIGHT as u32, 1);
    }
    encoder.copy_buffer_to_buffer(&input_buf, 0, &output_buf, 0, input.len() as u64);
    queue.submit(Some(encoder.finish()));
    device.poll(wgpu::Maintain::Wait);
    println!("shader execution {:?}", start_instant.elapsed());

    // Parse output

    let buf_slice = output_buf.slice(..);
    buf_slice.map_async(wgpu::MapMode::Read, move |_| {});
    device.poll(wgpu::Maintain::Wait);

    let data_raw = &*buf_slice.get_mapped_range();
    let data: &[f32] = bytemuck::cast_slice(data_raw);

    // Image output

    start_instant = Instant::now();
    let mut color_sink = image::ColorSink::new(WIDTH as u32, HEIGHT as u32);
    for (index, chunk) in data.chunks(4).enumerate() {
        color_sink.get_data()[index] = Color::new(
            (255.0f32 * chunk[0]) as u8,
            (255.0f32 * chunk[1]) as u8,
            (255.0f32 * chunk[2]) as u8,
            (255.0f32 * chunk[3]) as u8,
        );
    };
    image::write_png_image(color_sink, "output.png");
    println!("image output {:?}", start_instant.elapsed());
}
