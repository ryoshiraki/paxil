#version 410
const vec2 vertices[4] = vec2[4](
    vec2(-1.0f, - 1.0f),
    vec2(1.0f, - 1.0f),
    vec2(1.0f, 1.0f),
    vec2(-1.0f, 1.0f)
);

out vec2 v_uv;

void main() {
    vec2 v = vertices[gl_VertexID];
    v_uv = v * 0.5 + 0.5;
    gl_Position = vec4(v, 0.0, 1.0);
}