#![feature(clamp)]

use bevy::app::App;
use gl;
use sdl2::{self, event::Event};
use vek::{Mat4, Vec3};

mod component;
mod draw;
mod init;
mod input;
mod map;
mod mesh;
mod plugin;
mod shader;
mod texture;
mod util;

use draw::Draw;
use input::Input;
use mesh::Mesh;
use shader::ShaderProgram;
use texture::Texture;

use component::{Position, Rotation};
use plugin::{BasePlugin, GamePlugin};

type Result<T> = std::result::Result<T, String>;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

fn main() -> Result<()> {
    let sdl_context = sdl2::init()?;
    let video = sdl_context.video()?;

    init::gl(&video);
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
    init::load_gl(&video);

    sdl_context.mouse().set_relative_mouse_mode(true);

    let colvec: Vec<Vec3<i32>> = Vec::new();
    let mut bevy = std::mem::replace(
        &mut App::build()
            .add_plugin(BasePlugin)
            .add_plugin(GamePlugin)
            .add_resource(Input::new())
            .add_resource(colvec)
            .add_resource(map::Area::debug())
            .app,
        App::default(),
    );

    bevy.startup_schedule.initialize(&mut bevy.resources);
    bevy.startup_executor.run(
        &mut bevy.startup_schedule,
        &mut bevy.world,
        &mut bevy.resources,
    );

    let program = ShaderProgram::new(
        "shaders/base.vert",
        "shaders/basic.frag",
    )?;

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
        .texture_map(&[
            0.0, 0.0, // Bottom left
            1.0, 0.0, // Bottom right
            0.0, 1.0, // Top left
            1.0, 1.0, // Top right
        ])
        .finalize();

    let ceiling = Mesh::build()
        .verts(&[
            0.0, 1.0, 0.0, 20.0, 1.0, 0.0, 0.0, 1.0, 20.0,
            20.0, 1.0, 20.0,
        ])
        .indices(&[2, 0, 1, 2, 3, 1])
        .texture_map(&[
            0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0,
        ])
        .finalize();
    let cube = Mesh::build().cube().finalize();

    let wall_texture =
        Texture::new("assets/stone_wall_b.png");

    let floor_texture =
        Texture::new("assets/stone_floor_c.png");

    let mut event_pump = sdl_context.event_pump()?;

    let scale = Mat4::scaling_3d(Vec3::new(9.2, 1.0, 1.0));
    let model = Mat4::rotation_y(-90f32.to_radians()) * scale;

    // let view = Mat4::from_translation((0., 0.,
    // -3.).into());

    /*
    let view = glam::Mat4::look_at_rh(
        (0.0, 0.0, 3.0).into(),
        (0.0, 0.0, 0.0).into(),
        Vec3::new(0.0, 1.0, 0.0),
    );
    */

    let projection = Mat4::perspective_fov_rh_no(
        45f32.to_radians(),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        0.1,
        100.,
    );
    /*
    let projection = Mat4::perspective_rh_zo(
        45f32.to_radians(),
        WINDOW_WIDTH / WINDOW_HEIGHT,
        0.1,
        100.,
    );
    */

    let area = map::Area::debug();
    'running: loop {
        unsafe {
            gl::ClearColor(0.005, 0.0, 0.15, 1.0);
            gl::Clear(
                gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT,
            );
        }

        // Event handling
        {
            let mut input =
                bevy.resources.get_mut::<Input>().unwrap();
            input.set_mouse(0, 0);
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown {
                        keycode:
                            Some(sdl2::keyboard::Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => {
                        input.press(keycode);
                    }
                    Event::KeyUp {
                        keycode: Some(keycode),
                        ..
                    } => {
                        input.release(keycode);
                    }
                    Event::MouseMotion {
                        xrel, yrel, ..
                    } => {
                        input.set_mouse(xrel, yrel);
                    }
                    _ => {}
                }
            }
        }

        bevy.update();

        let mut view = Mat4::identity();

        for (pos, dir) in bevy
            .world
            .query::<(&Position, &Rotation)>()
            .iter()
        {
            /*
            view = Mat4::from_rotation_translation(
                dir.quat.conjugate(),
                pos.internal(),
            )
            .inverse();
            */
            view = Mat4::from(dir.quat.conjugate())
                .translated_3d(pos.internal())
                .inverted();
        }

        let mvp = projection * view * model;
        /*
        let d = Draw::with(&program)
            .with_texture_n(&wall_texture, 0)
            .with_matrix("mvp", &mvp);
        //.mesh(&plane);
        let model =
            Mat4::from_rotation_y(0f32.to_radians()) * scale;
        d.with_matrix(
            "mvp",
            &(projection * view * Mat4::identity()),
        )
        .mesh(&cube);
        */

        let colvec =
            bevy.resources.get::<Vec<Vec3<i32>>>().unwrap();
        for z in 0..4 {
            for x in 0..20 {
                for y in 0..8 {
                    let t = &area.tiles
                        [(20 * y + x) + z * 20 * 8];
                    if t.is_wall() {
                        let mvp = projection *
                            view *
                            Mat4::translation_3d(
                                Vec3::new(
                                    x as f32, z as f32,
                                    y as f32,
                                ),
                            );
                        Draw::with(&program)
                            .with_matrix("mvp", &mvp)
                            .with_texture_n(&wall_texture, 0)
                            .mesh(&cube);
                    } else {
                        /*
                        // Ceiling and floor?
                        let d = Draw::with(&program)
                            .with_texture_n(
                                &floor_texture,
                                0,
                            );

                        let mvp = projection *
                            view *
                            Mat4::from_translation(
                                Vec3::new(
                                    x as f32, 1.0, y as f32,
                                ),
                            );
                        let d = d
                            .with_matrix("mvp", &mvp)
                            .mesh(&cube);

                        let mvp = projection *
                            view *
                            Mat4::from_translation(
                                Vec3::new(
                                    x as f32, -1.0, y as f32,
                                ),
                            );
                        d.with_matrix("mvp", &mvp)
                            .mesh(&cube);
                            */
                    }
                }
            }
        }

        window.gl_swap_window();
    }

    Ok(())
}
