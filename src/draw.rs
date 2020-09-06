use crate::shader::ShaderProgram;
use crate::mesh::Mesh;

pub struct Draw<'a> {
    program: &'a ShaderProgram
}

impl<'a> Draw<'a> {
    pub fn with(program: &'a ShaderProgram) -> Draw<'a> {
        program.enable();
        Draw {
            program,
        }
    }
    
    pub fn mesh(self, mesh: &Mesh) -> Draw<'a> {
        mesh.draw();
        self
    }
}