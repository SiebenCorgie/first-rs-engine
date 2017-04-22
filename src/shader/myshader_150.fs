#version 330 core

in vec2 v_TexCoord;
in vec3 FragPos;
in vec3 Normal;

out vec4 Target0;
uniform sampler2D t_Color;

uniform Camera {
  vec3 viewPos;
};

uniform Simple_Light {
	vec3 lightPos;
};

void main() {

    vec4 tex = texture(t_Color, v_TexCoord);

    float specularStrength = 0.5f;
    vec3 lightColor = vec3(1.0);
    vec3 objectColor = vec3(tex.r, tex.g, tex.b);
    // Ambient
    float ambientStrength = 0.1f;
    vec3 ambient = ambientStrength * lightColor;

    // Diffuse
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;

    //Specular
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    //32 is shininess
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specular = specularStrength * spec * lightColor;


    vec3 result = (ambient + diffuse + specular) * objectColor;




    //float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
    Target0 = vec4(result, 1.0);
}
