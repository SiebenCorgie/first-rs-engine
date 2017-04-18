#version 150 core

uniform Background {
  vec4 bg_color;
};


out vec4 v_Color;


void main() {
  v_Color = bg_color;
}
