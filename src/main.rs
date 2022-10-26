use bevy::{log::LogSettings, prelude::*, utils::tracing};
use player::PlayerPlugin;
use sprite_animate_player::SpriteAnimatePlugin;
use state::StatePlugin;

mod player;
mod state;

fn main() {
    let mut app = App::new();

    #[cfg(feature = "debug")]
    app.insert_resource(LogSettings {
        level: tracing::Level::DEBUG,
        ..Default::default()
    });

    app.insert_resource(bevy::render::texture::ImageSettings::default_nearest());
    app.add_plugins(DefaultPlugins);

    app.add_plugin(SpriteAnimatePlugin);

    app.add_plugin(StatePlugin);
    app.add_plugin(PlayerPlugin);
    app.add_startup_system(setup);

    app.run()
}

fn setup(mut command: Commands) {
    command.spawn_bundle(Camera2dBundle::default());
}
