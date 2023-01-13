#version 330 core

uniform sampler2DArray tex_amogus;
uniform sampler2DArray tex_amogus_dyeing_mask;

in vec2 v_tex_coords;
in vec3 v_color;
flat in uint v_anim_frame;

out vec4 f_color;


void main(){
	vec4 amogus = texture(tex_amogus, vec3(v_tex_coords, v_anim_frame));
	vec4 colored_amogus = vec4(v_color, 1.0) * amogus;
	f_color = mix(amogus, colored_amogus, texture(tex_amogus_dyeing_mask, vec3(v_tex_coords, v_anim_frame)));
}