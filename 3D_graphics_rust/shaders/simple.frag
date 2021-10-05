#version 430 core

out vec4 colors;

in smooth vec3 frag_color;
in vec3 normals;


void main()
{
    vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));

    //colors = vec4(frag_color.rgb, 1.0f);
    colors = vec4(frag_color * max(0, dot(normals, -lightDirection)), 1.0f);
}