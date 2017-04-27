#version 150 core

in vec2 v_TexCoord;
out vec4 Target0;

//Textures
uniform sampler2D t_Diffuse;
uniform sampler2D t_Normal;
uniform sampler2D t_Specular;

in vec3 FragPos;
in vec3 Normal;

layout (std140) uniform Light_Directional {
  vec4 d_lightPos;
  vec4 d_lightColor;
  float d_lightStrength;
};

layout (std140) uniform Light_Spot{
  vec4 s_lightPos;
};

layout (std140) uniform Light_Point{
  vec4 p_lightPos;
  vec4 p_lightColor;
  float p_constant;
  float p_linear;
  float p_quadratic;
  float p_lightStrength;
};

layout (std140) uniform Material {
  float shininess;
  float ambient;
};

layout (std140) uniform Camera {
  vec4 c_viewPos;
};

void main() {
    //Point light
    float distance    = length(p_lightPos.xyz - FragPos);
    float attenuation = 1.0f / (p_constant + p_linear * distance +
        		                    p_quadratic * (distance * distance));



    // Ambient
    vec3 ambient = ambient * vec3(texture(t_Diffuse, v_TexCoord));

    // Diffuse
    vec3 norm = normalize(Normal) ;
    //vec3 lightDir = normalize(d_lightPos.xyz - FragPos);
    vec3 lightDir = normalize(-p_lightPos.xyz - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = d_lightColor.xyz * diff *  vec3(texture(t_Diffuse, v_TexCoord));

    // Specular
    vec3 viewDir = normalize(c_viewPos.xyz - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
    vec3 specular = d_lightStrength * spec * vec3(texture(t_Specular, v_TexCoord));

    //point light pre_stage
    ambient  *= attenuation;
    diffuse  *= attenuation;
    specular *= attenuation;


    vec3 result = ambient + diffuse + specular;

    Target0 = vec4(result, 1.0f);
}
