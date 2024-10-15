use glow::*;
use log::warn;

pub fn get_gl_format_from_internal(internal_format: u32) -> u32 {
    match internal_format {
        R8 | R16 | R16F | R32F => RED,

        R8I | R8UI | R16I | R16UI | R32I | R32UI => RED_INTEGER,

        RG8 | RG16 | RG16F | RG32F => RG,

        RG8I | RG8UI | RG16I | RG16UI | RG32I | RG32UI => RG_INTEGER,

        RGB8 | RGB16 | RGB16F | RGB32F => RGB,

        RGB8I | RGB8UI | RGB16I | RGB16UI | RGB32I | RGB32UI => RGB_INTEGER,

        RGBA8 | RGBA16 | RGBA16F | RGBA32F => RGBA,

        RGBA8I | RGBA8UI | RGBA16I | RGBA16UI | RGBA32I | RGBA32UI => RGBA_INTEGER,

        DEPTH_STENCIL | DEPTH24_STENCIL8 => DEPTH_STENCIL,

        DEPTH_COMPONENT | DEPTH_COMPONENT16 | DEPTH_COMPONENT24 | DEPTH_COMPONENT32 => {
            DEPTH_COMPONENT
        }

        STENCIL_INDEX | STENCIL_INDEX8 => STENCIL_INDEX,

        _ => {
            warn!(
                "Unknown internal format: {}, returning RGBA",
                internal_format
            );
            RGBA
        }
    }
}

pub fn get_gl_type_from_internal(internal_format: u32) -> u32 {
    match internal_format {
        R8 | RG8 | RGB8 | RGBA8 => UNSIGNED_BYTE,

        // normalized to 0-1
        R16 | RG16 | RGB16 | RGBA16 => UNSIGNED_SHORT,

        R16UI | RG16UI | RGB16UI | RGBA16UI => UNSIGNED_SHORT,

        R16I | RG16I | RGB16I | RGBA16I => SHORT,

        R32I | RG32I | RGB32I | RGBA32I => INT,

        R16F | RG16F | RGB16F | RGBA16F => HALF_FLOAT,

        R32F | RG32F | RGB32F | RGBA32F => FLOAT,

        DEPTH_STENCIL | DEPTH24_STENCIL8 => UNSIGNED_INT_24_8,

        DEPTH_COMPONENT16 => UNSIGNED_SHORT,

        DEPTH_COMPONENT | DEPTH_COMPONENT24 => UNSIGNED_INT,

        DEPTH_COMPONENT32 => UNSIGNED_INT,

        DEPTH_COMPONENT32F => FLOAT,

        STENCIL_INDEX | STENCIL_INDEX8 => UNSIGNED_BYTE,

        _ => {
            warn!(
                "Unknown internal format: {}, returning UNSIGNED_BYTE",
                internal_format
            );
            UNSIGNED_BYTE
        }
    }
}
