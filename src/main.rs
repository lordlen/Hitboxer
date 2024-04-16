// use bevy::{math::bounding::{Aabb2d, BoundingVolume}, prelude::*};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use hitboxer::{box_creation, draw_box_creation, my_cursor_system, zoom_system, BoxCreation, CursorLocation};
use hitboxes::draw_hitbox_atlas;
use ui::{change_index, color_index_incrementer, panels, ColorIndex, TextureAtlasState};

mod hitboxes;
mod hitboxer;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()),)
        .add_plugins(EguiPlugin)
        .init_resource::<TextureAtlasState>()
        .init_resource::<CursorLocation>()
        .init_resource::<ColorIndex>()
        .init_resource::<BoxCreation>()
        .add_systems(Startup, setup)
        .add_systems(Update,  (
            (zoom_system,
            change_index,
            my_cursor_system,
            color_index_incrementer,
            box_creation,
            draw_box_creation,
            draw_hitbox_atlas).after(panels),
            panels,
        ))
        .run();
}

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;
/// We will store the world position of the mouse cursor here.

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}