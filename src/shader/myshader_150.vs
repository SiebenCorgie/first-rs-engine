#version 330 core

in vec3 a_Pos;
in vec3 a_Normal;
in vec2 a_TexCoord;
in vec3 a_Color;

out vec3 Normal;
out vec3 FragPos;
out vec2 v_TexCoord;

out int dir_count;
out int spot_count;
out int point_count;

uniform Locals {
	mat4 u_Model_Transform;
	mat4 u_Projection;
	mat4 u_View;
};

layout (std140) uniform Light_Info {
  int max_dir_lights;
  int max_spot_lights;
  int max_point_lights;
};


void main() {

    v_TexCoord = a_TexCoord;
		FragPos = vec3(u_Model_Transform * vec4(a_Pos, 1.0));
		Normal = mat3(transpose(inverse(u_Model_Transform))) * a_Normal;

		dir_count = max_dir_lights;
		spot_count = max_spot_lights;
		point_count = max_point_lights;


		gl_Position = u_Projection * u_View * u_Model_Transform * vec4(a_Pos, 1.0);


		gl_ClipDistance[0] = 1.0;
}
