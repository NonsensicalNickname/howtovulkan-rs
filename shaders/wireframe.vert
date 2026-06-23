#version 450 core
#extension GL_EXT_debug_printf : enable

layout (std140, set = 0, binding = 0) uniform ShaderData {
    mat4 projection;
    mat4 view;
    mat4 model[3];
    vec4 lightPos;
    uint selected;
} shaderData;

layout (location = 0) in vec3 Pos; 
layout (location = 1) in vec3 Normal;
layout (location = 2) in vec2 UV;

void main() {
    mat4 modelMat = shaderData.model[gl_InstanceIndex];

    gl_Position = shaderData.projection * shaderData.view * modelMat * vec4(Pos.xyz + normalize(Normal) * 0.05, 1.0);
}
