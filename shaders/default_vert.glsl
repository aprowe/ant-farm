#version 330

uniform float scale;
uniform vec2 pos;
uniform float theta;
uniform vec3 color;

in vec2 position;

out vec3 vert_color;
out vec2 uv;

mat2 rotate(float angle) {
	float s = cos(angle);
	float c = sin(angle);

	return mat2(
		c, -s,
		s, c
	);
}

void main() {
    vec2 xy = position * rotate(theta) * scale + pos;
    gl_Position = vec4((xy - 0.5) * 2.0, 0.0, 1.0);

    // Exports
    vert_color = color;
    uv = position;
}
