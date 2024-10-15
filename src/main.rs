use glow::HasContext;
use std::rc::Rc; // これを追加

use paxil::app_runner::*;
use paxil::shader::*;
use paxil::vao::*;

struct MyApp {
    shader_program: ShaderProgram,
    vao: VAO,
}

impl App for MyApp {
    fn new(gl: Rc<glow::Context>) -> Self {
        let vao = VAO::new(gl.clone());

        let (vertex_shader_source, fragment_shader_source) = (
            r#"
            #version 410
            const vec2 verts[3] = vec2[3](
                vec2(0.5f, 1.0f),
                vec2(0.0f, 0.0f),
                vec2(1.0f, 0.0f)
            );
            out vec2 vert;
            void main() {
                vert = verts[gl_VertexID];
                gl_Position = vec4(vert - 0.5, 0.0, 1.0);
            }"#,
            r#"
            #version 410
            precision mediump float;
            in vec2 vert;
            out vec4 color;
            void main() {
                color = vec4(vert, 0.5, 1.0);
            }"#,
        );

        let shader_program =
            ShaderProgram::new(gl.clone(), vertex_shader_source, fragment_shader_source)
                .expect("Failed to create shader program");

        Self {
            shader_program,
            vao,
        }
    }

    fn draw(&mut self, gl: &glow::Context) {
        unsafe {
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            self.shader_program.use_program();
            self.vao.bind();
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
            self.vao.unbind();
        }
    }
}

fn main() {
    AppRunner::start::<MyApp>().unwrap();
}
