use super::{spawn_combatant::MainSkeletonEntity, Animations, CombatantsById};
use crate::{
    bevy_app::utils::link_animations::AnimationEntityLink, comm_channels::SelectAnimationEvent,
};
use bevy::prelude::*;
use std::time::Duration;

pub fn handle_animation_change_requests(
    mut animation_player_query: Query<&mut AnimationPlayer>,
    characters_with_animation_player_links: Query<&AnimationEntityLink, With<AnimationEntityLink>>,
    characters_by_id: Res<CombatantsById>,
    main_skeletons_query: Query<&MainSkeletonEntity>,
    animations: Res<Animations>,
    mut animation_change_request_event_reader: EventReader<SelectAnimationEvent>,
) {
    for event in animation_change_request_event_reader.read() {
        info!("reading event: {:#?}", event);
        if let Some(character_entity) = characters_by_id.0.get(&event.0.character_id) {
            info!("found character entity: {:#?}", character_entity);
            if let Ok(main_skeleton_entity) = main_skeletons_query.get(*character_entity) {
                if let Ok(animation_player_entity_link) =
                    characters_with_animation_player_links.get(main_skeleton_entity.0)
                {
                    info!(
                        "found animation player link: {:#?}",
                        animation_player_entity_link
                    );
                    if let Ok(mut animation_player) =
                        animation_player_query.get_mut(animation_player_entity_link.0)
                    {
                        info!("found animation player ");
                        if let Some(animation_clip_handle) = animations.0.get(&event.0.name) {
                            info!("found clip  handle: {:#?}", animation_clip_handle);
                            animation_player
                                .play_with_transition(
                                    animation_clip_handle.clone(),
                                    Duration::from_millis(500),
                                )
                                .repeat();
                        }
                    }
                }
            }
        }
    }
}
