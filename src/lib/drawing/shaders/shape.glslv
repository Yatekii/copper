#version 150 core

in vec2 position;
in vec4 color;
layout(std140) uniform Locals {
    mat4 perspective;
};
out vec4 col;
out vec4 pos;

void main() {
    pos = vec4(position, 0.0, 1.0);
    gl_Position = perspective * pos;
    col = color;
}