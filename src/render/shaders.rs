#![allow(unused)]
pub const SHADER_SIMPLE_FRAG: &'static str = r#"
#version 140
in vec3 v_normal;
in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;

void main() {
    color = vec4(texture(tex, v_tex_coords));
}
"#;

pub const SHADER_SIMPLE_VERT: &'static str = r#"
#version 330

in vec3 position;
in vec3 normal;
in vec2 tex_coords;
out vec3 v_normal;
out vec2 v_tex_coords;
uniform mat4 projection;
uniform mat4 transform;
uniform mat4 view;
void main() {
    mat4 modelview = view * transform;
    v_normal = normal;
    gl_Position = projection * modelview * vec4(position, 1.0);
    v_tex_coords = tex_coords;
}
"#;
