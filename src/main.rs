use gl;
use sdl2::{self, event::Event};

mod init;
mod mesh;
mod shader;

use mesh::Mesh;
use shader::ShaderProgram;

type Result<T> = std::result::Result<T, String>;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

fn main() -> Result<()> {
    let sdl_context = sdl2::init()?;
    let video = sdl_context.video()?;

    let window = video
        .window(
            "bevy sdl opengl fun",
            WINDOW_WIDTH as u32,
            WINDOW_HEIGHT as u32,
        )
        .opengl()
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let _ctx = window.gl_create_context()?;
    init::gl(&video);

    let triangle = Mesh::build()
        .verts(&[
            -0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0,
        ])
        .finalize();

    let program = ShaderProgram::new(
        "shaders/base.vert",
        "shaders/basic.frag",
    )?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        unsafe {
            gl::ClearColor(0.005, 0.0, 0.15, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        program.enable();
        triangle.bind_buffer();

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown {
                    keycode:
                        Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        window.gl_swap_window();
        ::std::thread::sleep(::std::time::Duration::new(
            0,
            1_000_000_000u32 / 60,
        ));
    }

    Ok(())
}
