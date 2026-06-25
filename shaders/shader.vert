#version 450 core
#extension GL_EXT_debug_printf : enable

layout (std140, set = 0, binding = 0) uniform ShaderData {
    mat4 projection;
    mat4 view;
    mat4 model[5];
    vec4 lightPos;
    uint selected;
    float shininess;
} shaderData;

layout (location = 0) in vec3 Pos; 
layout (location = 1) in vec3 Normal;
layout (location = 2) in vec2 UV;

layout (location = 3) out vec3 outNormal;
layout (location = 4) out vec2 outUV;
layout (location = 5) out vec3 outFactor;
layout (location = 6) out vec3 outLightVec;
layout (location = 7) out vec3 outViewVec;
layout (location = 8) out uint outInstanceIndex;
layout (location = 9) out float outShininess;


void main() {
    mat4 modelMat = shaderData.model[gl_InstanceIndex];

    outNormal = mat3(shaderData.view * modelMat) * Normal;
    outUV = UV;
    
    gl_Position = shaderData.projection * shaderData.view * modelMat * vec4(Pos.xyz, 1.0);

    outFactor = vec3(shaderData.selected == gl_InstanceIndex ? 3.0f : 1.0f);

    outInstanceIndex = gl_InstanceIndex;

    vec4 fragPos = shaderData.view * modelMat * vec4(Pos.xyz, 1.0);
    outLightVec = shaderData.lightPos.xyz - fragPos.xyz;
    outViewVec = -fragPos.xyz;
    outShininess = shaderData.shininess;
}
