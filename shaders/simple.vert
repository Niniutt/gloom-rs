#version 430 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec4 color;
uniform layout(location = 3) mat4x4 mvp_matrix;
uniform layout(location = 4) mat4x4 model_matrix;

out vec3 v_normal;
out vec4 v_color;

void main()
{
    vec4 position4 = vec4(position.x, position.y, position.z, 1.0f);
    gl_Position = mvp_matrix * position4;
    v_normal = normalize(mat3(model_matrix) * normal);
    v_color = color;
}