#version 450 core
#extension GL_EXT_nonuniform_qualifier : enable
#extension GL_EXT_debug_printf : enable

// pursue the following
// https://docs.mesa3d.org/spirv/index.html

layout (set = 1, binding = 0) uniform sampler2D textures[];

layout (location = 0) out vec4 Colour;

layout (location = 3) in vec3 Normal;
layout (location = 4) in vec2 UV;
layout (location = 5) in vec3 Factor;
layout (location = 6) in vec3 LightVec;
layout (location = 7) in vec3 ViewVec;
layout (location = 8) flat in uint InstanceIndex;

void main() {
    vec3 N = normalize(Normal);
    vec3 L = normalize(LightVec);
    vec3 V = normalize(ViewVec);
    vec3 R = reflect(-L, N);
    vec3 diffuse = vec3(max(dot(N, L), 0.0025));
    vec3 specular = vec3(pow(max(dot(R, V), 0.0), 16.0) * 0.75);

    vec3 color = texture(textures[nonuniformEXT(InstanceIndex)], UV).rgb * Factor;

    Colour = vec4(diffuse * color.rgb + specular, 1.0);
}
