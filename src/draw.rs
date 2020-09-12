use std::ffi;

use vek::Mat4;

use crate::{
    mesh::Mesh, shader::ShaderProgram, texture::Texture,
};

pub struct Draw<'a> {
    program: &'a ShaderProgram,
}

impl<'a> Draw<'a> {
    pub fn with(program: &'a ShaderProgram) -> Draw<'a> {
        program.enable();
        Draw { program }
    }

    fn get_uniform_location(&self, name: &str) -> i32 {
        let name = ffi::CString::new(name).unwrap();
        let location = unsafe {
            gl::GetUniformLocation(
                self.program.id(),
                name.as_ptr(),
            )
        };

        if location == -1 {
            println!(
                "Failed to load uniform location {:?} for {:?}",
                name, self.program.id()
            );
        }

        location
    }

    pub fn with_matrix(
        self,
        uniform: &str,
        matrix: &Mat4<f32>,
    ) -> Draw<'a> {
        let loc = self.get_uniform_location(uniform);

        unsafe {
            gl::UniformMatrix4fv(
                loc,
                1,
                matrix.gl_should_transpose() as _,
                matrix.as_col_ptr(),
            );
        }

        self
    }

    pub fn with_bool(
        self,
        uniform: &str,
        val: bool,
    ) -> Draw<'a> {
        let loc = self.get_uniform_location(uniform);

        unsafe {
            gl::Uniform1uiv(loc, 1, &(val as u32));
        }

        self
    }

    pub fn mesh(self, mesh: &Mesh) -> Draw<'a> {
        mesh.draw();
        self
    }

    pub fn with_texture_n(
        self,
        texture: &Texture,
        n: u32,
    ) -> Draw<'a> {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + n);
            gl::BindTexture(gl::TEXTURE_2D, texture.id)
        }
        self
    }
}
