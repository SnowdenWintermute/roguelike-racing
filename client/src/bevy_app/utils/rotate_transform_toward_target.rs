use bevy::math::u64;
use bevy::prelude::*;

pub const TIME_TO_ROTATE: u64 = 1000;

pub fn rotate_transform_toward_target(
    transform_to_rotate: &mut Transform,
    start_rotation: &Quat,
    target_rotation: &Quat,
    elapsed: u64,
    time_to_complete: u64,
) -> f32 {
    let clamped_elapsed_rotation_time = std::cmp::min(elapsed, time_to_complete);
    let percent_of_complete_rotation =
        clamped_elapsed_rotation_time as f32 / time_to_complete as f32;
    transform_to_rotate.rotation =
        start_rotation.slerp(*target_rotation, percent_of_complete_rotation);

    percent_of_complete_rotation
}
