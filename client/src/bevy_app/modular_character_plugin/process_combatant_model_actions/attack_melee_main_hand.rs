use std::collections::HashMap;
use crate::bevy_app::asset_loader_plugin::MyAssets;
use crate::bevy_app::modular_character_plugin::animation_manager_component::{AnimationManagerComponent, HpChangeNumber};
use crate::bevy_app::modular_character_plugin::handle_combat_turn_results::combatant_model_actions::{get_animation_name_from_model_action, CombatantModelActions};
use crate::bevy_app::modular_character_plugin::spawn_combatant::{CombatantActionResultsManagerComponent, CombatantMainArmatureEntityLink, MainSkeletonEntity};
use crate::bevy_app::modular_character_plugin::update_scene_aabbs::SceneAabb;
use crate::bevy_app::modular_character_plugin::{Animations, CombatantsById};
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use crate::bevy_app::utils::rotate_transform_toward_target::rotate_transform_toward_target;
use crate::bevy_app::utils::translate_transform_toward_target::translate_transform_toward_target;
use crate::comm_channels::{BevyTransmitter, MessageFromBevy};
use crate::frontend_common::CombatantSpecies;
use bevy::math::u64;
use bevy::prelude::*;
use bevy_mod_billboard::{BillboardDepth, BillboardTextBundle};
use common::items::equipment::EquipmentSlots;
use common::items::Item;
use js_sys::Date;

pub const UNKNOWN_ANIMATION_DURATION: u64 = 500;
pub const MH_MELEE_ANIMATION_DURATION_TRANSITION_THRESHOLD: f32 = 0.65;

pub fn attacking_with_melee_main_hand_processor(
    commands: &mut Commands,
    entity: Entity,
    skeleton_entity_transform: &mut Transform,
    skeleton_entity: Entity,
    combatant_species: &CombatantSpecies,
    equipment: &HashMap<EquipmentSlots, Item>,
    action_result_manager: &mut CombatantActionResultsManagerComponent,
    home_location: &Transform,
    elapsed: u64,
    combatants_by_id: &Res<CombatantsById>,
    animations: &Res<Animations>,
    animation_managers: &mut Query<&mut AnimationManagerComponent>,
    animation_players: &mut Query<&mut AnimationPlayer>,
    animation_player_links: &Query<&AnimationEntityLink>,
    assets_animation_clips: &Res<Assets<AnimationClip>>,
    asset_pack: &Res<MyAssets>,
    scenes_with_aabbs: &Query<&SceneAabb>,
    main_armature_links: &Query<&CombatantMainArmatureEntityLink>,
    main_skeleton_links: &Query<&MainSkeletonEntity>,
    transition_started: bool,
    bevy_transmitter: &Res<BevyTransmitter>,
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
        info!("starting transition from mh melee attack");
        //   start next model_action and mark this one as transition completed
        let mut animation_manager = animation_managers
            .get_mut(entity)
            .expect("to have an animation manager");
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
            .get_mut(&CombatantModelActions::AttackMeleeMainHand)
            .expect("this model action to be active")
            .transition_started = true;

        let current_action = action_result_manager
            .current_action_result_processing
            .as_ref()
            .expect("to have a current action result processing");

        if let Some(hp_changes) = &current_action.hp_changes_by_entity_id {
            for (entity_id, hp_change) in hp_changes {
                let target_entity = combatants_by_id
                    .0
                    .get(entity_id)
                    .expect("to have registered the entity");
                let mut target_animation_manager = animation_managers
                    .get_mut(*target_entity)
                    .expect("combatant entity to have an animation manager");
                //   push hit recovery model_action to target
                target_animation_manager
                    .model_action_queue
                    .push_back(CombatantModelActions::HitRecovery);
                // hp change numbers
                let font_handle = asset_pack
                    .font_files
                    .get("FiraSans-Regular.ttf")
                    .expect("to have loaded the font");

                let main_armature_entity_link = main_armature_links
                    .get(*target_entity)
                    .expect("the target entity to have an armature link");
                let main_armature_scene_aabb = scenes_with_aabbs
                    .get(main_armature_entity_link.0)
                    .expect("to have an aabb for the main armature");
                let mut hp_change_text_start_location = Transform::from_xyz(0.0, 0.0, 0.0);
                hp_change_text_start_location.translation.y = main_armature_scene_aabb.max.y * 0.75;

                let billboard_entity_commands = commands.spawn(BillboardTextBundle {
                    transform: hp_change_text_start_location.with_scale(Vec3::splat(0.0125)),
                    text: Text::from_sections([TextSection {
                        value: format!("{}", hp_change.abs()),
                        style: TextStyle {
                            font_size: 50.0,
                            font: font_handle.clone(),
                            color: Color::WHITE,
                        },
                    }]),
                    billboard_depth: BillboardDepth(false),
                    ..Default::default()
                });
                let billboard_entity = billboard_entity_commands.id();

                let target_skeleton_entity = main_skeleton_links
                    .get(*target_entity)
                    .expect("the target to have a skeleton entity link");
                let mut target_skeleton_commands = commands.entity(target_skeleton_entity.0);
                target_skeleton_commands.add_child(billboard_entity);

                let mut destination = hp_change_text_start_location.clone();
                destination.translation.y = main_armature_scene_aabb.max.y + 1.5;

                let color_option = if *hp_change > 0 {
                    Some(Vec3::new(0.0, 1.0, 0.0))
                } else {
                    None
                };

                target_animation_manager
                    .hp_change_numbers
                    .push(HpChangeNumber {
                        value: hp_change.abs() as u16,
                        home_location: hp_change_text_start_location,
                        destination,
                        entity: billboard_entity,
                        time_started: Date::new_0().get_time() as u64,
                        color_option,
                    });

                //   send message to yew to update target's hp
                let _result = bevy_transmitter
                    .0
                    .send(MessageFromBevy::HpChangeById(*entity_id, *hp_change));
            }
            if let Some(misses) = &current_action.misses_by_entity_id {
                for entity_id in misses {
                    let target_entity = combatants_by_id
                        .0
                        .get(entity_id)
                        .expect("to have registered the entity");
                    let mut target_animation_manager = animation_managers
                        .get_mut(*target_entity)
                        .expect("combatant entity to have an animation manager");
                    target_animation_manager
                        .model_action_queue
                        .push_back(CombatantModelActions::Evade);
                }
            }
        }
    }

    // if animation time completed, remove this from active model actions
    if percent_completed >= 1.0 {
        let mut animation_manager = animation_managers
            .get_mut(entity)
            .expect("to have an animation manager");
        animation_manager
            .active_model_actions
            .remove(&CombatantModelActions::AttackMeleeMainHand);
    }
}
