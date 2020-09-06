use gl;

mod vertex;

use vertex::VertexData;

pub struct MeshBuilder {
    vertices: Vec<f32>,
    indices: Option<Vec<u32>>,
}

impl MeshBuilder {
    pub fn new() -> MeshBuilder {
        MeshBuilder {
            vertices: Vec::new(),
            indices: None,
        }
    }

    pub fn verts(mut self, verts: &[f32]) -> MeshBuilder {
        self.vertices = verts.to_owned();
        self
    }

    pub fn finalize(self) -> Mesh {
        let verts =
            VertexData::new(self.vertices, self.indices);
        Mesh::new(verts)
    }
}
pub struct Mesh {
    vertex_data: VertexData,
    vao: u32,
}

impl Mesh {
    pub fn new(vertex_data: VertexData) -> Mesh {
        let vao = vertex_data.setup_buffers();

        Mesh { vertex_data, vao }
    }

    pub fn build() -> MeshBuilder {
        MeshBuilder::new()
    }

    pub fn bind_buffer(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }
}
