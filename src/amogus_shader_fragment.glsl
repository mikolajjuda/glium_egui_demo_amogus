#version 330 core

uniform sampler2DArray tex_amogus;
uniform sampler2DArray tex_amogus_dyeing_mask;
uniform uint anim_frame;

in vec2 v_tex_coords;
out vec4 f_color;


void main(){
	f_color = texture(tex_amogus, vec3(v_tex_coords, anim_frame));
}