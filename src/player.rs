use bevy::prelude::*;
use sprite_animate_player::{FrameAnimate, SpriteAnimatePlayer};

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
