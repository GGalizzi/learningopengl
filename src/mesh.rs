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

    pub fn cube() -> MeshBuilder {
        let xs: [f32; 2] = [-0.5, 0.5];
        let ys: [f32; 2] = [-0.5, 0.5];
        let zs: [f32; 2] = [-0.5, 0.5];

        let mut vertices: Vec<f32> = Vec::new();
        for x in &xs {
            for y in &ys {
                for z in &zs {
                    vertices.push(*x);
                    vertices.push(*y);
                    vertices.push(*z);
                }
            }
        }

        println!("{:?} verts {:?}", vertices, vertices.len());

        MeshBuilder {
            vertices,
            indices: None,
        }
    }

    pub fn verts(mut self, verts: &[f32]) -> MeshBuilder {
        self.vertices = verts.to_owned();
        self
    }

    pub fn indices(mut self, indices: &[u32]) -> MeshBuilder {
        self.indices = Some(indices.to_owned());
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

    pub fn draw(&self) {
        self.bind_buffer();
        unsafe {
            if self.vertex_data.has_indices() {
                gl::DrawElements(
                    gl::TRIANGLES,
                    self.vertex_data.indices_count(),
                    gl::UNSIGNED_INT,
                    0 as *const _,
                )
            } else {
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }
        }
    }
}
