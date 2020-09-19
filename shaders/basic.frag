#version 330 core

out vec4 FragColor;

in vec2 texCoord;
in vec3 fragPos;
in vec4 colorAdd;
in vec3 normal;

uniform bool collision_test;
uniform vec3 chunkColor;

void main()
{
    
    // TODO: These 3 would come from a uniform or something
    vec3 lightColor = vec3(0.8, 0.8, 0.8);
    vec3 objectColor = vec3(0.2, 0.02, 0.0) + chunkColor;
    vec3 lightPos = vec3(0.0, 882.0, 0.0);
    // TODO:: ambient needs to be calculated
    vec3 ambient = vec3(0.4, 0.4, 0.4);
    
    vec3 lightDir = normalize(lightPos - fragPos);
    
    float theta = max(dot(normal, lightDir), 0.0);
    vec3 diffuse = theta * lightColor;
    vec3 diffusedColor = (ambient + diffuse) * objectColor;

    FragColor = vec4(diffusedColor, 1.0);
}