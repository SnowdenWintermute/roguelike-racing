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

    for (i, (entity, mut floating_text_component)) in floating_text_query.iter_mut().enumerate() {
        let mut indices_to_remove = Vec::new();
        for floating_text in floating_text_component.0.iter() {
            if let Ok(mut transform) = transforms.get_mut(floating_text.billboard_entity) {
                let elapsed = current_time - floating_text.time_started;
                let percent_complete = elapsed as f32 / floating_text.time_to_live as f32;

                transform.translation = floating_text
                    .home_location
                    .translation
                    .lerp(floating_text.destination.translation, percent_complete);

                if percent_complete >= 1.0 {
                    indices_to_remove.push(i);
                }
            };
        }

        for i in indices_to_remove {
            let removed = floating_text_component.0.remove(i);
            let billboard_entity_commands = commands.entity(removed.billboard_entity);
            billboard_entity_commands.despawn_recursive();
        }

        if floating_text_component.0.len() == 0 {
            commands.entity(entity).remove::<FloatingTextComponent>();
        }
    }
}
