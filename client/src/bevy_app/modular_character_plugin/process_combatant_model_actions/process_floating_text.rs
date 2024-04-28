use super::FloatingTextComponent;
use bevy::prelude::*;
use js_sys::Date;

pub const FLOATING_TEXT_TIME_TO_LIVE_DEFAULT: f32 = 2000.0;

pub fn process_floating_text(
    mut commands: Commands,
    mut floating_text_query: Query<(Entity, &mut FloatingTextComponent)>,
    mut transforms: Query<&mut Transform>,
) {
    let current_time = Date::new_0().get_time() as u64;

    for (entity, mut floating_text_component) in floating_text_query.iter_mut() {
        let mut floating_text_entities_to_remove = Vec::new();
        for (billboard_entity, floating_text) in floating_text_component.0.iter() {
            if let Ok(mut transform) = transforms.get_mut(floating_text.billboard_entity) {
                let elapsed = current_time - floating_text.time_started;
                let percent_complete = elapsed as f32 / floating_text.time_to_live as f32;

                if let Some(destination) = floating_text.destination {
                    transform.translation = floating_text
                        .home_location
                        .translation
                        .lerp(destination.translation, percent_complete);
                }

                if percent_complete >= 1.0 {
                    floating_text_entities_to_remove.push(*billboard_entity);
                    let billboard_entity_commands = commands.entity(*billboard_entity);
                    billboard_entity_commands.despawn_recursive();
                }
            };
        }

        for billboard_entity in floating_text_entities_to_remove {
            floating_text_component.0.remove(&billboard_entity);
            info!("called despawn on {:?}", billboard_entity);
        }
    }
}
