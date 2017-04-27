#version 150 core

in vec2 v_TexCoord;
out vec4 Target0;

//Textures
uniform sampler2D t_Diffuse;
uniform sampler2D t_Normal;
uniform sampler2D t_Specular;

in vec3 FragPos;
in vec3 Normal;

struct Light_struct {
  vec4 lightPos;
  vec4 viewPos;
  vec4 lightColor;
  vec4 objectColor;
};

layout (std140) uniform Lights {
  vec4 lightPos;
  vec4 viewPos;
  vec4 lightColor;
  float lightStrength;
};

layout (std140) uniform Material {
  float shininess;
  float ambient;
};

void main() {
    // Ambient
    vec3 ambient = ambient * vec3(texture(t_Diffuse, v_TexCoord));

    // Diffuse
    vec3 norm = normalize(Normal) ;
    vec3 lightDir = normalize(lightPos.xyz - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = lightColor.xyz * diff *  vec3(texture(t_Diffuse, v_TexCoord));

    // Specular
    vec3 viewDir = normalize(viewPos.xyz - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
    vec3 specular = lightStrength * spec * vec3(texture(t_Specular, v_TexCoord));

    vec3 result = ambient + diffuse + specular;

    //vec4 tex = texture(t_Color, v_TexCoord);
    //float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
    Target0 = vec4(result, 1.0f);
}
