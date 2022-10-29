use bevy::{log, prelude::*, utils::HashMap};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Component)]
pub struct SpriteAnimatePlayer {
    frame_animates: HashMap<String, FrameAnimate>,
    pub cul_animate: String,
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
            self.index = self.indexs.len() - 1;
        }
    }

    pub fn is_end(&self) -> bool {
        if self.index == self.indexs.len() - 1 {
            return true;
        } else {
            return false;
        }
    }

    fn index(&self) -> usize {
        self.indexs[self.index]
    }
}

impl SpriteAnimatePlayer {
    pub fn play(&mut self, animate: &str) {
        if self.cul_animate != *animate {
            self.reset_animate(animate);
            self.cul_animate = animate.to_owned();
        }
    }

    pub fn get_frame_index(&self) -> Option<usize> {
        self.get(&self.cul_animate)
            .and_then(|frame| Some(frame.index()))
    }

    pub fn get_frame(&self) -> Option<&FrameAnimate> {
        self.get(&self.cul_animate).and_then(|frame| Some(frame))
    }

    fn reset_animate(&mut self, node: &str) {
        if let Some(frame) = self.get_mut(node) {
            frame.reset();
        }
    }

    pub(crate) fn update(&mut self, delta_time: f32) {
        let cul_animate = self.cul_animate.to_owned();

        if let Some(frame) = self.get_mut(&cul_animate) {
            frame.next(delta_time)
        } else {
            log::warn!("{} frame animate not found", cul_animate);
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
