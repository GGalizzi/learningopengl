use gl;
use image::{self, DynamicImage};

use std::path::Path;

pub struct Texture {
    width: u32,
    height: u32,
    pub id: u32,
}

impl Texture {
    pub fn new<P>(path: P) -> Texture
    where
        P: AsRef<Path>,
    {
        let texture_image = image::open(path).unwrap();
        let (width, height, pixels, gl_format) =
            match texture_image {
                DynamicImage::ImageRgb8(image) => (
                    image.width(),
                    image.height(),
                    image.into_raw(),
                    gl::RGB,
                ),
                DynamicImage::ImageRgba8(image) => {
                    let raw = image.clone().into_raw();
                    println!(
                        "{:?}",
                        (raw[0], raw[1], raw[2], raw[3])
                    );
                    (
                        image.width(),
                        image.height(),
                        image.into_raw(),
                        gl::RGBA,
                    )
                }
                _ => {
                    panic!("Not checking for that image type")
                }
            };
        let id = unsafe {
            let mut id = 0;
            gl::GenTextures(1, &mut id);
            id
        };

        unsafe {
            use std::ffi::c_void;
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl_format,
                gl::UNSIGNED_BYTE,
                pixels.as_ptr() as *const u8 as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        println!("loaded texture {:?}", id);

        Texture { width, height, id }
    }
}
