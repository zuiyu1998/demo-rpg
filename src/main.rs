use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_startup_system(setup);

    app.run()
}

fn setup(mut command: Commands) {
    command.spawn_bundle(Camera2dBundle::default());
}
