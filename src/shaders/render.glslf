#version 150 core

in vec2 pos;
out vec4 Target0;

uniform sampler2DMS Render;

vec4 textureMultisample(sampler2DMS sampler, ivec2 coord)
{
    vec4 color = vec4(0.0);
    int texSamples = 8;

    for (int i = 0; i < texSamples; i++)
        color += texelFetch(sampler, coord, i);

    return color / float(texSamples);
}

void main() {
    ivec2 texSize = textureSize(Render);
    ivec2 texCoord = ivec2((pos + 1.0) / 2.0 * texSize);
    Target0 = textureMultisample(Render, texCoord);
}