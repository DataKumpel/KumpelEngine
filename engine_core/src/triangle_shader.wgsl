

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    // Coordinates in WGPU range from -1.0 to 1.0
    var pos = array<vec2<f32>, 3>(
        vec2<f32>( 0.0,  0.5), // upper middle
        vec2<f32>(-0.5, -0.5), // lower left
        vec2<f32>( 0.5, -0.5), // lower right
    );

    return vec4<f32>(pos[in_vertex_index], 0.0, 1.0);
}


@fragment
fn fs_main() -> @location(0) vec4<f32> {
    // Color orange:
    return vec4<f32>(1.0, 0.5, 0.0, 1.0);
}
