#version 430 core

in vec3 v_normal;
in vec4 v_color;

out vec4 color;

void main()
{
    vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));
    color = vec4(v_color.xyz * max(0, dot(v_normal, -lightDirection)), v_color.w);
}