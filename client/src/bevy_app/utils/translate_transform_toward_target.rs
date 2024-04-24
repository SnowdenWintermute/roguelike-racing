use bevy::prelude::*;

/// returns percent of complete translation toward destination as a value between 0.0 and 1.0
pub fn translate_transform_toward_target(
    transform_to_translate: &mut Transform,
    start_location: &Transform,
    destination: &Transform,
    elapsed: u64,
    time_to_complete: f32,
) -> f32 {
    if start_location == destination {
        return 1.0;
    }
    let percent_of_complete_translation = elapsed as f32 / time_to_complete;
    transform_to_translate.translation = start_location
        .translation
        .lerp(destination.translation, percent_of_complete_translation);

    percent_of_complete_translation
}
