#version 430 core

layout(location=0) in vec3 positions;
layout(location=1) in vec3 colors;
layout(location=2) uniform mat4 transform;
layout(location=3) uniform float oscilator;

out smooth vec3 frag_color;

/*
    a, b, 0, c
    d, e, 0, f,
    0, 0, 1, 0,
    0, 0, 0, 1
*/
void main()
{
    
    frag_color = colors;
    mat4 oscilating_matrix = transform;
    //oscilating_matrix[1][0] = oscilator;
    gl_Position = oscilating_matrix*vec4(positions, 1.0f);
}