#version 430 core

in VS_OUTPUT {
    vec2 uv;
} vertex;

out vec4 fragColor;

void main() {
    fragColor = vec4(vertex.uv, 1., 1.);
}