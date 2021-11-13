#version 330
in vec3 vert_color;
in vec2 uv;

out vec4 f_color;

uniform int PASS_NUM;

uniform sampler2D TEX;

vec2 uv_norm = (uv + 1.0) / 2.0;

vec3 render() {
    if (PASS_NUM == 0) {
        vec2 textureSize2d = textureSize(TEX,0);
        vec2 texelSize = vec2(1.0) / textureSize2d;
        vec4 value =
            texture2D(TEX, uv_norm + vec2(-1.0, 0.0) * texelSize) +
            texture2D(TEX, uv_norm + vec2(0.0, 1.0) * texelSize) +
            texture2D(TEX, uv_norm + vec2(0.0, -1.0) * texelSize) +
            texture2D(TEX, uv_norm + vec2(1.0, 0.0) * texelSize);

        vec4 cur = texture2D(TEX, uv_norm);
        float dt = 0.80;
        vec3 output = ((1 - dt) * cur + dt * value / 4.0).xyz;

        return output * 0.994;
    } else {
        return texture(TEX, (uv + 1.0) / 2.0).xyz * 10;
        /* return vec3(0.0, 1.0, 0.0); */
    }
}

void main() {
    f_color = vec4(render(), 1.0);
}
