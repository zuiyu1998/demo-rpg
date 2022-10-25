use bevy::{log, prelude::*, utils::HashMap};
use std::ops::{Deref, DerefMut};

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
    mut query: Query<(&mut TextureAtlasSprite, &mut SpriteAnimatePlayer)>,
    time: Res<Time>,
) {
    for (mut sprite, mut player) in query.iter_mut() {
        let delta_time = time.delta_seconds();

        player.update(delta_time);

        if let Some(index) = player.get_frame_index() {
            sprite.index = index;
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct SpriteAnimatePlayer {
    frame_animates: HashMap<String, FrameAnimate>,
    pub cul_frame: String,
}

#[derive(Debug, Default)]
pub struct FrameAnimate {
    pub indexs: Vec<usize>,
    pub index: usize,
    pub is_loop: bool,
    pub frame_time: f32,
    delta_time: f32,
}

impl FrameAnimate {
    pub fn new(indexs: Vec<usize>, index: usize, is_loop: bool, frame_time: f32) -> Self {
        FrameAnimate {
            indexs,
            index,
            is_loop,
            frame_time,
            delta_time: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.delta_time = 0.0;
        self.index = 0;
    }

    fn next(&mut self, delta_time: f32) {
        let count = ((self.delta_time + delta_time) / self.frame_time) as usize;

        self.delta_time = (self.delta_time + delta_time) % self.frame_time;

        let index = self.index + count;

        if index < self.indexs.len() {
            self.index = index;
        } else if self.is_loop {
            self.index = index % self.indexs.len();
        } else {
            self.index = self.indexs.len();
        }
    }

    fn index(&self) -> usize {
        self.indexs[self.index]
    }
}

impl SpriteAnimatePlayer {
    pub fn play(&mut self, node: &str) {
        if self.cul_frame != *node {
            self.reset_animate(node);
            self.cul_frame = node.to_owned();
        }
    }

    pub fn get_frame_index(&self) -> Option<usize> {
        self.get(&self.cul_frame)
            .and_then(|frame| Some(frame.index()))
    }

    fn reset_animate(&mut self, node: &str) {
        if let Some(frame) = self.get_mut(node) {
            frame.reset();
        }
    }

    fn update(&mut self, delta_time: f32) {
        let cul_frame_name = self.cul_frame.to_owned();

        if let Some(frame) = self.get_mut(&cul_frame_name) {
            frame.next(delta_time)
        } else {
            log::warn!("{} frame animate not found", cul_frame_name);
        }
    }

    pub fn add(&mut self, animate_name: &str, frame_animate: FrameAnimate) {
        self.insert(animate_name.to_owned(), frame_animate);
    }
}

impl Deref for SpriteAnimatePlayer {
    type Target = HashMap<String, FrameAnimate>;

    fn deref(&self) -> &Self::Target {
        &self.frame_animates
    }
}

impl DerefMut for SpriteAnimatePlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.frame_animates
    }
}
