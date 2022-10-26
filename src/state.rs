use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    Menu,
    Game,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Null,
    AssetLoading,
    InGame,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::AssetLoading);
    }
}
