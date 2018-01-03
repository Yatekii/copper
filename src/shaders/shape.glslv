#version 150 core

in vec2 position;
layout(std140) uniform Locals {
    vec4 color;
    mat3 perspective;
};

void main() {
    vec3 pos = vec3(position, 1.0);
    gl_Position = vec4(perspective * pos, 1.0);
}