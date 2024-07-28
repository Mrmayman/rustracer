// vertex_shader.wgsl
@vertex
fn main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0), // bottom-left
        vec2<f32>(1.0, -1.0),  // bottom-right
        vec2<f32>(-1.0, 1.0),  // top-left
        vec2<f32>(-1.0, 1.0),  // top-left
        vec2<f32>(1.0, -1.0),  // bottom-right
        vec2<f32>(1.0, 1.0)    // top-right
    );
    let pos = positions[vertex_index];
    return vec4<f32>(pos, 0.0, 1.0);
}
