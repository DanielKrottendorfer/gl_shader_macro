# gl_shader_macro

rust macro for generating structs that abstract OpenGL shader functionality.

```rust
shader_program!(
    Color3DLight{
        uniform mat4 MVP;
        uniform mat4 M;
        uniform vec3 col;
        uniform vec3 light_position;
        uniform float light_power;
    }
);

let program:u32 = ...

let mut color_3d_light = Color3DLight::new();
color_3d_light.setup(&program);

color_3d_light.use_program();
color_3d_light.set_MVP(mat4(...));
color_3d_light.set_M(mat4(...));
color_3d_light.set_col(vec3(...));
color_3d_light.set_light_position(vec3(...));
color_3d_light.set_light_power(1000.0 as f32);

```
