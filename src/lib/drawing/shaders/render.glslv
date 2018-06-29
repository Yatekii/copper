#version 150 core

in vec2 position;
out vec2 pos;

void main() {
    pos = position;
    gl_Position = vec4(position, 0.0, 1.0);
}