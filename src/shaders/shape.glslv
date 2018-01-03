#version 150 core

in vec2 position;
layout(std140) uniform Locals {
    vec4 color;
    mat4 perspective;
};

void main() {
    vec4 pos = vec4(position, 0.0, 1.0);
    gl_Position = perspective * pos;
}