use bevy::{log::LogSettings, prelude::*, utils::tracing};
use bevy_inspector_egui::WorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug")]
        app.insert_resource(LogSettings {
            level: tracing::Level::INFO,
            ..Default::default()
        });

        app.add_plugin(WorldInspectorPlugin::default());
    }
}
