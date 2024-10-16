use super::utils::*;
use glow::HasContext;
use std::rc::Rc;
pub trait TextureTrait {
    fn get_texture_data(&self) -> &TextureData;

    fn get_context(&self) -> Rc<glow::Context> {
        self.get_texture_data().gl.clone()
    }

    fn get_id(&self) -> glow::Texture {
        self.get_texture_data().id
    }

    fn get_target(&self) -> u32 {
        self.get_texture_data().target
    }

    fn get_width(&self) -> usize {
        self.get_texture_data().width
    }

    fn get_height(&self) -> usize {
        self.get_texture_data().height
    }

    fn get_depth(&self) -> usize {
        self.get_texture_data().depth
    }

    fn get_internal_format(&self) -> u32 {
        self.get_texture_data().internal_format
    }

    fn get_format(&self) -> u32 {
        self.get_texture_data().format
    }

    fn get_texture_type(&self) -> u32 {
        self.get_texture_data().texture_type
    }

    fn bind(&self) {
        unsafe {
            self.get_context()
                .bind_texture(self.get_target(), Some(self.get_id()));
        }
    }
    fn unbind(&self) {
        unsafe {
            self.get_context().bind_texture(self.get_target(), None);
        }
    }
    fn set_min_filter(&self, filter: u32) {
        self.bind();
        unsafe {
            self.get_context().tex_parameter_i32(
                self.get_target(),
                glow::TEXTURE_MIN_FILTER,
                filter as i32,
            );
        }
        self.unbind();
    }
    fn set_mag_filter(&self, filter: u32) {
        self.bind();
        unsafe {
            self.get_context().tex_parameter_i32(
                self.get_target(),
                glow::TEXTURE_MAG_FILTER,
                filter as i32,
            );
        }
        self.unbind();
    }
    fn set_wrap_s(&self, wrap: u32) {
        self.bind();
        unsafe {
            self.get_context().tex_parameter_i32(
                self.get_target(),
                glow::TEXTURE_WRAP_S,
                wrap as i32,
            );
        }
        self.unbind();
    }
    fn set_wrap_t(&self, wrap: u32) {
        self.bind();
        unsafe {
            self.get_context().tex_parameter_i32(
                self.get_target(),
                glow::TEXTURE_WRAP_T,
                wrap as i32,
            );
        }
        self.unbind();
    }
    fn set_wrap_r(&self, wrap: u32) {
        self.bind();
        unsafe {
            self.get_context().tex_parameter_i32(
                self.get_target(),
                glow::TEXTURE_WRAP_R,
                wrap as i32,
            );
        }
        self.unbind();
    }
}

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

#[allow(unused, clippy::too_many_arguments)]
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
        data: Option<&[u8]>,
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
                    data,
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
                    data,
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
                    data,
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

    pub fn load_data(
        &self,
        data: &[u8],
        x_offset: usize,
        y_offset: usize,
        z_offset: usize,
        width: usize,
        height: usize,
        depth: usize,
    ) {
        unsafe {
            self.gl.bind_texture(self.target, Some(self.id));

            if (self.target == glow::TEXTURE_1D) {
                log::error!("Texture 1D not supported");
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

            self.gl.bind_texture(self.target, None);
        }
    }
}

impl Drop for TextureData {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_texture(self.id);
        }
    }
}

pub struct Texture1D {
    data: TextureData,
}

impl Texture1D {
    pub fn new(
        gl: Rc<glow::Context>,
        width: usize,
        internal_format: u32,
        format: Option<u32>,
        texture_type: Option<u32>,
        data: Option<&[u8]>,
    ) -> Result<Self, String> {
        let data = TextureData::new(
            gl.clone(),
            glow::TEXTURE_1D,
            width,
            1,
            1,
            internal_format,
            format,
            texture_type,
            data,
        )?;
        Ok(Self { data })
    }

    pub fn load_data(&self, data: &[u8], width: usize, x_offset: usize) {
        self.data.load_data(data, x_offset, 0, 0, width, 1, 1);
    }
}

impl TextureTrait for Texture1D {
    fn get_texture_data(&self) -> &TextureData {
        &self.data
    }
}

pub struct Texture2D {
    data: TextureData,
}

impl Texture2D {
    pub fn new(
        gl: Rc<glow::Context>,
        width: usize,
        height: usize,
        internal_format: u32,
        format: Option<u32>,
        texture_type: Option<u32>,
        data: Option<&[u8]>,
    ) -> Result<Self, String> {
        let data = TextureData::new(
            gl.clone(),
            glow::TEXTURE_2D,
            width,
            height,
            1,
            internal_format,
            format,
            texture_type,
            data,
        )?;
        Ok(Self { data })
    }

    pub fn load_data(
        &self,
        data: &[u8],
        x_offset: usize,
        y_offset: usize,
        width: usize,
        height: usize,
    ) {
        self.data
            .load_data(data, x_offset, y_offset, 0, width, height, 1);
    }
}

impl TextureTrait for Texture2D {
    fn get_texture_data(&self) -> &TextureData {
        &self.data
    }
}
pub struct Texture3D {
    data: TextureData,
}

#[allow(clippy::too_many_arguments)]
impl Texture3D {
    pub fn new(
        gl: Rc<glow::Context>,
        width: usize,
        height: usize,
        depth: usize,
        internal_format: u32,
        format: Option<u32>,
        texture_type: Option<u32>,
        data: Option<&[u8]>,
    ) -> Result<Self, String> {
        let data = TextureData::new(
            gl.clone(),
            glow::TEXTURE_3D,
            width,
            height,
            depth,
            internal_format,
            format,
            texture_type,
            data,
        )?;
        Ok(Self { data })
    }

    pub fn load_data(
        &self,
        data: &[u8],
        x_offset: usize,
        y_offset: usize,
        z_offset: usize,
        width: usize,
        height: usize,
        depth: usize,
    ) {
        self.data
            .load_data(data, x_offset, y_offset, z_offset, width, height, depth);
    }
}

impl TextureTrait for Texture3D {
    fn get_texture_data(&self) -> &TextureData {
        &self.data
    }
}

// use super::utils::*;
// use glow::HasContext;
// use std::rc::Rc;
// trait Texture {
//     fn get_context(&self) -> Rc<glow::Context>;
//     fn get_id(&self) -> glow::Texture;
//     fn get_target(&self) -> u32;
//     fn get_width(&self) -> usize;
//     fn get_height(&self) -> usize;
//     fn get_depth(&self) -> usize;
//     fn get_internal_format(&self) -> u32;
//     fn get_format(&self) -> u32;
//     fn get_texture_type(&self) -> u32;

//     fn bind(&self) {
//         unsafe {
//             self.get_context()
//                 .bind_texture(self.get_target(), Some(self.get_id()));
//         }
//     }
//     fn unbind(&self) {
//         unsafe {
//             self.get_context().bind_texture(self.get_target(), None);
//         }
//     }
//     fn set_min_filter(&self, filter: u32) {
//         self.bind();
//         unsafe {
//             self.get_context().tex_parameter_i32(
//                 self.get_target(),
//                 glow::TEXTURE_MIN_FILTER,
//                 filter as i32,
//             );
//         }
//         self.unbind();
//     }
//     fn set_mag_filter(&self, filter: u32) {
//         self.bind();
//         unsafe {
//             self.get_context().tex_parameter_i32(
//                 self.get_target(),
//                 glow::TEXTURE_MAG_FILTER,
//                 filter as i32,
//             );
//         }
//         self.unbind();
//     }
//     fn set_wrap_s(&self, wrap: u32) {
//         self.bind();
//         unsafe {
//             self.get_context().tex_parameter_i32(
//                 self.get_target(),
//                 glow::TEXTURE_WRAP_S,
//                 wrap as i32,
//             );
//         }
//         self.unbind();
//     }
//     fn set_wrap_t(&self, wrap: u32) {
//         self.bind();
//         unsafe {
//             self.get_context().tex_parameter_i32(
//                 self.get_target(),
//                 glow::TEXTURE_WRAP_T,
//                 wrap as i32,
//             );
//         }
//         self.unbind();
//     }
//     fn set_wrap_r(&self, wrap: u32) {
//         self.bind();
//         unsafe {
//             self.get_context().tex_parameter_i32(
//                 self.get_target(),
//                 glow::TEXTURE_WRAP_R,
//                 wrap as i32,
//             );
//         }
//         self.unbind();
//     }
// }

// struct TextureData {
//     gl: Rc<glow::Context>,
//     id: glow::Texture,
//     target: u32,
//     width: usize,
//     height: usize,
//     depth: usize,
//     internal_format: u32,
//     format: u32,
//     texture_type: u32,
// }

// #[allow(unused, clippy::too_many_arguments)]
// impl TextureData {
//     pub fn new(
//         gl: Rc<glow::Context>,
//         target: u32,
//         width: usize,
//         height: usize,
//         depth: usize,
//         internal_format: u32,
//         format: Option<u32>,
//         texture_type: Option<u32>,
//         data: Option<&[u8]>,
//     ) -> Result<Self, String> {
//         unsafe {
//             let id = gl.create_texture().map_err(|e| e.to_string())?;
//             gl.bind_texture(target, Some(id));

//             let format = format.unwrap_or_else(|| get_gl_format_from_internal(internal_format));
//             let texture_type =
//                 texture_type.unwrap_or_else(|| get_gl_type_from_internal(internal_format));

//             gl.tex_parameter_i32(target, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
//             gl.tex_parameter_i32(target, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
//             gl.tex_parameter_i32(target, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
//             gl.tex_parameter_i32(target, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
//             gl.tex_parameter_i32(target, glow::TEXTURE_WRAP_R, glow::CLAMP_TO_EDGE as i32);

//             if target == glow::TEXTURE_1D {
//                 gl.tex_image_1d(
//                     target,
//                     0,
//                     internal_format as i32,
//                     width as i32,
//                     0,
//                     format,
//                     texture_type,
//                     data,
//                 );
//             } else if target == glow::TEXTURE_2D {
//                 gl.tex_image_2d(
//                     target,
//                     0,
//                     internal_format as i32,
//                     width as i32,
//                     height as i32,
//                     0,
//                     format,
//                     texture_type,
//                     data,
//                 );
//             } else if target == glow::TEXTURE_3D {
//                 gl.tex_image_3d(
//                     target,
//                     0,
//                     internal_format as i32,
//                     width as i32,
//                     height as i32,
//                     depth as i32,
//                     0,
//                     format,
//                     texture_type,
//                     data,
//                 );
//             } else {
//                 gl.bind_texture(target, None);
//                 return Err("Invalid texture target".to_string());
//             }

//             gl.bind_texture(target, None);

//             Ok(Self {
//                 gl,
//                 id,
//                 target,
//                 width,
//                 height,
//                 depth,
//                 internal_format,
//                 format,
//                 texture_type,
//             })
//         }
//     }

//     // pub fn bind(&self) {
//     //     unsafe {
//     //         self.gl.bind_texture(self.target, Some(self.id));
//     //     }
//     // }

//     // pub fn unbind(&self) {
//     //     unsafe {
//     //         self.gl.bind_texture(self.target, None);
//     //     }
//     // }

//     // pub fn set_min_filter(&self, filter: u32) {
//     //     self.bind();
//     //     unsafe {
//     //         self.gl
//     //             .tex_parameter_i32(self.target, glow::TEXTURE_MIN_FILTER, filter as i32);
//     //     }
//     //     self.unbind();
//     // }

//     // pub fn set_mag_filter(&self, filter: u32) {
//     //     self.bind();
//     //     unsafe {
//     //         self.gl
//     //             .tex_parameter_i32(self.target, glow::TEXTURE_MAG_FILTER, filter as i32);
//     //     }
//     //     self.unbind();
//     // }

//     // pub fn set_wrap_s(&self, wrap: u32) {
//     //     self.bind();
//     //     unsafe {
//     //         self.gl
//     //             .tex_parameter_i32(self.target, glow::TEXTURE_WRAP_S, wrap as i32);
//     //     }
//     //     self.unbind();
//     // }

//     // pub fn set_wrap_t(&self, wrap: u32) {
//     //     self.bind();
//     //     unsafe {
//     //         self.gl
//     //             .tex_parameter_i32(self.target, glow::TEXTURE_WRAP_T, wrap as i32);
//     //     }
//     //     self.unbind();
//     // }

//     // pub fn set_wrap_r(&self, wrap: u32) {
//     //     self.bind();
//     //     unsafe {
//     //         self.gl
//     //             .tex_parameter_i32(self.target, glow::TEXTURE_WRAP_R, wrap as i32);
//     //     }
//     //     self.unbind();
//     // }

//     pub fn load_data(
//         &self,
//         data: &[u8],
//         x_offset: usize,
//         y_offset: usize,
//         z_offset: usize,
//         width: usize,
//         height: usize,
//         depth: usize,
//     ) {
//         self.bind();
//         unsafe {
//             if (self.target == glow::TEXTURE_1D) {
//                 log::error!("Texture 1D not supported");
//             } else if self.target == glow::TEXTURE_2D {
//                 self.gl.tex_sub_image_2d(
//                     self.target,
//                     0,
//                     x_offset as i32,
//                     y_offset as i32,
//                     width as i32,
//                     height as i32,
//                     self.format,
//                     self.texture_type,
//                     glow::PixelUnpackData::Slice(data),
//                 );
//             } else if self.target == glow::TEXTURE_3D {
//                 self.gl.tex_sub_image_3d(
//                     self.target,
//                     0,
//                     x_offset as i32,
//                     y_offset as i32,
//                     z_offset as i32,
//                     width as i32,
//                     height as i32,
//                     depth as i32,
//                     self.format,
//                     self.texture_type,
//                     glow::PixelUnpackData::Slice(data),
//                 );
//             }
//         }
//         self.unbind();
//     }

//     pub fn get_id(&self) -> glow::Texture {
//         self.id
//     }

//     pub fn get_target(&self) -> u32 {
//         self.target
//     }

//     pub fn get_width(&self) -> usize {
//         self.width
//     }

//     pub fn get_height(&self) -> usize {
//         self.height
//     }

//     pub fn get_depth(&self) -> usize {
//         self.depth
//     }

//     pub fn get_internal_format(&self) -> u32 {
//         self.internal_format
//     }

//     pub fn get_format(&self) -> u32 {
//         self.format
//     }

//     pub fn get_texture_type(&self) -> u32 {
//         self.texture_type
//     }
// }

// impl Drop for TextureData {
//     fn drop(&mut self) {
//         unsafe {
//             self.gl.delete_texture(self.id);
//         }
//     }
// }

// pub struct Texture1D {
//     data: TextureData,
// }

// impl Texture1D {
//     pub fn new(
//         gl: Rc<glow::Context>,
//         width: usize,
//         internal_format: u32,
//         format: Option<u32>,
//         texture_type: Option<u32>,
//         data: Option<&[u8]>,
//     ) -> Result<Self, String> {
//         let data = TextureData::new(
//             gl.clone(),
//             glow::TEXTURE_1D,
//             width,
//             1,
//             1,
//             internal_format,
//             format,
//             texture_type,
//             data,
//         )?;
//         Ok(Self { data })
//     }

//     pub fn load_data(&self, data: &[u8], width: usize, x_offset: usize) {
//         self.data.load_data(data, x_offset, 0, 0, width, 1, 1);
//     }
// }
// pub struct Texture2D {
//     data: TextureData,
// }

// impl Texture2D {
//     pub fn new(
//         gl: Rc<glow::Context>,
//         width: usize,
//         height: usize,
//         internal_format: u32,
//         format: Option<u32>,
//         texture_type: Option<u32>,
//         data: Option<&[u8]>,
//     ) -> Result<Self, String> {
//         let data = TextureData::new(
//             gl.clone(),
//             glow::TEXTURE_2D,
//             width,
//             height,
//             1,
//             internal_format,
//             format,
//             texture_type,
//             data,
//         )?;
//         Ok(Self { data })
//     }

//     pub fn load_data(
//         &self,
//         data: &[u8],
//         x_offset: usize,
//         y_offset: usize,
//         width: usize,
//         height: usize,
//     ) {
//         self.data
//             .load_data(data, x_offset, y_offset, 0, width, height, 1);
//     }
// }

// pub struct Texture3D {
//     data: TextureData,
// }

// #[allow(clippy::too_many_arguments)]
// impl Texture3D {
//     pub fn new(
//         gl: Rc<glow::Context>,
//         width: usize,
//         height: usize,
//         depth: usize,
//         internal_format: u32,
//         format: Option<u32>,
//         texture_type: Option<u32>,
//         data: Option<&[u8]>,
//     ) -> Result<Self, String> {
//         let data = TextureData::new(
//             gl.clone(),
//             glow::TEXTURE_3D,
//             width,
//             height,
//             depth,
//             internal_format,
//             format,
//             texture_type,
//             data,
//         )?;
//         Ok(Self { data })
//     }

//     pub fn load_data(
//         &self,
//         data: &[u8],
//         x_offset: usize,
//         y_offset: usize,
//         z_offset: usize,
//         width: usize,
//         height: usize,
//         depth: usize,
//     ) {
//         self.data
//             .load_data(data, x_offset, y_offset, z_offset, width, height, depth);
//     }
// }
