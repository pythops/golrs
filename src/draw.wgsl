@group(0) @binding(0) var<uniform> grid: f32;
@group(0) @binding(1) var<storage> cell_state: array<u32>;



struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
    @builtin(instance_index) cell_instance: u32,
};

struct VertexOutput {
    @builtin(position) pos: vec4f,
    @location(0) cell: vec2f,
    @location(1) color: vec3f,
};


@vertex
fn vs_main(input: VertexInput) -> VertexOutput {

    let cell_index = f32(input.cell_instance);

    let cell = vec2f(cell_index % grid, floor(cell_index / grid));

    let state = f32(cell_state[input.cell_instance]);

    let cell_offset = cell / grid * 2.0;

    let cell_postition = (input.position * state + 1.0) / grid - 1.0 + cell_offset;

    var out: VertexOutput;
    out.pos = vec4f(cell_postition, 0.0, 1.0);
    out.cell = cell;
    out.color = input.color;

    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4f {
    return vec4f(input.color, 1.0);
}
