use bevy::prelude::*;

pub mod animate_player;
pub mod animate_tree;

pub use animate_player::*;
pub use animate_tree::*;

pub struct SpriteAnimatePlugin;

#[derive(Debug, PartialEq, Eq, Hash, SystemLabel)]
pub enum AnimateLabel {
    Animate,
}

impl Plugin for SpriteAnimatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate.label(AnimateLabel::Animate));
    }
}

pub fn animate(
    mut normal_query: Query<
        (&mut TextureAtlasSprite, &mut SpriteAnimatePlayer),
        Without<SpriteAnimateTree>,
    >,

    mut tree_query: Query<(
        &mut TextureAtlasSprite,
        &mut SpriteAnimatePlayer,
        &SpriteAnimateTree,
    )>,
    time: Res<Time>,
) {
    let mut iter = vec![];

    for (sprite, mut player, tree) in tree_query.iter_mut() {
        if let Some(animate_name) = tree.get_frame_animate() {
            player.play(&animate_name);
        }

        iter.push((sprite, player));
    }

    iter.append(&mut normal_query.iter_mut().collect());

    for (mut sprite, mut player) in iter.into_iter() {
        let delta_time = time.delta_seconds();

        player.update(delta_time);

        if let Some(index) = player.get_frame_index() {
            sprite.index = index;
        }
    }
}
