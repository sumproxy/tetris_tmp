#version 150 core

uniform vec2 u_center;
uniform vec3 u_color;
out vec4 v_Color;

void main() {
    v_Color = vec4(u_color, 1.0);
    gl_Position = vec4(u_center, 0.0, 1.0);
}
