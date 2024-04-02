struct ImageBuf {
    data: array<vec4f>,
}

@group(0)
@binding(0)
var<storage, read_write> image: ImageBuf;

// Todo: send image size to shader
@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let id = global_id.x + global_id.y * 512;

    let pixel_level = u32( 32 );
    var p = vec2f( f32( global_id.x ) / 512.0f, f32( global_id.y ) / 512.0f );
    p = floor( p * 512.0f / f32( pixel_level ) ) / ( 512.0f / f32( pixel_level ) );

    var data = image.data[id];
    data = vec4f( p, 0, 1 );

    image.data[id] = data;
}