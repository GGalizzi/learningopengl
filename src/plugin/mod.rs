use bevy::{
    app::DefaultTaskPoolOptions,
    prelude::{
        AppBuilder, Commands, IntoForEachSystem,
        IntoQuerySystem, Mut, Plugin, Query, Res, ResMut,
        Time,
    },
};
use sdl2::keyboard::Keycode;
use vek::*;

use crate::map;

use crate::{
    component::{BoundingBox, Position, Rotation, Velocity},
    input::Input,
    util,
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
            bevy::diagnostic::DiagnosticsPlugin::default(),
        );
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn.system())
            //.add_system(gravity.system())
            .add_system(momentum.system())
            .add_system(movement.system())
            .add_system(rotation.system());
    }
}

fn spawn(mut commands: Commands) {
    commands.spawn((
        Position::new(0.0, -1.0, 0.0),
        Velocity::new(),
        Rotation::new(),
        BoundingBox::new(0.15, 0.1),
    ));
}

fn gravity(time: Res<Time>, mut velocity: Mut<Velocity>) {
    velocity.apply_force(Vec3::down() * time.delta_seconds);
}

fn momentum(
    time: Res<Time>,
    input: Res<Input>,
    rotation: &Rotation,
    mut velocity: Mut<Velocity>,
) {
    let dt = time.delta_seconds;

    let mut applied_vel: Vec3<f32> = Vec3::zero();
    let conj = rotation.quat.conjugate();

    if input.is_pressed(Keycode::Space) {
        applied_vel += conj * Vec3::new(0.0, 1.0, 0.0);
    }

    if input.is_pressed(Keycode::W) {
        applied_vel += conj * Vec3::forward_rh();
    }

    if input.is_pressed(Keycode::S) {
        applied_vel += conj * Vec3::back_rh();
    }

    if input.is_pressed(Keycode::A) {
        applied_vel += conj * Vec3::new(-1.0, 0.0, 0.0);
    }

    if input.is_pressed(Keycode::D) {
        applied_vel += conj * Vec3::new(1.0, 0.0, 0.0);
    }

    let transition_speed = 6.0;
    let max_speed = 0.10;
    if applied_vel.magnitude().abs() >= 0.01 {
        applied_vel = applied_vel.normalized();
    }

    applied_vel *= max_speed;
    *velocity = Velocity::from(
        velocity.internal() * (1.0 - dt * transition_speed) +
            applied_vel * (dt * transition_speed),
    );
}

fn movement(
    time: Res<Time>,
    area: Res<map::Area>,
    mut query: Query<(
        &Rotation,
        Mut<Velocity>,
        Mut<Position>,
        Option<&BoundingBox>,
    )>,
) {
    let speed = 12.0;
    let dt = time.delta_seconds;
    let speed = dt * speed;

    for (direction, mut velocity, mut position, bound_box) in
        &mut query.iter()
    {
        let movement_vector = velocity.internal();

        let mut new_position =
            position.move_towards(movement_vector).internal();

        const MAX_ATTEMPTS: usize = 16;
        let mut attempts = 0;
        while let Some((collidee, collider, _)) =
            area.blocks_around(new_position)
        {
            if attempts == MAX_ATTEMPTS {
                break;
            }

            let pen =
                collider.collision_vector_with_aabb(collidee);
            let max_axis =
                pen.map(|e| e.abs()).reduce_partial_min();
            let mut dir = -pen.map(|e| {
                if e.abs().to_bits() == max_axis.to_bits() {
                    e
                } else {
                    0.0
                }
            });

            if (false && dir.x.abs() > 0.0 ||
                dir.z.abs() > 0.0) &&
                pen.y.abs() < 0.25
            {
                dir = Vec3::new(-dir.x, 0.0, -dir.z);
            } else {
                *velocity = Velocity::from(
                    velocity.internal().map2(dir, |e, d| {
                        if d * e.signum() < 0.0 {
                            0.0
                        } else {
                            e
                        }
                    }),
                );
            }
            if attempts == 0 {
                new_position = new_position + dir;
            } else {
                new_position += dir;
            }

            attempts += 1;
        }
        *position = Position::from_vector(new_position);
    }
}

fn mul_each(vec: Vec3<f32>, other: Vec3<f32>) -> Vec3<f32> {
    Vec3::new(
        vec.x * other.x,
        vec.y * other.y,
        vec.z * other.z,
    )
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
