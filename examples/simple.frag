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