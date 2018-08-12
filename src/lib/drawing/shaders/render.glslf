#version 150 core

in vec2 pos;
out vec4 Target0;

uniform sampler2DMS Render;
uniform GlobalsRender {
    vec4 background_color;
    vec2 grid_size;
    vec2 grid_origin;
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
    vec4 texColor = textureMultisample(Render, texCoord);
    vec4 line_color = vec4(0.0, 0.0, 0.0, 1.0);

    // Apply background color & grid
    vec2 realPos = grid_origin + pos;
    vec2 grid = abs(mod(realPos, grid_size) - 0.5 * grid_size) / fwidth(realPos);
    float line = min(grid.x, grid.y);
    
    vec4 color = mix(background_color, line_color, 1.0 - min(line, 1.0));

    color = vec4(mix(color.xyz, texColor.xyz, texColor.a), 1.0);

    Target0 = color;
}