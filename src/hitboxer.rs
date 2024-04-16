use bevy::{ecs::event::EventReader, input::mouse::MouseWheel, prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContexts;

use crate::{hitboxes::{HitboxAtlas, COLOR_ORDER}, ui::{ColorIndex, TextureAtlasState}, MainCamera};


#[derive(Resource, Default, Deref, DerefMut)]
pub struct CursorLocation(Vec2);

#[derive(Resource, Default)]
pub struct BoxCreation {
    pub init_location: Vec2,
    pub curr_location: Vec2,
}

#[derive(Component)]
pub struct Hitboxer;

pub fn box_creation(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut box_creation_state: ResMut<BoxCreation>,
    curr_location: Res<CursorLocation>,
    mut egui_contexts: EguiContexts,
    mut hitbox_query: Query<&mut HitboxAtlas>,
    texture_atlas_state: Res<TextureAtlasState>,
    color_index: Res<ColorIndex>
) {
    let ctx = egui_contexts.ctx_mut();
    if ctx.is_pointer_over_area() {
        return;
    }

    if mouse_button_input.pressed(MouseButton::Left) {
        box_creation_state.curr_location = **curr_location;
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        box_creation_state.init_location = **curr_location;
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        let hitboxes = hitbox_query.get_single_mut();

        match hitboxes {
            Ok(mut hb_list) => {
                let rect = Rect::from_corners(box_creation_state.init_location, box_creation_state.curr_location);
                hb_list.add_hitbox(rect, texture_atlas_state.index, **color_index);
            },
            _ => {}
        }

        box_creation_state.curr_location = Vec2::ZERO;
        box_creation_state.init_location = Vec2::ZERO;
    }
}

pub fn draw_box_creation(mut gizmos: Gizmos, box_creation_state: Res<BoxCreation>, color_index: Res<ColorIndex>) {
    let rect = Rect::from_corners(box_creation_state.init_location, box_creation_state.curr_location);
    gizmos.rect_2d(rect.center(), 0.0, rect.half_size() * 2.0, COLOR_ORDER[**color_index])
}

pub fn my_cursor_system(
    mut mouse_coords: ResMut<CursorLocation>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        **mouse_coords = world_position;
    }
}

pub fn zoom_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut projection: Query<&mut OrthographicProjection>,
) {
    for mouse_wheel in mouse_wheel_events.read() {
        let mut projection = projection.single_mut();
        if mouse_wheel.y > 0.0 {
            projection.scale /= 1. + mouse_wheel.y * 0.25;
        } else {
            projection.scale *= 1. - mouse_wheel.y * 0.25;
        }
        
    }
}