use gl;
use glam::{Mat4, Vec3};
use sdl2::{self, event::Event};

mod draw;
mod init;
mod mesh;
mod shader;

use draw::Draw;
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

    let plane = Mesh::build()
        .verts(&[
            -0.5, -0.5, 0.0, // Bottom left
            0.5, -0.5, 0.0, // Bottom right
            -0.5, 0.5, 0.0, // Top left
            0.5, 0.5, 0.0, // Top right
        ])
        .indices(&[2, 0, 1, 2, 3, 1])
        .finalize();
    let program = ShaderProgram::new(
        "shaders/base.vert",
        "shaders/basic.frag",
    )?;

    let mut event_pump = sdl_context.event_pump()?;

    let model = 
        Mat4::from_rotation_y(-25f32.to_radians()) *
Mat4::from_scale(Vec3::new(9.2, 1.0, 1.0));

    let view = Mat4::from_translation((0., 0., -3.).into());

    /*
    let view = glam::Mat4::look_at_rh(
        (0.0, 0.0, 3.0).into(),
        (0.0, 0.0, 0.0).into(),
        Vec3::new(0.0, 1.0, 0.0),
    );
    */

    let projection = Mat4::perspective_rh(
        45f32.to_radians(),
        WINDOW_WIDTH / WINDOW_HEIGHT,
        0.1,
        100.,
    );

    let mvp = projection * view * model;

    'running: loop {
        unsafe {
            gl::ClearColor(0.005, 0.0, 0.15, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode:
                        Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        Draw::with(&program)
            .with_matrix("mvp", &mvp)
            .mesh(&plane);

        window.gl_swap_window();
        ::std::thread::sleep(::std::time::Duration::new(
            0,
            1_000_000_000u32 / 60,
        ));
    }

    Ok(())
}
