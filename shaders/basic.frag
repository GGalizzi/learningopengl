#version 330 core

in vec2 texCoord;

out vec4 FragColor;

uniform sampler2D u_texture;
uniform bool collision_test;

void main()
{
    // FragColor = vec4(0.5,0.01,0.2,1.0);
    vec4 collision = vec4(0.0, 0.0, 0.0, 0.0);
    if (collision_test || texCoord.x <= 0.011 || texCoord.y <= 0.01) {
        collision = vec4(1.0,0.0,0.0,0.0);
    }
    FragColor = texture(u_texture, texCoord) + collision;
}