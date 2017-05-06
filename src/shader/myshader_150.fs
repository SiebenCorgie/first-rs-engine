#version 150 core

#define MAX_DIR_LIGHTS 1
#define MAX_POINT_LIGHTS 6
#define MAX_SPOT_LIGHTS 1

const float kPi = 3.14159265;

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
  float s_outerCutOff;
  float s_constant;
  float s_linear;
  float s_quadratic;
  float _pad1;
  float _pad2;
  //float _pad3;
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
  DirectionalLight d_light[MAX_DIR_LIGHTS];
};

layout (std140) uniform Light_Spot{
  SpotLight s_light[MAX_SPOT_LIGHTS];
};

layout (std140) uniform Light_Point{
  PointLight p_light[MAX_POINT_LIGHTS];
};

layout (std140) uniform Material {
  float shininess;
  float ambient;
  float diffuse_intensity;
  float specular;
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
vec3 CalcSpotLight(SpotLight light, vec3 normal, vec3 fragPos, vec3 viewDir);


void main() {
  // Properties
  vec3 norm = normalize(Normal);
  vec3 viewDir = normalize(c_viewPos.xyz - FragPos);


  vec3 result = vec3(0.0);



  //Directional
  for(int i = 0; i < MAX_DIR_LIGHTS; i++)
    if (d_light[i].d_active)
    {
      result += CalcDirLight(d_light[i], norm, viewDir);
    }

  // Phase 2: Point lights
  for(int i = 0; i < MAX_POINT_LIGHTS; i++)
    if (p_light[i].p_active)
    {
      result += CalcPointLight(p_light[i], norm, FragPos, viewDir);
    }
  // Phase 3: Spot light
  for(int i = 0; i < MAX_SPOT_LIGHTS; i++)
    if (s_light[i].s_active)
    {
      result += CalcSpotLight(s_light[i], norm, FragPos, viewDir);
    }

  //Gamma Correction
  float gamma = 2.2;
  result = pow(result, vec3(gamma));

  //Present
  Target0 = vec4(result, 1.0);

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
  //Specular
  //Snacked the EnergyConservation from rorydriscoll.com
  float kEnergyConservation = ( 8.0 + shininess ) / ( 8.0 * kPi );
  vec3 halfwayDir = normalize(lightDir + viewDir);
  float spec = kEnergyConservation * pow(max(dot(normal, halfwayDir), 0.0), shininess);

  // Combine results
  vec3 ambient = ambient * vec3(texture(t_Diffuse, v_TexCoord));
  vec3 diffuse = light.d_lightColor.xyz * diff * vec3(texture(t_Diffuse, v_TexCoord));
  vec3 specular = specular * spec * vec3(texture(t_Specular, v_TexCoord));
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
  //Snacked the EnergyConservation from rorydriscoll.com
  float kEnergyConservation = ( 8.0 + shininess ) / ( 8.0 * kPi );
  vec3 halfwayDir = normalize(lightDir + viewDir);
  float spec = kEnergyConservation * pow(max(dot(normal, halfwayDir), 0.0), shininess);
  vec3 specular = specular * spec * vec3(texture(t_Specular, v_TexCoord));

  //point light pre_stage
  ambient  *= attenuation;
  diffuse  *= attenuation;
  specular *= attenuation;
    return (ambient + diffuse + specular);
}


vec3 CalcSpotLight(SpotLight light, vec3 normal, vec3 fragPos, vec3 viewDir){
    // Ambient
  vec3 ambient = ambient * vec3(texture(t_Diffuse, v_TexCoord));

  // Diffuse
  vec3 norm = normalize(Normal);
  vec3 lightDir = normalize(light.s_lightPos.xyz - FragPos);
  float diff = max(dot(norm, lightDir), 0.0);
  vec3 diffuse = light.s_lightColor.xyz * diff * vec3(texture(t_Diffuse, v_TexCoord));

  // Specular shading blinn
  //Snacked the EnergyConservation from rorydriscoll.com
  float kEnergyConservation = ( 8.0 + shininess ) / ( 8.0 * kPi );
  vec3 halfwayDir = normalize(lightDir + viewDir);
  float spec = kEnergyConservation * pow(max(dot(normal, halfwayDir), 0.0), shininess);
  vec3 specular = specular * spec * vec3(texture(t_Specular, v_TexCoord));

  // Spotlight (soft edges)
  float theta = dot(lightDir, normalize(-light.s_lightDirection.xyz));
  float epsilon = (light.s_cutOff - light.s_outerCutOff);
  float intensity = clamp((theta - light.s_outerCutOff) / epsilon, 0.0, 1.0);
  diffuse  *= intensity;
  specular *= intensity;

  // Attenuation
  float distance    = length(light.s_lightPos.xyz - FragPos);
  float attenuation = 1.0f / (light.s_constant + light.s_linear * distance +
                              light.s_quadratic * (distance * distance));
  ambient  *= attenuation;
  diffuse  *= attenuation;
  specular *= attenuation;
  //Return ambient if not in angle
    return(ambient+ diffuse + specular);
}
