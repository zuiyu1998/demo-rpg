use bevy::{prelude::*, utils::HashMap};
use std::ops::{Deref, DerefMut};

pub mod nodes;

pub use nodes::*;

#[derive(Component, Default)]
pub struct SpriteAnimateTree {
    nodes: HashMap<String, Box<dyn SpriteAnimateNode>>,
    cul_node: String,
}

impl SpriteAnimateTree {
    pub fn add_node<N: SpriteAnimateNode>(&mut self, node: N) {
        self.insert(node.node_name(), Box::new(node));
    }

    pub fn set_vec2(&mut self, name: &str, control: Vec2) {
        if let Some(node) = self.get_mut(name) {
            node.set_vec2(control);
        }
    }

    pub fn track(&mut self, node: &str) {
        if self.cul_node != *node {
            let tmp = self.cul_node.to_owned();

            if let Some(node) = self.get_mut(&tmp) {
                node.reset();
            }
        }

        self.cul_node = node.to_owned();
    }

    pub(crate) fn get_frame_animate(&self) -> Option<String> {
        self.nodes
            .get(&self.cul_node)
            .and_then(|node| node.get_frame_animate())
    }
}

pub trait SpriteAnimateNode: 'static + Send + Sync {
    fn node_name(&self) -> String;

    fn get_frame_animate(&self) -> Option<String>;

    fn set_vec2(&mut self, _value: Vec2) {}

    fn reset(&mut self);
}

impl Deref for SpriteAnimateTree {
    type Target = HashMap<String, Box<dyn SpriteAnimateNode>>;

    fn deref(&self) -> &Self::Target {
        &self.nodes
    }
}

impl DerefMut for SpriteAnimateTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.nodes
    }
}
