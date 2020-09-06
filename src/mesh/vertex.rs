use gl::types::*;
use std::ffi;
pub struct VertexData {
    data: Vec<f32>,
    indices: Option<Vec<u32>>,
}

impl VertexData {
    pub fn new(
        data: Vec<f32>,
        indices: Option<Vec<u32>>,
    ) -> VertexData {
        VertexData { data, indices }
    }

    pub fn size(&self) -> GLsizeiptr {
        (self.data.len() * std::mem::size_of::<GLfloat>())
            as GLsizeiptr
    }

    pub fn as_ptr(&self) -> *const ffi::c_void {
        self.data.as_ptr() as *const _
    }

    pub fn stride(&self) -> GLsizei {
        let mut elements_per_vertex = 3;
        /*
        if self.has_color_data() {
            elements_per_vertex += 3;
        }
        if self.has_texture_data() {
            elements_per_vertex += 2;
        }
        */
        println!(
            "stride: {:?} * sizeof(float)",
            elements_per_vertex
        );
        elements_per_vertex *
            std::mem::size_of::<GLfloat>() as GLsizei
    }

    pub fn setup_buffers(&self) -> u32 {
        unsafe {
            if let Some(ref indices) = self.indices {
                // TODO: EBO
            }

            let mut vao = 0;
            let mut vbo = 0;

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                self.size(),
                self.as_ptr(),
                gl::STATIC_DRAW,
            );

            self.setup_position_attribute();

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
}
