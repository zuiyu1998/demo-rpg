use bevy::prelude::Vec2;

use super::SpriteAnimateNode;

#[derive(Debug, Default)]
pub struct SpriteAnimateVec2 {
    node_name: String,
    frame_animates: Vec<(Vec2, String)>,
    cul_control: Vec2,
    next_node_name: String,
}

impl SpriteAnimateVec2 {
    pub fn add_frame_animate(&mut self, frame_animate: &str, position: Vec2) {
        self.frame_animates
            .push((position, frame_animate.to_owned()))
    }

    pub fn set_control(&mut self, control: Vec2) {
        self.cul_control = control;
    }

    pub fn set_node_name(&mut self, name: &str) {
        self.node_name = name.to_owned();
    }
}

impl SpriteAnimateNode for SpriteAnimateVec2 {
    fn node_name(&self) -> String {
        self.node_name.to_owned()
    }

    fn set_next_node_name(&mut self, node: &str) {
        self.next_node_name = node.to_owned();
    }

    fn next_node_name(&self) -> String {
        self.next_node_name.to_owned()
    }

    fn set_vec2(&mut self, control: Vec2) {
        self.set_control(control);
    }

    fn get_frame_animate(&self) -> Option<String> {
        if self.frame_animates.is_empty() {
            return None;
        }

        let new_distances = self
            .frame_animates
            .iter()
            .map(|item| {
                return item.0.distance_squared(self.cul_control);
            })
            .collect::<Vec<f32>>();

        let mut min = new_distances[0];
        let mut min_index = 0;

        new_distances.iter().enumerate().for_each(|(index, item)| {
            if *item < min {
                min_index = index;
                min = *item;
            }
        });

        return Some(self.frame_animates[min_index].1.to_owned());
    }

    fn reset(&mut self) {
        self.cul_control = Vec2::ZERO;
    }
}
