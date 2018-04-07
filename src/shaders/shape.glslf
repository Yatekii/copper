#version 150 core

layout(std140) uniform Locals {
    mat4 perspective;
};
in vec4 col;
out vec4 Target1;

void main() {
    Target1 = col;
}