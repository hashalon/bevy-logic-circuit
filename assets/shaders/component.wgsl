// shader material to display the state of a component

// Tutorial: https://www.youtube.com/watch?v=1zzxqwm5kps

// pass colors to the shader
struct UniformData {
    color: vec4<f32>
};

// define render effects for off, low aand high states
// use some kind of interpolation for those two states

@group(1) @binding(0) var<uniform> state_off  : UniformData;
@group(1) @binding(1) var<uniform> state_low  : UniformData;
@group(1) @binding(2) var<uniform> state_high : UniformData;


struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
}

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    return state_low.color;
}