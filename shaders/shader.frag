#version 450 core
#extension GL_EXT_nonuniform_qualifier : enable
#extension GL_EXT_debug_printf : enable

layout (set = 1, binding = 0) uniform sampler2D textures[];

layout (location = 0) out vec4 Colour;

layout (location = 3) in vec3 Normal;
layout (location = 4) in vec2 UV;
layout (location = 5) in vec3 Factor;
layout (location = 6) in vec3 LightVec;
layout (location = 7) in vec3 ViewVec;
layout (location = 8) flat in uint InstanceIndex;

void main() {
    float shininess = 10.0;

    vec3 N = normalize(Normal);
    vec3 L = normalize(LightVec);
    vec3 V = normalize(ViewVec);
    vec3 R = reflect(-L, N);
    float diffuse = max(dot(N, L), 0.0);
    float specular = pow(max(dot(R, V), 0.0), shininess) * 0.4;

    vec3 colour = texture(textures[nonuniformEXT(InstanceIndex)], UV).rgb * Factor;
    vec3 outc = vec3(0.0);

    int nq = 4;
    for (int i = nq; i >= 1; i--) {
        float x = (1.0 / nq) * i;
        if (diffuse > x) {
            outc += x * colour;
            break;
        }
    }

    for (int i = nq; i >= 1; i--) {
        float x = (1.0 / nq) * i;
        if (specular > x) {
            outc += vec3(x);
            break;
        }
    }

    Colour = vec4(outc, 1.0);
}
