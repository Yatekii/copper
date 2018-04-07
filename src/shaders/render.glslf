#version 150 core

in vec2 pos;
out vec4 Target0;

uniform sampler2DMS Render;

vec4 textureMultisample(sampler2DMS sampler, ivec2 coord)
{
    vec4 color = vec4(0.0);
    int texSamples = 8;

    float totalWeight = 0.0;
    for (int i = 0; i < texSamples; i++) {
        float weight = smoothstep(0.3, 0.7, 1.0 / abs(float(i - texSamples) / 2.0));
        color += weight *texelFetch(sampler, coord, i);
        totalWeight += weight;
    }
    
    return color / totalWeight;
}

void main() {
    ivec2 texSize = textureSize(Render);
    ivec2 texCoord = ivec2((pos + 1.0) / 2.0 * texSize);
    Target0 = textureMultisample(Render, texCoord);
}