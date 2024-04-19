use bevy::prelude::*;

pub fn process_floating_text() {
    let mut all_numbers_completed = true;
    for (i, hp_change_number) in animation_manager
        .hp_change_numbers
        .clone()
        .iter()
        .enumerate()
    {
        if let Ok(mut transform) = transforms.get_mut(hp_change_number.entity) {
            let elapsed = current_time - hp_change_number.time_started;
            let number_showing_percent_completed =
                elapsed as f32 / TIME_TO_SHOW_HP_CHANGE_NUMBER as f32;

            transform.translation = hp_change_number.home_location.translation.lerp(
                hp_change_number.destination.translation,
                number_showing_percent_completed,
            );

            if number_showing_percent_completed >= 1.0 {
                animation_manager.hp_change_numbers.remove(i);
                let billboard_entity_commands = commands.entity(hp_change_number.entity);
                billboard_entity_commands.despawn_recursive();
            } else {
                all_numbers_completed = false;
            }
        };
    }
}
