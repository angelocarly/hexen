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

fn screenPosToHexCoord(position : vec2f, diameter: f32) -> vec3f {
    let hexToScreenMatrix = ( 1.0f / 3.0f ) * mat2x2(-3, 0, -sqrt( 3.0f ), 2.0 * sqrt( 3.0f ) );

    let posInHexSpace = hexToScreenMatrix * (position / vec2f(diameter));

    let newPos = vec3f( -posInHexSpace.x - posInHexSpace.y, posInHexSpace.x, posInHexSpace.y );

    // How much does the position deviate from a unit coord?
    let roundDelta = vec3f( abs(round(newPos.x) - newPos.x), abs(round(newPos.y) - newPos.y), abs(round(newPos.z) - newPos.z) );

    // Recalculate the axis with the biggest error
    var nodepos: vec3f;
    if (roundDelta.z > roundDelta.x && roundDelta.z > roundDelta.y)
    {
        // Z biggest error
        nodepos.x = round(newPos.x);
        nodepos.y = round(newPos.y);
        nodepos.z = -nodepos.x - nodepos.y;
    } else if (roundDelta.y > roundDelta.x && roundDelta.y > roundDelta.z)
    {
        // Y biggest error
        nodepos.x = round(newPos.x);
        nodepos.z = round(newPos.z);
        nodepos.y = -nodepos.x - nodepos.z;
    } else
    {
        // X biggest error
        nodepos.y = round(newPos.y);
        nodepos.z = round(newPos.z);
        nodepos.x = -nodepos.y - nodepos.z;
    }

    return nodepos;
}


// Todo: send image size to shader
@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let id = global_id.x + global_id.y * 512;

    let pixel_level = u32( 4 );
    var p = vec2f( f32( global_id.x ) / 512.0f, f32( global_id.y ) / 512.0f );
    p = floor( p * 512.0f / f32( pixel_level ) ) / ( 512.0f / f32( pixel_level ) ) + vec2f( 0.5f / ( 512.0f / f32( pixel_level ) ) );

    // Center position
    p = p - vec2f( .5f );

    // Hexagons!
    let hexCoord = screenPosToHexCoord( p, 0.02f );
    var data = vec4f( hexCoord, 1 );

    // Edge
    if length( p ) > .47f {
        data = vec4f( 0, 0, 0, 1 );
    }

    data += funcdata.data[0];

    image.data[id] = data;
}