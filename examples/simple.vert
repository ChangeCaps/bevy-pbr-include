#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;

layout(location = 0) out vec3 v_WorldPosition;
layout(location = 1) out vec3 v_WorldNormal;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    v_WorldPosition = (Model * vec4(Vertex_Position, 1.0)).xyz;
    v_WorldNormal = (Model * vec4(Vertex_Normal, 0.0)).xyz;
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
}