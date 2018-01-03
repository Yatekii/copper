#version 150 core

layout(std140) uniform Locals {
    vec4 color;
    mat3 perspective;
};
out vec4 Target0;

void main() {
    Target0 = color;
}