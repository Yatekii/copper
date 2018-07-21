#version 150 core

in vec2 position;
in vec4 color;
in uint id;
// layout(std140) uniform Locals {
//     mat4 perspective;
// };

 uniform Globals {
    mat4 perspective;
};

struct Attributes {
    mat4 transform;
};

uniform u_attributes { Attributes attributes[1000]; };

out vec4 col;
out vec4 pos;

void main() {
    pos = vec4(position, 0.0, 1.0);
    gl_Position = perspective * attributes[id].transform * pos;
    col = color;
}