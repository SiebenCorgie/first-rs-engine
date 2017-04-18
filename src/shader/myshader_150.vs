#version 150 core

in vec4 a_Pos;
in vec2 a_TexCoord;
out vec2 v_TexCoord;

uniform Locals {
	mat4 u_Model_Transform;
	mat4 u_Projection;
	mat4 u_View;

};

void main() {
    v_TexCoord = a_TexCoord;
    gl_Position = u_Projection * u_View * u_Model_Transform * a_Pos;
    gl_ClipDistance[0] = 1.0;
}
