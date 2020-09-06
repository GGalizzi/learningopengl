use std::path::Path;

use gl::types::*;

use crate::Result;

pub struct ShaderProgram {
    program_id: u32,
}

impl ShaderProgram {
    pub fn new<P>(
        vertex: P,
        fragment: P,
    ) -> Result<ShaderProgram>
    where
        P: AsRef<Path>,
    {
        let read_vertex = std::fs::read_to_string(vertex)
            .map_err(|e| e.to_string())?;

        let read_fragment = std::fs::read_to_string(fragment)
            .map_err(|e| e.to_string())?;

        let vertex_shader =
            load_shader(ShaderType::Vert, &read_vertex)?;
        let fragment_shader =
            load_shader(ShaderType::Frag, &read_fragment)?;

        let program_id = unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            let mut success = 1;
            gl::GetProgramiv(
                program,
                gl::LINK_STATUS,
                &mut success,
            );

            if success != 1 {
                return Err(
                    "Failed to link program.".to_string()
                );
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            program
        };

        Ok(ShaderProgram { program_id })
    }

    pub fn enable(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }
}

#[derive(Debug)]
pub enum ShaderType {
    Frag,
    Vert,
}

impl ShaderType {
    pub fn to_gl(&self) -> GLenum {
        match self {
            ShaderType::Frag => gl::FRAGMENT_SHADER,
            ShaderType::Vert => gl::VERTEX_SHADER,
        }
    }
}

pub fn load_shader(
    shader_type: ShaderType,
    src: &str,
) -> Result<u32> {
    let shader =
        unsafe { gl::CreateShader(shader_type.to_gl()) };
    unsafe {
        gl::ShaderSource(
            shader,
            1,
            &std::ffi::CString::new(src.as_bytes())
                .unwrap()
                .as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(shader);

        let mut success = 1;
        gl::GetShaderiv(
            shader,
            gl::COMPILE_STATUS,
            &mut success,
        );

        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1);

        if success != 1 {
            gl::GetShaderInfoLog(
                shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            return Err(format!(
                "Failed compiling shader {}",
                std::str::from_utf8(&info_log).unwrap()
            ));
        }
        println!(
            "{:?} shader compilation status: {:?}",
            shader_type, success
        );
    }

    Ok(shader)
}
