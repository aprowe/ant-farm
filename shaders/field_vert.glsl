#version 330

uniform int PASS_NUM;

in vec2 position;

out vec3 vert_color;
out vec2 uv;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);

    // Exports
    vert_color = vec3(0.0, 1.0, 0.0);
    uv = position;
}
