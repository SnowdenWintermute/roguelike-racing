use bevy::prelude::*;

/// returns percent of complete translation toward destination as a value between 0.0 and 1.0
pub fn translate_transform_toward_target(
    mut transform_to_translate: &mut Transform,
    start_location: &Transform,
    destination: &Transform,
    elapsed: u64,
    time_to_complete: u64,
) -> f32 {
    let clamped_elapsed_translation_time = std::cmp::min(elapsed, time_to_complete);
    let percent_of_complete_translation =
        clamped_elapsed_translation_time as f32 / time_to_complete as f32;
    transform_to_translate.translation = start_location
        .translation
        .lerp(destination.translation, percent_of_complete_translation);

    percent_of_complete_translation
}
