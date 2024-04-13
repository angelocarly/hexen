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

fn rand22(n: vec2f) -> f32 { return fract(sin(dot(n, vec2f(12.9898, 4.1414))) * 43758.5453); }

fn ray_cast(p: vec2f) -> vec3f {

    // Calculate normal
    let range = 0.47f;

    if length(p) > range {
        return vec3f(0.0f);
    }

    var normal = vec3f(p.x, p.y, range - length(p));
    normal = normalize(normal);

    // Cast ray from normal
    // Maybe do something with noise
    var col = vec3f(0);
    var pos = vec3f(0);
    let dist = 24.0f;
    let size = 40.0f;

    for (var i = 0; i < 1000; i++) {
        pos += normal * (1.411f);

        if length(pos - vec3f(0, -dist, 0)) < size {
            normal.y = -normal.y;
        }
        if length(pos - vec3f(0, dist, 0)) < size {
            normal.y = -normal.y;
        }
        if length(pos - vec3f(0, 0, -dist)) < size {
            normal.z = -normal.z;
        }
        if length(pos - vec3f(0, 0, dist)) < size {
            normal.z = -normal.z;
        }
        if length(pos - vec3f(-dist, 0, 0)) < size {
            normal.x = -normal.x;
        }
        if length(pos - vec3f(dist, 0, 0)) < size {
            normal.x = -normal.x;
        }
    }

    var v = length(pos) / 20.0f;
    v = v * v * v * v;

    return vec3f(v);
}

// Todo: send image size to shader
    @compute
    @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let width: u32 = u32(2048);
    let id = global_id.x + global_id.y * width;

    let pixel_level = u32(1);
    var p = vec2f(f32(global_id.x) / f32(width), f32(global_id.y) / f32(width));
    p = floor(p * f32(width) / f32(pixel_level)) / (f32(width) / f32(pixel_level)) + vec2f(0.5f / (f32(width) / f32(pixel_level)));

    // Center position
    p = p - vec2f(.5f);

    var color = vec3f(0);
    color += ray_cast(p);

    // Edge
    image.data[id] = vec4f(color, 1);
}
