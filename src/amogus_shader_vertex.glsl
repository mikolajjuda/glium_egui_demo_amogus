#version 330 core

//per vertex
in vec2 position;
in vec2 tex_coords;

//per instance
in vec3 color;
in mat4 model_matrix;
in uint animation_frame;

out vec2 v_tex_coords;
out vec3 v_color;
flat out uint v_anim_frame;

uniform mat4 projection;

void main() {
	gl_Position = projection * model_matrix * vec4(position, 0.0, 1.0);
	v_tex_coords = tex_coords;
	v_color = color;
	v_anim_frame = animation_frame;
}
