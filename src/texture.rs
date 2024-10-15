use super::utils::*;
use glow::HasContext;
use std::rc::Rc;

pub struct TextureData {
    gl: Rc<glow::Context>,
    id: glow::Texture,
    target: u32,
    width: usize,
    height: usize,
    depth: usize,
    internal_format: u32,
    format: u32,
    texture_type: u32,
}

impl TextureData {
    pub fn new(
        gl: Rc<glow::Context>,
        target: u32,
        width: usize,
        height: usize,
        depth: usize,
        internal_format: u32,
        format: Option<u32>,
        texture_type: Option<u32>,
    ) -> Result<Self, String> {
        unsafe {
            let id = gl.create_texture().map_err(|e| e.to_string())?;
            gl.bind_texture(target, Some(id));

            let format = format.unwrap_or_else(|| get_gl_format_from_internal(internal_format));
            let texture_type =
                texture_type.unwrap_or_else(|| get_gl_type_from_internal(internal_format));

            gl.tex_parameter_i32(target, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
            gl.tex_parameter_i32(target, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
            gl.tex_parameter_i32(target, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(target, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(target, glow::TEXTURE_WRAP_R, glow::CLAMP_TO_EDGE as i32);

            if target == glow::TEXTURE_1D {
                gl.tex_image_1d(
                    target,
                    0,
                    internal_format as i32,
                    width as i32,
                    0,
                    format,
                    texture_type,
                    None,
                );
            } else if target == glow::TEXTURE_2D {
                gl.tex_image_2d(
                    target,
                    0,
                    internal_format as i32,
                    width as i32,
                    height as i32,
                    0,
                    format,
                    texture_type,
                    None,
                );
            } else if target == glow::TEXTURE_3D {
                gl.tex_image_3d(
                    target,
                    0,
                    internal_format as i32,
                    width as i32,
                    height as i32,
                    depth as i32,
                    0,
                    format,
                    texture_type,
                    None,
                );
            } else {
                gl.bind_texture(target, None);
                return Err("Invalid texture target".to_string());
            }

            gl.bind_texture(target, None);

            Ok(Self {
                gl,
                id,
                target,
                width,
                height,
                depth,
                internal_format,
                format,
                texture_type,
            })
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.bind_texture(self.target, Some(self.id));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.bind_texture(self.target, None);
        }
    }

    pub fn set_min_filter(&self, filter: u32) {
        self.bind();
        unsafe {
            self.gl
                .tex_parameter_i32(self.target, glow::TEXTURE_MIN_FILTER, filter as i32);
        }
        self.unbind();
    }

    pub fn set_mag_filter(&self, filter: u32) {
        self.bind();
        unsafe {
            self.gl
                .tex_parameter_i32(self.target, glow::TEXTURE_MAG_FILTER, filter as i32);
        }
        self.unbind();
    }

    pub fn set_wrap_s(&self, wrap: u32) {
        self.bind();
        unsafe {
            self.gl
                .tex_parameter_i32(self.target, glow::TEXTURE_WRAP_S, wrap as i32);
        }
        self.unbind();
    }

    pub fn set_wrap_t(&self, wrap: u32) {
        self.bind();
        unsafe {
            self.gl
                .tex_parameter_i32(self.target, glow::TEXTURE_WRAP_T, wrap as i32);
        }
        self.unbind();
    }

    pub fn set_wrap_r(&self, wrap: u32) {
        self.bind();
        unsafe {
            self.gl
                .tex_parameter_i32(self.target, glow::TEXTURE_WRAP_R, wrap as i32);
        }
        self.unbind();
    }

    pub fn load_data(
        &self,
        data: &[u8],
        width: usize,
        height: usize,
        depth: usize,
        x_offset: usize,
        y_offset: usize,
        z_offset: usize,
    ) {
        self.bind();
        unsafe {
            if (self.target == glow::TEXTURE_1D) {
                self.gl.tex_sub_image_2d(
                    self.target,
                    0,
                    x_offset as i32,
                    0,
                    width as i32,
                    1,
                    self.format,
                    self.texture_type,
                    glow::PixelUnpackData::Slice(data),
                );
            } else if self.target == glow::TEXTURE_2D {
                self.gl.tex_sub_image_2d(
                    self.target,
                    0,
                    x_offset as i32,
                    y_offset as i32,
                    width as i32,
                    height as i32,
                    self.format,
                    self.texture_type,
                    glow::PixelUnpackData::Slice(data),
                );
            } else if self.target == glow::TEXTURE_3D {
                self.gl.tex_sub_image_3d(
                    self.target,
                    0,
                    x_offset as i32,
                    y_offset as i32,
                    z_offset as i32,
                    width as i32,
                    height as i32,
                    depth as i32,
                    self.format,
                    self.texture_type,
                    glow::PixelUnpackData::Slice(data),
                );
            }
        }
        self.unbind();
    }

    pub fn get_id(&self) -> glow::Texture {
        self.id
    }

    pub fn get_target(&self) -> u32 {
        self.target
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_depth(&self) -> usize {
        self.depth
    }

    pub fn get_internal_format(&self) -> u32 {
        self.internal_format
    }

    pub fn get_format(&self) -> u32 {
        self.format
    }

    pub fn get_texture_type(&self) -> u32 {
        self.texture_type
    }
}

impl Drop for TextureData {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_texture(self.id);
        }
    }
}
