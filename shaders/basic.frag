#version 330 core

in vec2 texCoord;

out vec4 FragColor;
flat in vec4 colorAdd;

uniform bool collision_test;

void main()
{
    // FragColor = vec4(0.5,0.01,0.2,1.0);
    vec4 collision = vec4(0.2, 0.1, 0.1, 0.0) + colorAdd;
    if (collision_test || texCoord.x <= 0.011 || texCoord.y <= 0.01) {
        // collision = vec4(1.0,0.0,0.0,0.0);
    }
    FragColor = collision;
}