#version 150 core

in vec2 pos;
out vec4 Target0;

uniform sampler2DMS Render;
uniform GlobalsRender {
    vec4 background_color;
    vec2 grid_size;
};

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
    vec4 color = textureMultisample(Render, texCoord);

    // Apply background color & grid
    if(color.a == 0) {
        vec2 grid = abs(mod(pos, grid_size) - grid_size) / fwidth(pos);
        float line = min(grid.x, grid.y);
        color = vec4(vec3(1.0 - min(line, 1.0)), 1.0);
    }

    Target0 = color;
}