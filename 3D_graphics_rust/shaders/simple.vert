#version 430 core

layout(location=0) in vec3 positions;
layout(location=1) in vec4 colors;
layout(location=2) uniform mat4 mvp;
//layout(location=3) uniform float oscilator;
layout(location=4) in vec3 unit_vectors;
layout(location=5) uniform mat4 model;

out smooth vec4 frag_color;
out vec3 normals;

/*
    a, b, 0, c
    d, e, 0, f,
    0, 0, 1, 0,
    0, 0, 0, 1
*/
void main()
{ 
    normals = normalize(mat3(model) * unit_vectors);
    //normals = unit_vectors;
    frag_color = colors;
    //mat4 oscilating_matrix = mvp;
    //oscilating_matrix[1][0] = oscilator;
    gl_Position = mvp*vec4(positions, 1.0f);
    
}