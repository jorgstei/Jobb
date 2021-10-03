#version 430 core

out vec4 colors;

in smooth vec3 frag_color;



void main()
{
    colors = vec4(frag_color.rgb, 1.0f);
}