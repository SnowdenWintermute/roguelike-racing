// use crate::bevy_app::{
//     modular_character_plugin::{
//         animation_manager_component::AnimationManagerComponent,
//         spawn_combatant::MainSkeletonEntity, Animations, CombatantsById,
//         CombatantsExecutingAttacks, HomeLocation,
//     },
//     utils::link_animations::AnimationEntityLink,
// };
// use bevy::prelude::*;
// use js_sys::Date;
// use std::time::Duration;

// const TIME_TO_ROTATE: u64 = 500;
// const TIME_TO_RETURN: u64 = 10000;
// const TIME_TO_STRIKE: u64 = 1500;
// const SPEED_MODIFIER: f32 = 0.2;

// pub fn move_entities_toward_destinations(
//     combatants_by_id: Res<CombatantsById>,
//     mut combatants: Query<(
//         &MainSkeletonEntity,
//         &mut AnimationManagerComponent,
//         &HomeLocation,
//     )>,
//     mut transforms: Query<&mut Transform>,
//     combatants_with_active_action_states: ResMut<CombatantsExecutingAttacks>,
//     animation_player_links: Query<&AnimationEntityLink>,
//     mut animation_players: Query<&mut AnimationPlayer>,
//     animations: Res<Animations>,
//     assets_animation_clips: Res<Assets<AnimationClip>>,
// ) {
//     for combatant_id in combatants_with_active_action_states.0.iter() {
//         let combatant_entity = combatants_by_id
//             .0
//             .get(combatant_id)
//             .expect("to have the combatant");
//         let (skeleton_entity, mut animation_manager, home_location) = combatants
//             .get_mut(*combatant_entity)
//             .expect("to have the combatant");
//         if animation_manager.destination.is_some()
//             && animation_manager.current_animation_name != "Run_Back"
//         {
//             let destination = animation_manager.destination.unwrap();
//             let mut combatant_transform = transforms
//                 .get_mut(skeleton_entity.0)
//                 .expect("to have the transform");
//             let up = *combatant_transform.up().clone();
//             let target_rotation = combatant_transform
//                 .looking_at(destination.translation, up)
//                 .rotation;

//             let time_started = animation_manager
//                 .time_started
//                 .expect("to have marked the start time");
//             let current_time = Date::new_0().get_time() as u64;
//             let elapsed = current_time - time_started;
//             let clamped_elapsed = std::cmp::min(elapsed, TIME_TO_ROTATE);
//             let clamped_translation_time = std::cmp::min(elapsed, TIME_TO_STRIKE);
//             let percent_of_complete_rotation = clamped_elapsed as f32 / TIME_TO_ROTATE as f32;
//             let percent_of_complete_translation =
//                 clamped_translation_time as f32 / TIME_TO_STRIKE as f32;

//             combatant_transform.rotation = home_location
//                 .0
//                 .rotation
//                 .slerp(target_rotation, percent_of_complete_rotation);
//             combatant_transform.translation = home_location
//                 .0
//                 .translation
//                 .lerp(destination.translation, percent_of_complete_translation);

//             if percent_of_complete_translation >= 0.8
//                 && animation_manager.current_animation_name != "Sword_Slash"
//             {
//                 animation_manager.current_animation_name = "Sword_Slash".to_string();
//                 let animation_player_link = animation_player_links
//                     .get(skeleton_entity.0)
//                     .expect("to have an animation player link");
//                 let mut animation_player = animation_players
//                     .get_mut(animation_player_link.0)
//                     .expect("to have a player");
//                 let animation_handle = animations
//                     .0
//                     .get("Sword_Slash")
//                     .expect("to have this animation");
//                 animation_player
//                     .play_with_transition(animation_handle.clone(), Duration::from_millis(500))
//                     .set_speed(SPEED_MODIFIER);
//             }
//         } else if animation_manager.current_animation_name == "Sword_Slash" {
//             let animation_player_link = animation_player_links
//                 .get(skeleton_entity.0)
//                 .expect("to have an animation player link");
//             let mut animation_player = animation_players
//                 .get_mut(animation_player_link.0)
//                 .expect("to have a player");
//             let sword_slash_animation_handle = animations
//                 .0
//                 .get("Sword_Slash")
//                 .expect("to have this animation");
//             let animation_handle = animations
//                 .0
//                 .get("Run_Back")
//                 .expect("to have this animation");
//             let animation_clip = assets_animation_clips
//                 .get(sword_slash_animation_handle)
//                 .expect("to have the clip");
//             let percent_completed =
//                 (animation_player.elapsed() * SPEED_MODIFIER) / animation_clip.duration();
//             info!("percent completed: {:?}", percent_completed);
//             if percent_completed >= 0.65 {
//                 animation_manager.last_location = animation_manager.destination.take();
//                 animation_manager.destination = Some(home_location.0);
//                 animation_manager.current_animation_name = "Run_Back".to_string();
//                 animation_manager.time_started = Some(Date::new_0().get_time() as u64);
//                 animation_player
//                     .play_with_transition(animation_handle.clone(), Duration::from_millis(5000))
//                     .repeat()
//                     .set_speed(SPEED_MODIFIER);
//             }
//         } else if animation_manager.current_animation_name == "Run_Back" {
//             let current_time = Date::new_0().get_time() as u64;
//             let elapsed = current_time - animation_manager.time_started.unwrap();
//             let clamped_elapsed = std::cmp::min(elapsed, TIME_TO_RETURN);
//             let mut combatant_transform = transforms
//                 .get_mut(skeleton_entity.0)
//                 .expect("to have the transform");
//             let percent_of_complete_translation = clamped_elapsed as f32 / TIME_TO_RETURN as f32;
//             info!("percent of completed: {percent_of_complete_translation}");
//             combatant_transform.translation = animation_manager
//                 .last_location
//                 .expect("to have a last location")
//                 .translation
//                 .lerp(home_location.0.translation, percent_of_complete_translation);
//         }
//     }
// }
