#version 140

in vec2 position;
uniform vec2 resolution;
uniform vec2 point_origin;
uniform float radius;

void main() {
    vec2 new_position = position * radius + point_origin;

    vec2 projection = vec2(1.0 / resolution.x, 1.0 / resolution.y);
    gl_Position = vec4(new_position * projection, 0.0, 1.0);
}