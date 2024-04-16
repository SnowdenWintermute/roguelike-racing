use std::collections::HashMap;
use crate::bevy_app::modular_character_plugin::animation_manager_component::AnimationManagerComponent;
use crate::bevy_app::modular_character_plugin::handle_combat_turn_results::combatant_model_actions::{get_animation_name_from_model_action, CombatantModelActions};
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantActionResultsManagerComponent;
use crate::bevy_app::modular_character_plugin::Animations;
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use crate::bevy_app::utils::rotate_transform_toward_target::rotate_transform_toward_target;
use crate::bevy_app::utils::translate_transform_toward_target::translate_transform_toward_target;
use crate::frontend_common::CombatantSpecies;
use bevy::math::u64;
use bevy::prelude::*;
use common::items::equipment::EquipmentSlots;
use common::items::Item;

pub const UNKNOWN_ANIMATION_DURATION: u64 = 500;
pub const MH_MELEE_ANIMATION_DURATION_TRANSITION_THRESHOLD: f32 = 0.65;

pub fn attacking_with_melee_main_hand_processor(
    skeleton_entity_transform: &mut Transform,
    skeleton_entity: Entity,
    combatant_species: &CombatantSpecies,
    equipment: &HashMap<EquipmentSlots, Item>,
    animation_manager: &mut AnimationManagerComponent,
    action_result_manager: &mut CombatantActionResultsManagerComponent,
    home_location: &Transform,
    elapsed: u64,
    animations: &Res<Animations>,
    animation_players: &mut Query<&mut AnimationPlayer>,
    animation_player_links: &Query<&AnimationEntityLink>,
    assets_animation_clips: &Res<Assets<AnimationClip>>,
    transition_started: bool,
) {
    // check percent completed of animation
    let percent_completed = if let Some(animation_name) = get_animation_name_from_model_action(
        combatant_species,
        &CombatantModelActions::AttackMeleeMainHand,
    ) {
        let animation_player_link = animation_player_links
            .get(skeleton_entity)
            .expect("the skeleton to have an animation player link");
        let animation_player = animation_players
            .get(animation_player_link.0)
            .expect("the skeleton's animation entity link to have an animation player");
        let animation_handle = animations
            .0
            .get(&animation_name)
            .expect("to have this animation registered");
        let animation_clip = assets_animation_clips
            .get(animation_handle)
            .expect("to have the clip");
        animation_player.elapsed() / animation_clip.duration()
    } else {
        elapsed as f32 / UNKNOWN_ANIMATION_DURATION as f32
    };

    if percent_completed > MH_MELEE_ANIMATION_DURATION_TRANSITION_THRESHOLD && !transition_started {
        //   start next model_action and mark this one as transition completed
        animation_manager.start_next_model_action(
            animation_player_links,
            animation_players,
            animations,
            skeleton_entity,
            combatant_species,
            equipment,
            500,
        );
        animation_manager
            .active_model_actions
            .get_mut(&CombatantModelActions::ApproachMeleeTarget)
            .expect("this model action to be active")
            .transition_started = true;
        //   push hit recovery model_action to target
        //   send message to yew to update target's hp
    }

    // if animation time completed, remove this from active animations
    //
    //
    // let anim_name = species.animation_name(AnimationType::Attack);
    // // - if duration threshold passed, activate returning
    // let animation_handle = animations
    //     .0
    //     .get(&anim_name)
    //     .expect("to have this animation registered");
    // let animation_clip = assets_animation_clips
    //     .get(animation_handle)
    //     .expect("to have the clip");
    // let percent_completed = animation_player.elapsed() / animation_clip.duration();
}
