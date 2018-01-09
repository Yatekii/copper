#version 150 core

layout(std140) uniform Locals {
    mat4 perspective;
};
in vec4 col;
out vec4 Target0;

void main() {
    Target0 = col;
}