use glow::HasContext;
use paxil::AppConfig;

use std::rc::Rc;

use paxil::app_runner::*;
use paxil::shader::*;
use paxil::vao::*;

struct MyApp {
    shader: Shader,
    vao: VAO,
}

impl App for MyApp {
    fn new(gl: Rc<glow::Context>) -> Self {
        let vao = VAO::new(gl.clone());

        let vertex_shader_src = include_str!("shader/test.vert");
        let fragment_shader_src = include_str!("shader/test.frag");
        let shader = Shader::new(gl.clone(), vertex_shader_src, fragment_shader_src)
            .expect("Failed to create shader program");

        Self { shader, vao }
    }

    fn draw(&mut self, gl: &glow::Context) {
        unsafe {
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            self.shader.bind();
            self.vao.bind();
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
            self.vao.unbind();
            self.shader.unbind();
        }
    }
}

fn main() {
    let app_config = AppConfig::default();
    AppRunner::run::<MyApp>(app_config).unwrap();
}
