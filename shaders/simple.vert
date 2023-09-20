#version 430 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 color;
uniform layout(location = 2) mat4x4 matrix;

out vec4 v_color;

void main()
{
    vec4 position4 = vec4(position.x, position.y, position.z, 1.0f);
    gl_Position = matrix * position4;
    v_color = color;
}