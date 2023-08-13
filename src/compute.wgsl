@group(0) @binding(0) var<uniform> grid: f32;
@group(0) @binding(1) var<storage> cell_state_in: array<u32>;
@group(0) @binding(2) var<storage, read_write> cell_state_out: array<u32>;


fn get_cell_index(cell: vec2u) -> u32 {
    return cell.y * u32(grid) + cell.x;
}

@compute @workgroup_size(8,8)
fn cs_main(@builtin(global_invocation_id) cell: vec3u) {
    let cell_index = get_cell_index(cell.xy);
    if cell_state_in[cell_index] == 1u {
        cell_state_out[cell_index] = 0u;
    }
    //} else {
    //    cell_state_out[cell_index] = 1u;
    //}
}
