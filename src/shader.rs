use glow::HasContext;
use std::rc::Rc;

pub struct ShaderProgram {
    gl: Rc<glow::Context>,
    program: glow::Program,
}

impl ShaderProgram {
    pub fn new(
        gl: Rc<glow::Context>,
        vertex_shader_src: &str,
        fragment_shader_src: &str,
    ) -> Result<Self, String> {
        unsafe {
            // コンパイルエラーやリンクエラーをキャッチするためのクロージャ
            let compile_shader =
                |gl: &glow::Context, shader_type, src: &str| -> Result<glow::Shader, String> {
                    let shader = gl.create_shader(shader_type)?;
                    gl.shader_source(shader, src);
                    gl.compile_shader(shader);
                    if !gl.get_shader_compile_status(shader) {
                        return Err(gl.get_shader_info_log(shader));
                    }
                    Ok(shader)
                };

            let vertex_shader = compile_shader(&gl, glow::VERTEX_SHADER, vertex_shader_src)?;
            let fragment_shader = compile_shader(&gl, glow::FRAGMENT_SHADER, fragment_shader_src)?;

            // プログラムのリンク
            let program = gl.create_program()?;
            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);

            if !gl.get_program_link_status(program) {
                return Err(gl.get_program_info_log(program));
            }

            // シェーダーオブジェクトはもう不要なので削除
            gl.detach_shader(program, vertex_shader);
            gl.detach_shader(program, fragment_shader);
            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);

            Ok(Self { gl, program })
        }
    }

    // プログラムの使用を開始
    pub fn use_program(&self) {
        unsafe {
            self.gl.use_program(Some(self.program));
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.program);
        }
    }
}
