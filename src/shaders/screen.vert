#version 430 core

layout (location = 0) in vec2 position;
layout (location = 1) in vec2 texCoord;

out VS_OUTPUT {
    vec2 uv;
} vertex;

void main() {
    gl_Position = vec4(position, 1., 1.);
    vertex.uv = texCoord;
}