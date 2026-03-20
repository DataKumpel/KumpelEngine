//***** UNIFORMS **********************************************************************************
struct CameraUniform {
    view_proj: mat4x4<f32>,
}

struct LightUniform {
    position: vec4<f32>, // xyz + radius
    color: vec4<f32>,
}
//***** UNIFORMS **********************************************************************************


//***** BIND GROUPS *******************************************************************************
@group(0) @binding(0) var<uniform> camera: CameraUniform;

@group(1) @binding(0) var tex_diffuse: texture_2d<f32>;
@group(1) @binding(1) var sampler_diffuse: sampler;

@group(2) @binding(0) var<uniform> light: LightUniform;
//***** BIND GROUPS *******************************************************************************


//***** STRUCTURES ********************************************************************************
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
}

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
}
//***** STRUCTURES ********************************************************************************


@vertex
fn vs_main(model: VertexInput, instance: InstanceInput) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    var out: VertexOutput;
    out.tex_coords = model.tex_coords;

    let world_position = model_matrix * vec4<f32>(model.position, 1.0);
    out.world_position = world_position.xyz;
    out.world_normal = (model_matrix * vec4<f32>(model.normal, 0.0)).xyz;  // W=0.0 because its a direction, not point in space...

    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(model.position, 1.0);
    return out;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // ---> Read texture color:
    let object_color = textureSample(tex_diffuse, sampler_diffuse, in.tex_coords);

    // ---> Read light properties:
    let light_pos = light.position.xyz;
    let light_radius = light.position.w;

    // ---> Calculate distance:
    let light_distance = length(light_pos - in.world_position);

    // ---> Smooth light attenuation:
    let attenuation = smoothstep(light_radius, 0.0, light_distance);

    // ---> Ambient light:
    let ambient_strength = 0.01;
    let ambient_color = light.color.xyz * ambient_strength;

    // ---> Diffuse light:
    let normal = normalize(in.world_normal);
    let light_dir = normalize(light.position.xyz - in.world_position);
    let diffuse_strength = max(dot(normal, light_dir), 0.0); // No negative light...
    let diffuse_color = light.color.xyz * attenuation * diffuse_strength;

    // ---> Combine to result:
    let result = (ambient_color + diffuse_color) * object_color.xyz;

    return vec4<f32>(result, object_color.a);
}


