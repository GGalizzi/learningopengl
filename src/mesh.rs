use gl;
use vek::{Mat4, Vec3};

mod constant;
mod vertex;

use vertex::VertexData;

pub struct MeshBuilder {
    vertices: Vec<f32>,
    indices: Option<Vec<u32>>,
    texture: Option<Vec<f32>>,
    normals: Option<Vec<f32>>,
    instanced: Option<Vec<Mat4<f32>>>,
}

impl MeshBuilder {
    pub fn new() -> MeshBuilder {
        MeshBuilder {
            vertices: Vec::new(),
            texture: None,
            indices: None,
            normals: None,
            instanced: None,
        }
    }

    pub fn cube(self) -> MeshBuilder {
        self.verts(&constant::CUBE_VERTICES)
    }

    pub fn verts(mut self, verts: &[f32]) -> MeshBuilder {
        self.vertices = verts.to_owned();
        self
    }

    pub fn indices(mut self, indices: &[u32]) -> MeshBuilder {
        self.indices = Some(indices.to_owned());
        self
    }

    pub fn texture_map(mut self, map: &[f32]) -> MeshBuilder {
        self.texture = Some(map.to_owned());
        self
    }
    
    pub fn normals(mut self, normals: &[f32]) -> MeshBuilder {
        self.normals = Some(normals.to_owned());
        self
    }

    pub fn instanced(mut self) -> MeshBuilder {
        let mut v = Vec::with_capacity(80);
        v.push(Mat4::identity());
        self.instanced = Some(v);
        self
    }

    pub fn finalize(self) -> Mesh {
        let verts = VertexData::new(
            self.vertices,
            self.indices,
            self.texture,
            self.normals,
            self.instanced,
        );

        Mesh::new(verts)
    }
}
pub struct Mesh {
    vertex_data: VertexData,
    vao: u32,
    instance_index: usize,
}

impl Mesh {
    pub fn new(mut vertex_data: VertexData) -> Mesh {
        let vao = vertex_data.setup_buffers();

        Mesh {
            vertex_data,
            vao,
            instance_index: 0,
        }
    }

    pub fn build() -> MeshBuilder {
        MeshBuilder::new()
    }

    pub fn bind_buffer(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn instance_reset(&mut self) {
        self.instance_index = 0;
    }

    pub fn next_instance(&mut self, offset: Mat4<f32>) {
        self.vertex_data
            .set_instance_data(self.instance_index, offset);
        self.instance_index += 1;
    }

    pub fn bind_instance_data(&mut self) {
        self.bind_buffer();
        self.vertex_data.setup_instance_buffer();
    }

    pub fn draw(&self) {
        self.bind_buffer();
        unsafe {
            if self.vertex_data.has_indices() {
                if self.vertex_data.is_instanced() {
                    unimplemented!("instanced DrawElements");
                } else {
                    gl::DrawElements(
                        gl::TRIANGLES,
                        self.vertex_data.indices_count(),
                        gl::UNSIGNED_INT,
                        0 as *const _,
                    )
                }
            } else {
                if self.vertex_data.is_instanced() {
                    // println!("should draw {:?}",
                    // self.vertex_data.instanced);
                    gl::DrawArraysInstanced(
                        gl::TRIANGLES,
                        0,
                        self.vertex_data.vertices_count(),
                        self.vertex_data.instance_len(),
                    );
                } else {
                    gl::DrawArrays(
                        gl::TRIANGLES,
                        0,
                        self.vertex_data.vertices_count(),
                    );
                }
            }
        }
    }
    
    pub fn draw_instanced(&self, count: i32) {
        self.bind_buffer();
        unsafe {
            gl::DrawArraysInstanced(
                gl::TRIANGLES,
                0,
                self.vertex_data.vertices_count(),
                count,
            );
        }
    }
}
