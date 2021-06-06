# Bevy pbr include
If you ever wanted to have the variables of pbr defined by a shader, this is the crate for you.

## Usage
The plugin replaces any instance of `#pbr` in glsl shaders with a template that grants the following functionality:
* ```glsl
  struct Pbr {
      vec3 camera_pos;
      vec3 world_pos;
      vec3 normal;
      vec3 normal_map;
      vec4 tangent;
      vec3 albedo;
      float metallic;
      float roughness;
      float occlusion;
      float reflectance;
      vec4 emissive;
  };
* ```glsl
  // creates a Pbr with default settings, the same as for StandardMaterial
  Pbr default_pbr(vec3 camera_pos, vec3 world_pos, vec3 world_normal);
* ```glsl
  // basically just runs the pbr shader, but with user defined parameters
  vec4 pbr(Pbr pbr);
Example Shader:
```glsl
#pbr

layout(location = 0) in vec3 v_WorldPosition;
layout(location = 1) in vec3 v_WorldNormal;

layout(location = 0) out vec4 o_Target;

layout(set = 0, binding = 1) uniform CameraPosition {
    vec3 cameraPos;
};

void main() {
    Pbr settings = default_pbr(cameraPos, v_WorldPosition, v_WorldNormal);

    float x = dot(normalize(v_WorldNormal), vec3(1.0, 0.0, 0.0));
    float y = dot(normalize(v_WorldNormal), vec3(0.0, 1.0, 0.0));
    float z = dot(normalize(v_WorldNormal), vec3(0.0, 0.0, 1.0));
    settings.albedo = vec3(x, y, z);

    o_Target = pbr(settings);
}
```

## Compatibility
Currently only targets main.