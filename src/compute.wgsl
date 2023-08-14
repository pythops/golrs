@group(0) @binding(0) var<uniform> grid: f32;
@group(0) @binding(1) var<storage> cell_state_in: array<u32>;
@group(0) @binding(2) var<storage, read_write> cell_state_out: array<u32>;


fn get_cell_index(cell: vec2u) -> u32 {
    return (cell.y % u32(grid)) * u32(grid) + (cell.x % u32(grid));
}

fn cell_active(x: u32, y: u32) -> u32 {
    let cell_index = get_cell_index(vec2u(x, y));
    return cell_state_in[cell_index];
}

@compute @workgroup_size(8,8)
fn cs_main(@builtin(global_invocation_id) cell: vec3u) {
    let active_neighbots: u32 = cell_active(cell.x + 1u, cell.y + 1u) + cell_active(cell.x + 1u, cell.y) + cell_active(cell.x + 1u, cell.y - 1u) + cell_active(cell.x, cell.y - 1u) + cell_active(cell.x - 1u, cell.y - 1u) + cell_active(cell.x - 1u, cell.y) + cell_active(cell.x - 1u, cell.y + 1u) + cell_active(cell.x, cell.y + 1u);

    let cell_index = get_cell_index(cell.xy);

    switch active_neighbots {
        case 2u {
            cell_state_out[cell_index] = cell_state_in[cell_index];
        }
        case 3u {
            cell_state_out[cell_index] = 1u;
        }
        default {
            cell_state_out[cell_index] = 0u;
        }
    }
}
