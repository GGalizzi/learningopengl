use std::ffi;

use gl::types::*;
use vek::{Mat4, Vec4};

#[derive(Debug)]
pub struct VertexData {
    data: Vec<f32>,
    indices: Option<Vec<u32>>,
    texture: Option<Vec<f32>>,
    normals: Option<Vec<f32>>,
    pub instanced: Option<Vec<Mat4<f32>>>,
    instance_vbo: u32,
}

impl VertexData {
    pub fn new(
        vertices: Vec<f32>,
        indices: Option<Vec<u32>>,
        texture: Option<Vec<f32>>,
        normals: Option<Vec<f32>>,
        instanced: Option<Vec<Mat4<f32>>>,
    ) -> VertexData {
        let data = vertices.chunks_exact(3).enumerate().fold(
            Vec::new(),
            |mut acc, (i, vert)| {
                acc.push(vert[0]);
                acc.push(vert[1]);
                acc.push(vert[2]);
                
                if let Some(texture) = texture.as_ref() {
                    acc.push(texture[i * 2]);
                    acc.push(texture[i * 2 + 1]);
                }
                if let Some(normal) = normals.as_ref() {
                    acc.push(normal[i * 3]);
                    acc.push(normal[i * 3 + 1]);
                    acc.push(normal[i * 3 + 2]);
                }
                acc
            },
        );
        VertexData {
            data,
            indices,
            texture,
            normals,
            instanced,
            instance_vbo: 0,
        }
    }

    pub fn has_indices(&self) -> bool {
        self.indices.is_some()
    }

    pub fn is_instanced(&self) -> bool {
        self.instanced.is_some()
    }

    pub fn indices_count(&self) -> i32 {
        self.indices.as_ref().unwrap().len() as i32
    }

    pub fn size(&self) -> GLsizeiptr {
        (self.data.len() * std::mem::size_of::<GLfloat>())
            as GLsizeiptr
    }

    pub fn indices_size(&self) -> GLsizeiptr {
        (self.indices.as_ref().unwrap().len() *
            std::mem::size_of::<GLfloat>())
            as GLsizeiptr
    }

    pub fn instance_size(&self) -> GLsizeiptr {
        (self.instance_len() as usize *
            std::mem::size_of::<Mat4<f32>>())
            as GLsizeiptr
    }

    pub fn instance_len(&self) -> GLsizei {
        self.instanced.as_ref().unwrap().len() as i32
    }

    pub fn as_ptr(&self) -> *const ffi::c_void {
        self.data.as_ptr() as *const _
    }

    pub fn indices_ptr(&self) -> *const ffi::c_void {
        self.indices.as_ref().unwrap().as_ptr() as *const _
    }

    pub fn instance_ptr(&self) -> *const ffi::c_void {
        self.instanced.as_ref().unwrap()[0].as_col_ptr()
            as *const _
        // self.instanced.as_ref().unwrap()[0] as *const
        // Mat4<f32> as *const _
    }

    pub fn vertices_count(&self) -> i32 {
        self.data
            .chunks_exact(self.elements_per_vertex() as usize)
            .len() as i32
    }

    fn elements_per_vertex(&self) -> i32 {
        let mut elements_per_vertex = 3;
        /*
        if self.has_color_data() {
            elements_per_vertex += 3;
        }

        */
        if self.texture.is_some() {
            elements_per_vertex += 2;
        }

        if self.normals.is_some() {
            elements_per_vertex += 3;
        }

        elements_per_vertex
    }

    pub fn stride(&self) -> GLsizei {
        let elements_per_vertex = self.elements_per_vertex();
        elements_per_vertex *
            std::mem::size_of::<GLfloat>() as GLsizei
    }

    pub fn texture_offset(&self) -> *const ffi::c_void {
        (3 * std::mem::size_of::<GLfloat>()) as *const _
    }

    pub fn normals_offset(&self) -> *const ffi::c_void {
        let offset = if self.texture.is_some() {
            5
        } else {
            3
        };

        (offset * std::mem::size_of::<GLfloat>()) as *const _
    }

    pub fn set_instance_data(
        &mut self,
        index: usize,
        offset: Mat4<f32>,
    ) {
        let vec = self.instanced.as_mut().unwrap();
        if index >= vec.len() {
            vec.resize(index + 50, offset);
        }
        vec[index] = offset;
    }

    pub fn setup_instance_buffer(&mut self) {
        unsafe {
            gl::BindBuffer(
                gl::ARRAY_BUFFER,
                self.instance_vbo,
            );
            gl::BufferData(
                gl::ARRAY_BUFFER,
                self.instance_size(),
                self.instance_ptr(),
                gl::STATIC_DRAW,
            );
        }
        self.setup_instance_attribute();
    }
    pub fn setup_buffers(&mut self) -> u32 {
        unsafe {
            let mut vao = 0;

            gl::GenVertexArrays(1, &mut vao);
            let mut vbo = 0;

            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                self.size(),
                self.as_ptr(),
                gl::STATIC_DRAW,
            );

            if let Some(ref indices) = self.indices {
                let mut ebo = 0;
                gl::GenBuffers(1, &mut ebo);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    self.indices_size(),
                    self.indices_ptr(),
                    gl::STATIC_DRAW,
                );
            }

            if let Some(_) = self.instanced {
                gl::GenBuffers(1, &mut self.instance_vbo);
                self.setup_instance_buffer();
                gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            }

            self.setup_position_attribute();
            
            if let Some(ref _textures) = self.texture {
                self.setup_texture_attribute();
            }

            if let Some(ref _normals) = self.normals {
                self.setup_normal_attribute();
            }

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            vao
        }
    }

    pub fn setup_position_attribute(&self) {
        unsafe {
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                self.stride(),
                std::ptr::null(),
            );

            gl::EnableVertexAttribArray(0);
        }
    }
    
    pub fn setup_normal_attribute(&self) {
        unsafe {
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                self.stride(),
                self.normals_offset(),
            );

            gl::EnableVertexAttribArray(1);
        }
    }

    pub fn setup_texture_attribute(&self) {
        unsafe {
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                self.stride(),
                self.texture_offset(),
            );

            gl::EnableVertexAttribArray(1);
        }
    }

    pub fn setup_instance_attribute(&self) {
        unsafe {
            let size_mat =
                std::mem::size_of::<Mat4<f32>>() as i32;
            let size_vec =
                std::mem::size_of::<Vec4<f32>>() as i32;
            gl::VertexAttribPointer(
                3,
                4,
                gl::FLOAT,
                gl::FALSE,
                size_mat,
                0 as *const _,
            );

            gl::VertexAttribPointer(
                4,
                4,
                gl::FLOAT,
                gl::FALSE,
                size_mat,
                size_vec as *const _,
            );

            gl::VertexAttribPointer(
                5,
                4,
                gl::FLOAT,
                gl::FALSE,
                size_mat,
                (2 * size_vec) as *const _,
            );

            gl::VertexAttribPointer(
                6,
                4,
                gl::FLOAT,
                gl::FALSE,
                size_mat,
                (3 * size_vec) as *const _,
            );

            gl::EnableVertexAttribArray(3);
            gl::EnableVertexAttribArray(4);
            gl::EnableVertexAttribArray(5);
            gl::EnableVertexAttribArray(6);
            gl::VertexAttribDivisor(3, 1);
            gl::VertexAttribDivisor(4, 1);
            gl::VertexAttribDivisor(5, 1);
            gl::VertexAttribDivisor(6, 1);
        }
    }
}
