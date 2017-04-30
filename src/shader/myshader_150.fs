#version 150 core

in vec2 v_TexCoord;
out vec4 Target0;

//Textures
uniform sampler2D t_Diffuse;
uniform sampler2D t_Normal;
uniform sampler2D t_Specular;

in vec3 FragPos;
in vec3 Normal;


struct DirectionalLight {
  vec4 d_lightDir;
  vec4 d_lightColor;
  float d_lightStrength;
  float _pad1;
  float _pad2;
  bool d_active;

};

struct SpotLight {
  vec4 s_lightPos;
  vec4 s_lightDirection;
  vec4 s_lightColor;
  float s_cutOff;
  float _pad1;
  float _pad2;
  bool s_active;
};

struct PointLight
{
  vec4 p_lightPos;
  vec4 p_lightColor;
  float p_constant;
  float p_linear;
  float p_quadratic;
  float p_lightStrength;
  float _pad1;
  float _pad2;
  float _pad3;
  bool p_active;
};

//might have to change array size
layout (std140) uniform Light_Directional{
  DirectionalLight d_light[2];
};

layout (std140) uniform Light_Spot{
  SpotLight s_light[10];
};

layout (std140) uniform Light_Point{
  PointLight p_light[10];
};

layout (std140) uniform Material {
  float shininess;
  float ambient;
};

layout (std140) uniform Camera {
  vec4 c_viewPos;
};

layout (std140) uniform Light_Info {
  int max_dir_lights;
  int max_spot_lights;
  int max_point_lights;
};

// Function prototypes
vec3 CalcDirLight(DirectionalLight light, vec3 normal, vec3 viewDir);
vec3 CalcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir);
vec3 Nothing();


void main() {
  // Properties
  vec3 norm = normalize(Normal);
  vec3 viewDir = normalize(c_viewPos.xyz - FragPos);

  // == ======================================
  // Our lighting is set up in 3 phases: directional, point lights and an optional flashlight
  // For each phase, a calculate function is defined that calculates the corresponding color
  // per lamp. In the main() function we take all the calculated colors and sum them up for
  // this fragment's final color.
  // == ======================================
  // Phase 1: Directional lighting
  vec3 result = vec3(0.0);

  /*
  for(int i = 0; i < max_dir_lights; i++)
    if (d_light[i].d_active)
      result += CalcDirLight(d_light[i], norm, viewDir);
  */
  // Phase 2: Point lights
  for(int i = 0; i < max_point_lights; i++)
    if (p_light[i].p_active)
    {
      result += CalcPointLight(p_light[i], norm, FragPos, viewDir);
    }
  // Phase 3: Spot light
  // result += CalcSpotLight(spotLight, norm, FragPos, viewDir);
  Target0 = vec4(result, 1.0f);

}

//Do nothing
vec3 Nothing(){
  return vec3(0.0);
}


// Calculates the color when using a directional light.
vec3 CalcDirLight(DirectionalLight light, vec3 normal, vec3 viewDir)
{
    vec3 lightDir = normalize(-light.d_lightDir.xyz);
    // Diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);
    // Specular shading
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
    // Combine results
    vec3 ambient = ambient * vec3(texture(t_Diffuse, v_TexCoord));
    vec3 diffuse = light.d_lightColor.xyz * diff * vec3(texture(t_Diffuse, v_TexCoord));
    vec3 specular = light.d_lightStrength * spec * vec3(texture(t_Specular, v_TexCoord));
    return (ambient + diffuse + specular);
}

// Calculates the color when using a point light.
vec3 CalcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir)
{
  //Point light
  float distance    = length(light.p_lightPos.xyz - FragPos);
  float attenuation = 1.0f / (light.p_constant + light.p_linear * distance +
                              light.p_quadratic * (distance * distance));



  // Ambient
  vec3 ambient = ambient * vec3(texture(t_Diffuse, v_TexCoord));

  // Diffuse
  vec3 norm = normalize(Normal) ;
  vec3 lightDir = normalize(-light.p_lightPos.xyz - FragPos);
  float diff = max(dot(norm, lightDir), 0.0);
  vec3 diffuse = light.p_lightColor.xyz * diff *  vec3(texture(t_Diffuse, v_TexCoord));

  // Specular
  //vec3 viewDir = normalize(c_viewPos.xyz - FragPos);
  vec3 reflectDir = reflect(-lightDir, norm);
  float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
  vec3 specular = light.p_lightStrength * spec * vec3(texture(t_Specular, v_TexCoord));

  //point light pre_stage
  ambient  *= attenuation;
  diffuse  *= attenuation;
  specular *= attenuation;
    return (ambient + diffuse + specular);
}
