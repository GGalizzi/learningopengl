#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

uniform mat4 mvp;
uniform mat4 model;

out vec4 colorAdd;
out vec3 fragPos;
out vec3 normal;

void main()
{
    colorAdd = vec4(aNormal, 1.0);
    fragPos = vec3(model * vec4(aPos, 1.0));
    normal = vec3(model * vec4(aNormal, 0.0));
    gl_Position = mvp * vec4(aPos, 1.0);
}