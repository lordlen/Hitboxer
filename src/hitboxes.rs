use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ui::TextureAtlasState;

pub const COLOR_TEXT: [&str; 5] = ["Green", "Red", "Yellow", "Orange", "Blue"];
pub const COLOR_ORDER: [Color; 5] = [Color::GREEN, Color::RED, Color::YELLOW, Color::ORANGE, Color::BLUE];

#[derive(Debug, Deref, DerefMut, Clone, Deserialize, Serialize)]
pub struct Hitboxes (pub Vec<Vec<Rect>>);

#[derive(Component, Debug, Deref, DerefMut, Clone)]
pub struct HitboxIndex(pub Option<(usize, usize)>);

#[derive(Component, Deref, DerefMut, Deserialize, Serialize, Debug)]
pub struct HitboxAtlas (pub Vec<Hitboxes>);

impl HitboxAtlas {
    pub fn draw_current_hitbox(&self, gizmos: &mut Gizmos, index: usize) {
        (*self)[index].draw_hitboxes(gizmos)
    }

    pub fn add_hitbox(&mut self, rect: Rect, frame_index: usize, color_index: usize) {
        (**self)[frame_index].add_hitbox(rect, color_index);
    }

    pub fn remove_hitbox(&mut self, frame_index: usize, color_index: usize, hitbox_index: usize) -> Rect {
        (**self)[frame_index].remove_hitbox(color_index, hitbox_index)
    }
}

impl Default for Hitboxes {
    fn default() -> Self {
        Hitboxes(vec![vec![]; COLOR_TEXT.len()])
    }
}

impl Hitboxes {
    pub fn draw_hitboxes(&self, gizmos: &mut Gizmos) {
        for (i, hitbox_list) in self.iter().enumerate() {
            let color = COLOR_ORDER[i];
            for hitbox in hitbox_list {
                gizmos.rect_2d(hitbox.center(), 0.0, hitbox.half_size() * 2.0, color);
            }
        }
    }

    pub fn draw_hitbox(&self, gizmos: &mut Gizmos, color_ind: usize, hitbox_ind: usize) {
        let hitbox = (*self)[color_ind][hitbox_ind];
        let color = COLOR_ORDER[color_ind];
        gizmos.rect_2d(hitbox.center(), 0.0, hitbox.half_size() * 2.0, color);
    }

    pub fn add_hitbox(&mut self, rect: Rect, color_ind: usize) {
        (**self)[color_ind].push(rect);
    }

    pub fn remove_hitbox(&mut self, color_ind: usize, hitbox_ind: usize) -> Rect {
        (**self)[color_ind].remove(hitbox_ind)
    }
}

pub fn draw_hitbox_atlas(
    mut gizmos: Gizmos,
    query: Query<(&HitboxAtlas, &HitboxIndex)>,
    texture_atlas_state: Res<TextureAtlasState>,
) {
    for (hitbox_atlas, hitbox_index) in query.iter() {
        match **hitbox_index {
            None => hitbox_atlas.draw_current_hitbox(&mut gizmos, texture_atlas_state.index),
            Some((color_index, hb_ind)) => (*hitbox_atlas)[texture_atlas_state.index].draw_hitbox(&mut gizmos, color_index, hb_ind)
        }
        
    }
}
