#version 330 core

in vec3 a_Pos;
in vec3 a_Normal;
in vec2 a_TexCoord;
in vec3 a_Color;
in vec3 a_Tangent;

out vec3 Normal;
out vec3 FragPos;
out vec2 v_TexCoord;
out mat3 TBN;

uniform Locals {
	mat4 u_Model_Transform;
	mat4 u_Projection;
	mat4 u_View;
};




void main() {

    v_TexCoord = a_TexCoord;
		FragPos = vec3(u_Model_Transform * vec4(a_Pos, 1.0));
		Normal = mat3(transpose(inverse(u_Model_Transform))) * a_Normal;

		vec3 T = normalize(vec3(u_Model_Transform * vec4(a_Tangent, 0.0)));
		vec3 N = normalize(vec3(u_Model_Transform * vec4(a_Normal, 0.0)));
		// re-orthogonalize T with respect to N
		T = normalize(T - dot(T, N) * N);
		// then retrieve perpendicular vector B with the cross product of T and N
		vec3 B = cross(N, T);

		TBN = mat3(T, B, N);

		gl_Position = u_Projection * u_View * u_Model_Transform * vec4(a_Pos, 1.0);


		gl_ClipDistance[0] = 1.0;
}
