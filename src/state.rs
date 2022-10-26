use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    Menu,
    InGame,
    AssetLoading,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(AppState::AssetLoading);
    }
}
