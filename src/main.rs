use glow::HasContext;
use paxil::AppConfig;

use std::rc::Rc;

use paxil::app_runner::*;
use paxil::image::*;
use paxil::shader::*;
use paxil::texture::*;
use paxil::vao::*;

struct MyApp {
    shader: Shader,
    vao: VAO,
    image: Image,
}

#[allow(unused_must_use)]
impl App for MyApp {
    fn new(gl: Rc<glow::Context>) -> Self {
        let vao = VAO::new(gl.clone());

        let image = Image::load(gl.clone(), "test.jpg").unwrap();

        let vertex_shader_src = include_str!("shader/fill.vert");
        let fragment_shader_src = include_str!("shader/fill.frag");
        let shader = Shader::new(gl.clone(), vertex_shader_src, fragment_shader_src)
            .expect("Failed to create shader program");

        Self { shader, vao, image }
    }

    fn draw(&mut self, gl: &glow::Context) {
        unsafe {
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            self.shader.bind();
            self.shader
                .set_uniform_texture("u_texture", 0, &self.image.texture.get_id())
                .unwrap();
            self.shader.set_uniform_1f("u_b", 0.0);
            self.vao.bind();
            gl.draw_arrays(glow::TRIANGLE_FAN, 0, 4);
            self.vao.unbind();
            self.shader.unbind();
        }
    }
}

fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    let app_config = AppConfig::default();
    AppRunner::run::<MyApp>(app_config).unwrap();
}
