use bevy::prelude::*;
use bevy::app::DefaultTaskPoolOptions;
pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut AppBuilder) {
        DefaultTaskPoolOptions::default().create_default_pools(app.resources_mut());
        app.add_plugin(bevy::type_registry::TypeRegistryPlugin::default());
        app.add_plugin(bevy::core::CorePlugin::default());
        app.add_plugin(bevy::transform::TransformPlugin::default());
        app.add_plugin(bevy::diagnostic::DiagnosticsPlugin::default());
        app.add_plugin(bevy::asset::AssetPlugin::default());
        app.add_plugin(bevy::scene::ScenePlugin::default());
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn.system())
            .add_system(move_camera.system())
            .add_system(movement.system());
    }
}

fn spawn(mut commands: Commands) {
    println!("Bevy init");
}
fn move_camera(time: Res<Time>) {}
fn movement(time: Res<Time>) {
    println!("Bevy running");
}