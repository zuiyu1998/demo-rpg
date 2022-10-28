use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use heron::prelude::*;
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

fn player_control(
    mut player_state_query: Query<(&ActionState<Action>, &mut Velocity), With<Player>>,
    mut tree_query: Query<&mut SpriteAnimateTree, With<Player>>,
) {
    let (action_state, mut velocity) = player_state_query.single_mut();
    let mut tree = tree_query.single_mut();

    let mut input = Vec2::ZERO;

    input.x = boo_as_f32(action_state.pressed(Action::Right))
        - boo_as_f32(action_state.pressed(Action::Left));

    input.y = boo_as_f32(action_state.pressed(Action::Up))
        - boo_as_f32(action_state.pressed(Action::Down));

    let tmp_velocity = velocity.linear;

    if input == Vec2::ZERO {
        tree.set_vec2("Idle", input);
        tree.track("Idle");
    } else {
        tree.set_vec2("Run", input);
        tree.track("Run");
    }

    *velocity = Velocity::from_linear(tmp_velocity);
}

fn spawn_main(mut commands: Commands, player_asset: Res<PlayerAsset>) {
    PlayerPlugin::spawn_player(&mut commands, &player_asset);
}

impl PlayerPlugin {
    pub fn spawn_player(commands: &mut Commands, player_asset: &PlayerAsset) -> Entity {
        commands
            .spawn()
            .insert_bundle(TransformBundle::default())
            .insert_bundle(VisibilityBundle::default())
            .insert(Player)
            .insert(RigidBody::Dynamic)
            .insert(Velocity::from_linear(Vec3::ZERO))
            .insert_bundle(InputManagerBundle::<Action> {
                action_state: ActionState::default(),
                input_map: InputMap::new([
                    (KeyCode::A, Action::Left),
                    (KeyCode::D, Action::Right),
                    (KeyCode::W, Action::Up),
                    (KeyCode::S, Action::Down),
                ]),
            })
            .with_children(|parent| {
                let animate_player = Player::animate_player();
                let animate_tree = Player::animate_tree();

                parent
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: player_asset.sprite_handle.clone(),
                        ..Default::default()
                    })
                    .insert(Player)
                    .insert(animate_player)
                    .insert(animate_tree);

                parent
                    .spawn()
                    .insert(CollisionShape::Cuboid {
                        half_extends: Vec3::new(6.0, 11.5, 0.0),
                        border_radius: None,
                    })
                    .insert_bundle(VisibilityBundle::default())
                    .insert_bundle(TransformBundle {
                        local: Transform {
                            translation: Vec3::new(-1.0, 1.0, 1.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
            })
            .insert(Name::new("Player"))
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

#[derive(Component)]
pub struct Profile {}

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
