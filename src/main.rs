use bevy::prelude::*;
use sprite_animate_player::SpriteAnimatePlugin;

mod player;

fn main() {
    let mut app = App::new();

    app.insert_resource(bevy::render::texture::ImageSettings::default_nearest());
    app.add_plugins(DefaultPlugins);

    app.add_plugin(SpriteAnimatePlugin);
    app.add_startup_system(setup);

    app.run()
}

fn setup(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    mut atlas: ResMut<Assets<TextureAtlas>>,
) {
    let handle = asset_server.load("player/Player.png");

    let texture_atla = TextureAtlas::from_grid(handle, Vec2::new(64.0, 64.0), 60, 1);
    let texture_atla_handle = atlas.add(texture_atla);
    command.spawn_bundle(Camera2dBundle::default());

    let animate_player = player::Player::animate_player();

    command
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atla_handle,
            ..Default::default()
        })
        .insert(animate_player)
        .insert(player::Player);
}
