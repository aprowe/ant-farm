#version 330
in vec3 vert_color;
in vec2 uv;
out vec4 f_color;

vec3 render() {
    if (uv.x > 0 && abs(uv.y) < 0.1) {
        return vec3(0.0, 0.0, 0.0);
    }

    return vert_color;
}

void main() {
    f_color = vec4(render(), 1.0);
}
