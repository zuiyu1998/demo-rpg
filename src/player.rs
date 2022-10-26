use bevy::log;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;
use sprite_animate_player::{FrameAnimate, SpriteAnimatePlayer};

use crate::state::AppState;

pub struct PlayerPlugin;

#[derive(AssetCollection)]
pub struct PlayerAsset {
    #[asset(texture_atlas(
        tile_size_x = 64.,
        tile_size_y = 64.,
        columns = 60,
        rows = 1,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "player/Player.png")]
    pub sprite_handle: Handle<TextureAtlas>,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Action>::default())
            .add_loading_state(
                LoadingState::new(AppState::AssetLoading)
                    .continue_to_state(AppState::InGame)
                    .with_collection::<PlayerAsset>(),
            )
            .add_enter_system(AppState::InGame, spawn_main)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(player_control)
                    .into(),
            );
    }
}

fn player_control(
    mut query: Query<(&ActionState<Action>, &mut SpriteAnimatePlayer), With<Player>>,
) {
    let (action_state, mut player) = query.single_mut();

    if action_state.just_pressed(Action::Left) {
        player.play("RunLeft");
    }

    if action_state.just_pressed(Action::Down) {
        player.play("RunDown");
    }
    if action_state.just_pressed(Action::Right) {
        player.play("RunRight");
    }
    if action_state.just_pressed(Action::Up) {
        player.play("RunUp");
    }
}

fn spawn_main(mut commands: Commands, player_asset: Res<PlayerAsset>) {
    PlayerPlugin::spawn_player(&mut commands, &player_asset);
}

impl PlayerPlugin {
    pub fn spawn_player(commands: &mut Commands, player_asset: &PlayerAsset) -> Entity {
        let animate_player = Player::animate_player();

        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: player_asset.sprite_handle.clone(),
                ..Default::default()
            })
            .insert(animate_player)
            .insert(Player)
            .insert_bundle(InputManagerBundle::<Action> {
                action_state: ActionState::default(),
                input_map: InputMap::new([
                    (KeyCode::A, Action::Left),
                    (KeyCode::D, Action::Right),
                    (KeyCode::W, Action::Up),
                    (KeyCode::S, Action::Down),
                ]),
            })
            .id()
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Component)]
pub struct Player;

impl Player {
    pub fn animate_player() -> SpriteAnimatePlayer {
        let mut player = SpriteAnimatePlayer::default();

        player.cul_frame = "RunDown".to_owned();

        player.add(
            "RunRight",
            FrameAnimate::new(vec![0, 1, 2, 3, 4, 5], 0, true, 0.1),
        );

        player.add(
            "RunUp",
            FrameAnimate::new(vec![6, 7, 8, 9, 10, 11], 6, true, 0.1),
        );

        player.add(
            "RunLeft",
            FrameAnimate::new(vec![12, 13, 14, 15, 16, 17], 12, true, 0.1),
        );

        player.add(
            "RunDown",
            FrameAnimate::new(vec![18, 19, 20, 21, 22, 23], 18, true, 0.1),
        );

        player
    }
}
