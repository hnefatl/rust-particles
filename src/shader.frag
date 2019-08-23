#version 140

out vec4 color;
uniform vec2 resolution;
uniform vec2 point_origin;
uniform float radius;
uniform vec3 colour;

float light_falloff_function(float dist) {
    return pow(dist / radius, 0.1);
}

void main() {
    vec2 fragment_pos = 2 * (gl_FragCoord.xy - resolution / 2.0);
    float opacity = 1.0 - light_falloff_function(length(fragment_pos - point_origin));
    color = vec4(colour, clamp(0.0, opacity, 1.0));
}