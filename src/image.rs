use image::GenericImageView;
use std::{path::Path, rc::Rc};

use crate::texture;

use super::texture::Texture2D;
pub struct Image {
    pub texture: texture::Texture2D,
    pub data: Vec<u8>,
}

impl Image {
    pub fn load<P: AsRef<Path>>(gl: Rc<glow::Context>, path: P) -> Result<Self, String> {
        let img = image::open(path).map_err(|e| e.to_string())?;
        let (width, height) = img.dimensions();
        let data = img.as_bytes();

        let format = match img {
            image::DynamicImage::ImageLuma8(_) => glow::R8,
            image::DynamicImage::ImageLumaA8(_) => glow::RG8,
            image::DynamicImage::ImageRgb8(_) => glow::RGB8,
            image::DynamicImage::ImageRgba8(_) => glow::RGBA8,
            image::DynamicImage::ImageLuma16(_) => glow::R16,
            image::DynamicImage::ImageLumaA16(_) => glow::R16,
            image::DynamicImage::ImageRgb16(_) => glow::RGB16,
            image::DynamicImage::ImageRgba16(_) => glow::RGBA16,
            image::DynamicImage::ImageRgb32F(_) => glow::RGB32F,
            image::DynamicImage::ImageRgba32F(_) => glow::RGBA32F,
            _ => return Err("Unsupported image format".to_string()),
        };

        let texture = Texture2D::new(
            gl,
            width as usize,
            height as usize,
            format,
            None,
            None,
            Some(data),
        )
        .map_err(|e| e.to_string())?;

        //
        texture.load_data(data, 0, 0, width as usize, height as usize);
        //

        Ok(Self {
            texture,
            data: data.to_vec(),
        })
    }
}
