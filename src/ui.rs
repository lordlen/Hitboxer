use bevy::{input::{keyboard::KeyboardInput, ButtonState}, prelude::*};
use bevy_egui::{egui, EguiContexts};
use rfd::FileDialog;
use std::fs;

use crate::{hitboxer::{CursorLocation, Hitboxer}, hitboxes::{HitboxAtlas, HitboxIndex, Hitboxes, COLOR_TEXT}};

#[derive(Resource)]
pub struct TextureAtlasState {
    tile_size: Vec2,
    padding: Vec2,
    offset: Vec2,
    columns: usize,
    rows: usize,
    pub index: usize
}

impl Default for TextureAtlasState {
    fn default() -> Self {
        TextureAtlasState {
            tile_size: Vec2 { x: 32.0, y: 32.0 },
            padding: Vec2 { x: 0.0, y: 0.0 },
            offset: Vec2 { x: 0.0, y: 0.0 },
            rows: 1,
            columns: 1,
            index: 0,
        }
    }
}

impl TextureAtlasState {
    fn length(&self) -> usize {
        self.rows * self.columns
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct ColorIndex(usize);

fn cyclical_increment(val: usize, length: usize) -> usize {
    if val + 1 < length {
        val + 1
    } else {
        0
    }
}
fn cyclical_decrement(val: usize, length: usize) -> usize {
    if val == 0 {
        length - 1
    } else {
        val - 1
    }
}

pub fn change_index(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut texture_atlas_state: ResMut<TextureAtlasState>,
    mut texture_atlas_query: Query<&mut TextureAtlas>
){
    for event in keyboard_input_events.read() {
        let curr_index = texture_atlas_state.index;
        if texture_atlas_query.is_empty() { return }
        let mut texture_atlas = texture_atlas_query.single_mut();
        if event.key_code == KeyCode::ArrowLeft && event.state == ButtonState::Pressed {
            let next_index = cyclical_decrement(curr_index, texture_atlas_state.length());
            texture_atlas.index = next_index;
            texture_atlas_state.index = next_index;
        } else if event.key_code == KeyCode::ArrowRight && event.state == ButtonState::Pressed {
            let next_index = cyclical_increment(curr_index, texture_atlas_state.length());
            texture_atlas.index = next_index;
            texture_atlas_state.index = next_index;
        }
    }
}


pub fn color_index_incrementer(mut color_index: ResMut<ColorIndex>, mut keyboard_input: EventReader<KeyboardInput>,) {
    for keyboard_event in keyboard_input.read() {
        if keyboard_event.state == ButtonState::Pressed {
            if keyboard_event.key_code == KeyCode::KeyQ {
                **color_index = cyclical_decrement(**color_index, COLOR_TEXT.len());
            } else if keyboard_event.key_code == KeyCode::KeyW {
                **color_index = cyclical_increment(**color_index, COLOR_TEXT.len());
            }
        }
    }
}


pub fn panels(
    mut ctx: EguiContexts,
    color_index: Res<ColorIndex>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut texture_atlas_state: ResMut<TextureAtlasState>,
    hitboxer_query: Query<Entity, With<Hitboxer>>,
    mut hitboxes_query: Query<&mut HitboxAtlas>,
    mut hitbox_index: Query<&mut HitboxIndex>,
    mouse_coords: Res<CursorLocation>,
) {
    egui::SidePanel::right("hitbox_info").show(ctx.ctx_mut(), |ui| {
        ui.heading("Hitboxes");

        ui.horizontal(|ui| {
            ui.label("Current color: ");
            ui.label(COLOR_TEXT[**color_index]);
        });

        ui.label("Use Q and W to switch colors");

        if ui.button("Texture Atlas").clicked() {
            if let Some(path) = FileDialog::new().pick_file() {
                let texture = asset_server.load(path);
                let layout = TextureAtlasLayout::from_grid(
                    texture_atlas_state.tile_size,
                    texture_atlas_state.columns,
                    texture_atlas_state.rows,
                    Some(texture_atlas_state.padding),
                    Some(texture_atlas_state.offset));
                let texture_atlas_layout = texture_atlas_layouts.add(layout);
                texture_atlas_state.index = 0;
                let sprite_bundle = SpriteSheetBundle {
                    texture,
                    atlas: TextureAtlas {
                        layout: texture_atlas_layout,
                        index: texture_atlas_state.index,
                    },
                    ..default()
                };
                // delete old hitboxers
                for entity in hitboxer_query.iter() {
                    commands.entity(entity).despawn();
                }

                // spawn in new hitboxer
                commands.spawn((
                    Hitboxer,
                    HitboxAtlas(vec![Hitboxes::default(); texture_atlas_state.columns * texture_atlas_state.rows]),
                    HitboxIndex(None),
                    sprite_bundle
                ));
            }
        }

        if ui.button("Save Hitboxes").clicked() {
            let hitbox_atlas = hitboxes_query.single();
            if let (Ok(json_contents), Some(path)) = (serde_json::to_string(&*hitbox_atlas), FileDialog::new().add_filter("json", &["json"]).save_file()) {
                fs::write(path, json_contents).expect("Unable to write file");
            }
            
        }

        if ui.button("Load Hitboxes").clicked() {
            let mut hitbox_atlas = hitboxes_query.single_mut();
            if let Some(path) = FileDialog::new().add_filter("json", &["json"]).pick_file() {
                let data = fs::read_to_string(path).expect("Could not load file");
                *hitbox_atlas = serde_json::from_str(&data).expect("Could not convert hitbox atlas");
            }
        }

        for mut hitbox_atlas in hitboxes_query.iter_mut() {
            let mut hitbox_ind = hitbox_index.single_mut();
            **hitbox_ind = None;
            let mut hitbox_to_remove: Option<(usize, usize)> = None;
            for (color_index, hitboxes) in (**hitbox_atlas)[texture_atlas_state.index].iter_mut().enumerate() {
                ui.label(COLOR_TEXT[color_index]);
                for (hb_ind, _) in hitboxes.iter_mut().enumerate() {
                    let label = ui.label(format!("{}", hb_ind));
                    if label.clicked() {
                        // remove the hitbox
                        hitbox_to_remove = Some((color_index, hb_ind));
                    } else if label.hovered() {
                        **hitbox_ind = Some((color_index, hb_ind));
                    }
                }
            }
            if let Some((color_ind, remove_ind)) = hitbox_to_remove {
                hitbox_atlas.remove_hitbox(texture_atlas_state.index, color_ind, remove_ind);
            }
        }
    });

    egui::SidePanel::left("texture_atlas_settings").show(ctx.ctx_mut(), |ui| {
        ui.heading("Texture Atlas Settings");
        ui.add(egui::Slider::new(&mut texture_atlas_state.tile_size.x, 0.0..=1028.0).text("Tile Width"));
        ui.add(egui::Slider::new(&mut texture_atlas_state.tile_size.y, 0.0..=1028.0).text("Tile Height"));
        ui.add(egui::Slider::new(&mut texture_atlas_state.padding.x, 0.0..=1028.0).text("Padding X"));
        ui.add(egui::Slider::new(&mut texture_atlas_state.padding.y, 0.0..=1028.0).text("Padding Y"));
        ui.add(egui::Slider::new(&mut texture_atlas_state.offset.x, 0.0..=1028.0).text("Offset X"));
        ui.add(egui::Slider::new(&mut texture_atlas_state.offset.y, 0.0..=1028.0).text("Offset Y"));
        ui.add(egui::Slider::new(&mut texture_atlas_state.columns, 0..=1028).text("Columns"));
        ui.add(egui::Slider::new(&mut texture_atlas_state.rows, 0..=1028).text("Rows"));
        ui.horizontal(|ui| {
            ui.label("Texture Atlas Index: ");
            ui.label(texture_atlas_state.index.to_string());
        });
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.horizontal(|ui| {
                ui.label("y: ");
                ui.label(mouse_coords.y.to_string());
            });
            ui.horizontal(|ui| {
                ui.label("x: ");
                ui.label(mouse_coords.x.to_string());
            });
        });
        
    });
}