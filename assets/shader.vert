#version 450 core

layout (set = 0, binding = 0) uniform sampler2D textures[];
layout (set = 0, binding = 1) uniform ShaderData {
    mat4 projection;
    mat4 view;
    mat4 model[3];
    vec4 lightPos;
    uint selected;
} shaderData;

layout (location = 0) in vec3 Pos; 
layout (location = 1) in vec3 Normal;
layout (location = 2) in vec2 UV;

layout (location = 3) out vec4 outPos; 
layout (location = 4) out vec3 outNormal;
layout (location = 5) out vec2 outUV;
layout (location = 6) out vec3 outFactor;
layout (location = 7) out vec3 outLightVec;
layout (location = 8) out vec3 outViewVec;
layout (location = 9) out uint outInstanceIndex;


void main() {
    mat4 modelMat = shaderData.model[gl_VertexIndex];
    outNormal = mat3(shaderData.view * modelMat) * Normal;
    outUV = UV;
    outPos = shaderData.projection * shaderData.view * modelMat * vec4(Pos.xyz, 1.0);
    outFactor = vec3(shaderData.selected == gl_VertexIndex ? 3.0f : 1.0f);
    outInstanceIndex = gl_VertexIndex;

    vec4 fragPos = shaderData.view * modelMat * vec4(Pos.xyz, 1.0);
    outLightVec = shaderData.lightPos.xyz - fragPos.xyz;
    outViewVec = -fragPos.xyz;
}
