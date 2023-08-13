@group(0) @binding(0) var<uniform> grid: f32;

struct VertexOutput {
    @builtin(position) foo: vec4f,
    @location(0) pos: vec2f,
};

@vertex
fn vs_main(@location(0) position: vec2<f32>) -> VertexOutput {
    var out: VertexOutput;
    out.foo = vec4f(position / grid, 0.0, 1.0);
    out.pos = position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return vec4f(1.0, 0.0, 0.0, 1.0);
}
