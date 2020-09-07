#version 330 core

in vec2 texCoord;

out vec4 FragColor;

uniform sampler2D u_texture;

void main()
{
    // FragColor = vec4(0.5,0.01,0.2,1.0);
    FragColor = texture(u_texture, texCoord);
}