use super::get_percent_animation_completed::get_percent_animation_completed;
use super::model_actions::get_animation_name_from_model_action;
use super::model_actions::CombatantModelActions;
use super::process_active_model_actions::ModelActionCombatantQueryStructItem;
use super::process_active_model_actions::ModelActionSystemParams;
use crate::bevy_app::bevy_app_consts::UNKNOWN_ANIMATION_DURATION;
use bevy::prelude::*;

pub fn animation_only_model_action_processor(
    entity: Entity,
    elapsed: u64,
    model_action_params: &mut ModelActionSystemParams,
    model_action: &CombatantModelActions,
) {
    let ModelActionCombatantQueryStructItem {
        skeleton_entity,
        combatant_properties_component,
        mut active_model_actions,
        ..
    } = model_action_params
        .combatants_query
        .get_mut(entity)
        .expect("to have this entity in the query");
    let species_component = model_action_params
        .species_query
        .get(skeleton_entity.0)
        .expect("the skeleton to have a species");
    // check percent completed of animation
    let percent_completed = if let Some(animation_name) = get_animation_name_from_model_action(
        &species_component.0,
        model_action,
        &combatant_properties_component.0,
    ) {
        get_percent_animation_completed(
            &skeleton_entity.0,
            &model_action_params.animation_player_links,
            &model_action_params.animation_players,
            &model_action_params.animations,
            &model_action_params.assets_animation_clips,
            &animation_name,
        )
    } else {
        elapsed as f32 / UNKNOWN_ANIMATION_DURATION as f32
    };

    if percent_completed >= 1.0 {
        let removed = active_model_actions.0.remove(model_action);
        info!(
            "removing model action {:?}, got removed: {:?}",
            model_action, removed
        );
    }
}
