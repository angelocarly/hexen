use std::default::Default;
use std::time::Instant;
use wgpu::InstanceDescriptor;
use wgpu::util::DeviceExt;

#[tokio::main]
async fn main() {
    let instance = wgpu::Instance::new(InstanceDescriptor::default());
    let adapter = instance.request_adapter(&Default::default()).await.unwrap();
    let features = adapter.features();
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: features & wgpu::Features::TIMESTAMP_QUERY,
                required_limits: Default::default()
            },
            None,
        )
        .await
        .unwrap();

    let start_instant = Instant::now();
    let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });
    println!("shader compilation {:?}", start_instant.elapsed());

    let intput_f = &[1.0f32, 2.0f32];
    let input: &[u8] = bytemuck::bytes_of(intput_f);
    let input_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: input,
        usage: wgpu::BufferUsages::STORAGE
        | wgpu::BufferUsages::COPY_DST
        | wgpu::BufferUsages::COPY_SRC
    });
    let output_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: input.len() as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // What's a bind group layout? Is it a descriptorsetlayout?
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None
        }],
    });
    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&compute_pipeline_layout),
        module: &cs_module,
        entry_point: "main"
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: input_buf.as_entire_binding()
        }],
    });

    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let mut cpass = encoder.begin_compute_pass(&Default::default());
        cpass.set_pipeline(&pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.dispatch_workgroups(intput_f.len() as u32, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&input_buf, 0, &output_buf, 0, input.len() as u64);
    queue.submit(Some(encoder.finish()));

    device.poll(wgpu::Maintain::Wait);

    let buf_slice = output_buf.slice(..);
    buf_slice.map_async(wgpu::MapMode::Read, move |_| {});
    device.poll(wgpu::Maintain::Wait);

    let data_raw = &*buf_slice.get_mapped_range();
    let data: &[f32] = bytemuck::cast_slice(data_raw);
    println!("data: {:?}", &*data);
}
