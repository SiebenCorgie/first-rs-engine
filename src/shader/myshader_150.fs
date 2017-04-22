#version 150 core

in vec2 v_TexCoord;
out vec4 Target0;
uniform sampler2D t_Color;

in vec3 FragPos;
in vec3 Normal;

struct Light_struct {
  vec3 lightPos;
  vec3 viewPos;
  vec3 lightColor;
  vec3 objectColor;
  float pad;
};

layout (std140) uniform Lights {
  Light_struct light;
};


void main() {
    // Ambient
    float ambientStrength = 0.1f;
    vec3 ambient = ambientStrength * light.lightColor;

    // Diffuse
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(light.lightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * light.lightColor;

    // Specular
    float specularStrength = 0.5f;
    vec3 viewDir = normalize(light.viewPos - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specular = specularStrength * spec * light.lightColor;

    vec3 result = (ambient + diffuse + specular) * light.objectColor;

    //vec4 tex = texture(t_Color, v_TexCoord);
    //float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
    Target0 = vec4(result, 1.0f);
}
