#version 410
precision mediump float;

in vec2 v_uv;
out vec4 f_col;

uniform sampler2D u_texture;
uniform float u_b;

void main() {
    vec4 tex = texture(u_texture, v_uv);
    f_col =  tex + vec4(v_uv, u_b, 1.0);
}