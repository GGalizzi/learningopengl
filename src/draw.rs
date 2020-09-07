use std::ffi;

use glam::Mat4;

use crate::mesh::Mesh;
use crate::shader::ShaderProgram;

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
        matrix: &Mat4,
    ) -> Draw<'a> {
        let loc = self.get_uniform_location(uniform);

        unsafe {
            gl::UniformMatrix4fv(
                loc,
                1,
                gl::FALSE,
                matrix.as_ref().as_ptr(),
            );
        }

        self
    }

    pub fn mesh(self, mesh: &Mesh) -> Draw<'a> {
        mesh.draw();
        self
    }
}
