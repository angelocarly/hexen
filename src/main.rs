use std::default::Default;
use std::time::Instant;
use wgpu::InstanceDescriptor;

#[tokio::main]
async fn main() {
    let instance = wgpu::Instance::new(InstanceDescriptor::default());
    let adapter = instance.request_adapter(&Default::default()).await.unwrap();
    let features = adapter.features();
    let (device, _queue) = adapter
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

    let _query_set = if features.contains(wgpu::Features::TIMESTAMP_QUERY) {
        Some(device.create_query_set(&wgpu::QuerySetDescriptor {
            count: 2,
            ty: wgpu::QueryType::Timestamp,
            label: None,
        }))
    } else {
        None
    };

    let start_instant = Instant::now();
    let _cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });
    println!("shader compilation {:?}", start_instant.elapsed());

}
