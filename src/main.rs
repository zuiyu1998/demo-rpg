use bevy::prelude::*;
use heron::PhysicsPlugin;
use player::PlayerPlugin;
use sprite_animate_player::SpriteAnimatePlugin;
use state::StatePlugin;

mod player;
mod state;

#[cfg(feature = "debug")]
mod debug;

fn main() {
    let mut app = App::new();

    app.insert_resource(bevy::render::texture::ImageSettings::default_nearest());

    app.add_plugins(DefaultPlugins);
    app.add_plugin(PhysicsPlugin::default());

    app.add_plugin(SpriteAnimatePlugin);

    #[cfg(feature = "debug")]
    app.add_plugin(debug::DebugPlugin);

    app.add_plugin(StatePlugin);
    app.add_plugin(PlayerPlugin);
    app.add_startup_system(setup);

    app.run()
}

fn setup(mut command: Commands) {
    command.spawn_bundle(Camera2dBundle::default());
}
