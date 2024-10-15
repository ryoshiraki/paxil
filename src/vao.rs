use glow::HasContext;
use std::rc::Rc;

pub struct VAO {
    gl: Rc<glow::Context>,
    vao: glow::VertexArray,
}

impl VAO {
    // VAOの作成
    pub fn new(gl: Rc<glow::Context>) -> Self {
        unsafe {
            let vao = gl.create_vertex_array().expect("Failed to create VAO");
            gl.bind_vertex_array(Some(vao));
            Self { gl, vao }
        }
    }

    // VAOをバインド
    pub fn bind(&self) {
        unsafe {
            self.gl.bind_vertex_array(Some(self.vao));
        }
    }

    // VAOをアンバインド
    pub fn unbind(&self) {
        unsafe {
            self.gl.bind_vertex_array(None);
        }
    }
}

impl Drop for VAO {
    // VAOの削除
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.vao);
        }
    }
}
