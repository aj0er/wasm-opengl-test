use core::panic;
use std::convert::TryInto;

use image::ImageFormat;
use web_sys::{WebGl2RenderingContext, WebGlTexture};

pub fn load_texture(context: &WebGl2RenderingContext, bytes: &[u8], format: ImageFormat) -> WebGlTexture {
    let texture = context.create_texture();
    context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.as_ref());
    context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::REPEAT.try_into().unwrap());
    context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::REPEAT.try_into().unwrap());

    context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR.try_into().unwrap());
    context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR.try_into().unwrap());

    match image::load_from_memory_with_format(bytes, format) {
        Ok(img) => {
            context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                WebGl2RenderingContext::TEXTURE_2D as u32, 0, WebGl2RenderingContext::RGBA as i32, img.width() as i32, 
                img.height() as i32,0,WebGl2RenderingContext::RGBA,WebGl2RenderingContext::UNSIGNED_BYTE,
                Some(&img.to_rgba8().into_vec())).unwrap();

            context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D as u32);
        }
        Err(_) => {
            panic!("ok");
        }
    }

    return texture.unwrap();
}