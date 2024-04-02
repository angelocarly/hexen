struct ImageBuf {
    data: array<vec4f>,
}

struct DataBuf {
    data: array<vec4f>,
}

@group(0)
@binding(0)
var<storage, read_write> image: ImageBuf;

@group(0)
@binding(1)
var<storage, read_write> funcdata: DataBuf;

// Todo: send image size to shader
@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let id = global_id.x + global_id.y * 512;

    let pixel_level = u32( 4 );
    var p = vec2f( f32( global_id.x ) / 512.0f, f32( global_id.y ) / 512.0f );
    p = floor( p * 512.0f / f32( pixel_level ) ) / ( 512.0f / f32( pixel_level ) ) + vec2f( 0.5f / ( 512.0f / f32( pixel_level ) ) );

    var data = image.data[id];
    data = vec4f( p, funcdata.data[0].r, 1 );

    if length( p - vec2f( .5f ) ) > .47f {
        data = vec4f( 0, 0, 0, 1 );
    }

    image.data[id] = data;
}