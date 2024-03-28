struct DataBuf {
    data: array<vec4f>,
}

@group(0)
@binding(0)
var<storage, read_write> v_indices: DataBuf;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // Todo: send image size to shader
    let id = global_id.x + global_id.y * 512;
    var data = v_indices.data[id];
    data = vec4f( f32( global_id.x ) / 512.0f, f32( global_id.y ) / 512.0f, 0, 1 );
    v_indices.data[id] = data;
}