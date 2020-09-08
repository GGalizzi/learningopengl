use bevy::{app::DefaultTaskPoolOptions, prelude::*};
use glam::{Mat4, Vec4};
use sdl2::keyboard::Keycode;

use crate::{
    component::{Position, Rotation},
    input::Input,
};
pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut AppBuilder) {
        DefaultTaskPoolOptions::default()
            .create_default_pools(app.resources_mut());
        app.add_plugin(
            bevy::type_registry::TypeRegistryPlugin::default(
            ),
        );
        app.add_plugin(bevy::core::CorePlugin::default());
        app.add_plugin(
            bevy::transform::TransformPlugin::default(),
        );
        app.add_plugin(
            bevy::diagnostic::DiagnosticsPlugin::default(),
        );
        app.add_plugin(bevy::asset::AssetPlugin::default());
        app.add_plugin(bevy::scene::ScenePlugin::default());
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn.system())
            .add_system(move_camera.system())
            .add_system(movement.system())
            .add_system(rotation.system());
    }
}

fn spawn(mut commands: Commands) {
    commands.spawn((
        Position::new(1.0, 0.0, 1.0),
        Rotation::new(),
    ));
}
fn move_camera(time: Res<Time>) {}
fn movement(
    time: Res<Time>,
    input: Res<Input>,
    direction: &Rotation,
    mut position: Mut<Position>,
) {
    let speed = 12.0;
    let dt = time.delta_seconds;
    let speed = dt * speed;

    if input.is_pressed(Keycode::Space) {
        *position = position.move_towards(
            direction.quat.conjugate() *
                Vec3::new(0.0, 1.0 * speed, 0.0),
        );
    }

    if input.is_pressed(Keycode::W) {
        *position = position.move_towards(
            direction.quat.conjugate() *
                Vec3::new(0.0, 0.0, -1.0 * speed),
        );
    }

    if input.is_pressed(Keycode::S) {
        *position = position.move_towards(
            direction.quat.conjugate() *
                Vec3::new(0.0, 0.0, 1.0 * speed),
        );
    }

    if input.is_pressed(Keycode::A) {
        *position = position.move_towards(
            direction.quat.conjugate() *
                Vec3::new(-1.0 * speed, 0.0, 0.0),
        );
    }

    if input.is_pressed(Keycode::D) {
        *position = position.move_towards(
            direction.quat.conjugate() *
                Vec3::new(1.0 * speed, 0.0, 0.0),
        );
    }
}

fn rotation(
    time: Res<Time>,
    input: Res<Input>,
    mut direction: Mut<Rotation>,
) {
    let dt = time.delta_seconds;
    let speed = 120.0;
    let speed = dt * speed;

    direction.rotate_on_x(input.mouse_y() * speed, dt);
    direction.rotate_on_y(input.mouse_x() * speed, dt);
}
