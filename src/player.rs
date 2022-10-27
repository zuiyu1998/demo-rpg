use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;
use sprite_animate_player::{FrameAnimate, SpriteAnimatePlayer};
use sprite_animate_player::{SpriteAnimateTree, SpriteAnimateVec2};

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

fn boo_as_f32(bool: bool) -> f32 {
    if bool {
        return 1.0;
    } else {
        return 0.0;
    }
}

fn player_control(mut query: Query<(&ActionState<Action>, &mut SpriteAnimateTree), With<Player>>) {
    let (action_state, mut tree) = query.single_mut();

    let mut input = Vec2::ZERO;

    input.x = boo_as_f32(action_state.pressed(Action::Right))
        - boo_as_f32(action_state.pressed(Action::Left));

    input.y = boo_as_f32(action_state.pressed(Action::Up))
        - boo_as_f32(action_state.pressed(Action::Down));

    if input == Vec2::ZERO {
        tree.set_vec2("Idle", input);
        tree.track("Idle");
    } else {
        tree.set_vec2("Run", input);
        tree.track("Run");
    }
}

fn spawn_main(mut commands: Commands, player_asset: Res<PlayerAsset>) {
    PlayerPlugin::spawn_player(&mut commands, &player_asset);
}

impl PlayerPlugin {
    pub fn spawn_player(commands: &mut Commands, player_asset: &PlayerAsset) -> Entity {
        let animate_player = Player::animate_player();
        let animate_tree = Player::animate_tree();

        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: player_asset.sprite_handle.clone(),
                ..Default::default()
            })
            .insert(animate_player)
            .insert(animate_tree)
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

        player.cul_animate = "RunDown".to_owned();

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

    pub fn animate_tree() -> SpriteAnimateTree {
        let mut tree = SpriteAnimateTree::default();

        let mut run_node = SpriteAnimateVec2::default();

        run_node.add_frame_animate("RunLeft", Vec2::new(-1.0, 0.0));
        run_node.add_frame_animate("RunDown", Vec2::new(0.0, -1.0));
        run_node.add_frame_animate("RunRight", Vec2::new(1.0, 0.0));
        run_node.add_frame_animate("RunUp", Vec2::new(0.0, 1.0));

        run_node.set_node_name("Run");

        tree.add_node(run_node);

        tree
    }
}
