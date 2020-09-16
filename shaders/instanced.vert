#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 3) in mat4 aOffset;

uniform mat4 mvp;

out vec4 colorAdd;

void main()
{
    colorAdd = vec4(0.0,0.0,0.0,0.0);
    if (aPos.y == 0.1) {
        colorAdd = vec4(0.0,-0.1,-0.1,0.0);
    }
    gl_Position = mvp * aOffset * vec4(aPos, 1.0);
}