use glow::HasContext;
use std::rc::Rc;

use crate::texture;

pub struct Shader {
    gl: Rc<glow::Context>,
    program: glow::Program,
}

impl Shader {
    pub fn new(
        gl: Rc<glow::Context>,
        vertex_shader_src: &str,
        fragment_shader_src: &str,
    ) -> Result<Self, String> {
        unsafe {
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

            let program = gl.create_program()?;
            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);

            if !gl.get_program_link_status(program) {
                return Err(gl.get_program_info_log(program));
            }

            gl.detach_shader(program, vertex_shader);
            gl.detach_shader(program, fragment_shader);
            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);

            Ok(Self { gl, program })
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.use_program(Some(self.program));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.use_program(None);
        }
    }

    pub fn set_uniform_1i(&self, name: &str, value: i32) -> Result<(), String> {
        unsafe {
            let location = self
                .gl
                .get_uniform_location(self.program, name)
                .ok_or_else(|| format!("Uniform '{}' not found", name))?;
            self.gl.uniform_1_i32(Some(&location), value);
        }
        Ok(())
    }

    pub fn set_uniform_2i(&self, name: &str, x: i32, y: i32) -> Result<(), String> {
        unsafe {
            let location = self
                .gl
                .get_uniform_location(self.program, name)
                .ok_or_else(|| format!("Uniform '{}' not found", name))?;
            self.gl.uniform_2_i32(Some(&location), x, y);
        }
        Ok(())
    }

    pub fn set_uniform_3i(&self, name: &str, x: i32, y: i32, z: i32) -> Result<(), String> {
        unsafe {
            let location = self
                .gl
                .get_uniform_location(self.program, name)
                .ok_or_else(|| format!("Uniform '{}' not found", name))?;
            self.gl.uniform_3_i32(Some(&location), x, y, z);
        }
        Ok(())
    }

    pub fn set_uniform_1f(&self, name: &str, value: f32) -> Result<(), String> {
        unsafe {
            let location = self
                .gl
                .get_uniform_location(self.program, name)
                .ok_or_else(|| format!("Uniform '{}' not found", name))?;
            self.gl.uniform_1_f32(Some(&location), value);
        }
        Ok(())
    }

    pub fn set_uniform_2f(&self, name: &str, x: f32, y: f32) -> Result<(), String> {
        unsafe {
            let location = self
                .gl
                .get_uniform_location(self.program, name)
                .ok_or_else(|| format!("Uniform '{}' not found", name))?;
            self.gl.uniform_2_f32(Some(&location), x, y);
        }
        Ok(())
    }

    pub fn set_uniform_3f(&self, name: &str, x: f32, y: f32, z: f32) -> Result<(), String> {
        unsafe {
            let location = self
                .gl
                .get_uniform_location(self.program, name)
                .ok_or_else(|| format!("Uniform '{}' not found", name))?;
            self.gl.uniform_3_f32(Some(&location), x, y, z);
        }
        Ok(())
    }

    pub fn set_uniform_matrix3fv(&self, name: &str, matrix: &[f32]) -> Result<(), String> {
        unsafe {
            let location = self
                .gl
                .get_uniform_location(self.program, name)
                .ok_or_else(|| format!("Uniform '{}' not found", name))?;
            self.gl
                .uniform_matrix_3_f32_slice(Some(&location), false, matrix);
        }
        Ok(())
    }

    pub fn set_uniform_matrix4fv(&self, name: &str, matrix: &[f32]) -> Result<(), String> {
        unsafe {
            let location = self
                .gl
                .get_uniform_location(self.program, name)
                .ok_or_else(|| format!("Uniform '{}' not found", name))?;
            self.gl
                .uniform_matrix_4_f32_slice(Some(&location), false, matrix);
        }
        Ok(())
    }

    pub fn set_uniform_texture(
        &self,
        name: &str,
        unit: u32,
        texture: &glow::Texture,
    ) -> Result<(), String> {
        unsafe {
            let location = self
                .gl
                .get_uniform_location(self.program, name)
                .ok_or_else(|| format!("Uniform '{}' not found", name))?;

            self.gl.active_texture(glow::TEXTURE0 + unit);
            self.gl.bind_texture(glow::TEXTURE_2D, Some(*texture));
            self.gl.uniform_1_i32(Some(&location), unit as i32);
        }
        Ok(())
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.program);
        }
    }
}
