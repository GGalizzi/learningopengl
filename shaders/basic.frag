#version 330 core

in vec2 texCoord;

out vec4 FragColor;
in vec4 colorAdd;

uniform bool collision_test;

void main()
{
    FragColor = colorAdd;
}