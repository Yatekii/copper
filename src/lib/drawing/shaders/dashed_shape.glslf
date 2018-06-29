#version 150 core

in vec4 pos;
layout(std140) uniform Locals {
    vec4 color;
    mat4 perspective;
};
out vec4 Target0;

void main() {
    float m11 = perspective[1][1];
    float m22 = perspective[1][1];
    if(floor(pos.x % 0.1) == 0){
        Target0 = color;
    } else {
        Target0 = vec4(color.xyz, 0.0);
    }
}