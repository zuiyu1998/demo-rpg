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

    pub(crate) fn get_next_node(&self) -> Option<String> {
        self.nodes
            .get(&self.cul_node)
            .and_then(|node| Some(node.next_node_name()))
    }
}

pub trait SpriteAnimateNode: 'static + Send + Sync {
    fn node_name(&self) -> String;

    fn set_next_node_name(&mut self, node: &str);

    fn next_node_name(&self) -> String {
        self.node_name()
    }

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
