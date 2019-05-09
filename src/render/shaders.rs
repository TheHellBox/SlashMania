pub const SHADER_SIMPLE_FRAG: &'static str = r#"
#version 140
in vec3 v_normal;
in vec2 v_tex_coords;

out vec4 color;

uniform sampler2D tex;

void main() {
    vec3 u_light = vec3(0.1,0.1,0.4);
    float brightness = dot(normalize(v_normal), normalize(u_light));

    vec4 dark_color = vec4(0.7, 0.7, 0.7, 1.0) * vec4(texture(tex, v_tex_coords));
    vec4 regular_color = vec4(1.0, 1.0, 1.0, 1.0) * vec4(texture(tex, v_tex_coords));

    color = vec4(mix(dark_color, regular_color, brightness));
}
"#;

pub const SHADER_SIMPLE_VERT: &'static str = r#"
#version 140

in vec3 position;
in vec2 tex_coords;
in vec3 normal;

out vec2 v_tex_coords;
out vec3 v_normal;

uniform mat4 projection;
uniform mat4 transform;
uniform mat4 view;

void main() {
    mat4 modelview = view * transform;
    gl_Position = projection * modelview * vec4(position, 1.0);
    v_tex_coords = tex_coords;
    v_normal = normal;
}
"#;

pub const SHADER2D_SIMPLE_FRAG: &'static str = r#"
#version 140
in vec2 v_tex_coords;

out vec4 color;

uniform sampler2DArray tex;

void main() {
    color = vec4(texture(tex, vec3(v_tex_coords, 0)));
}
"#;

pub const SHADER2D_SIMPLE_VERT: &'static str = r#"
#version 140

in vec2 position;
in vec2 tex_coords;

out vec2 v_tex_coords;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_tex_coords = tex_coords;
}
"#;
