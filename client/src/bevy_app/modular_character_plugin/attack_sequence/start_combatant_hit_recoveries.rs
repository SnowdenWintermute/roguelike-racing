use crate::bevy_app::asset_loader_plugin::MyAssets;
use crate::bevy_app::modular_character_plugin::animation_manager_component::ActionSequenceStates;
use crate::bevy_app::modular_character_plugin::animation_manager_component::AnimationManagerComponent;
use crate::bevy_app::modular_character_plugin::animation_manager_component::HpChangeNumber;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantMainArmatureEntityLink;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantSpeciesComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonEntity;
use crate::bevy_app::modular_character_plugin::update_scene_aabbs::SceneAabb;
use crate::bevy_app::modular_character_plugin::Animations;
use crate::bevy_app::modular_character_plugin::CombatantsById;
use crate::bevy_app::modular_character_plugin::CombatantsExecutingAttacks;
use crate::bevy_app::modular_character_plugin::HitRecoveryActivationEvent;
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use crate::frontend_common::animation_names::AnimationType;
use crate::frontend_common::animation_names::CombatantAnimations;
use crate::frontend_common::CombatantSpecies;
use bevy::prelude::*;
use bevy_mod_billboard::BillboardDepth;
use bevy_mod_billboard::BillboardTextBundle;
use js_sys::Date;
use std::time::Duration;

pub fn start_combatant_hit_recoveries(
    mut commands: Commands,
    combatants_by_id: Res<CombatantsById>,
    mut combatants: Query<(
        &MainSkeletonEntity,
        &mut AnimationManagerComponent,
        &CombatantMainArmatureEntityLink,
    )>,
    species_query: Query<&CombatantSpeciesComponent>,
    mut combatants_with_active_action_states: ResMut<CombatantsExecutingAttacks>,
    animation_player_links: Query<&AnimationEntityLink>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    mut hit_recovery_activation_event_reader: EventReader<HitRecoveryActivationEvent>,
    asset_pack: Res<MyAssets>,
    scenes_with_aabbs: Query<&SceneAabb>,
) {
    let current_time = Date::new_0().get_time() as u64;
    for event in hit_recovery_activation_event_reader.read() {
        info!("read hit recovery event");
        let HitRecoveryActivationEvent(targets_and_damages) = event;
        for (target_id, damage) in targets_and_damages {
            combatants_with_active_action_states.0.insert(*target_id);

            let target_entity = combatants_by_id
                .0
                .get(target_id)
                .expect("to have the entity");
            let (skeleton_entity, mut animation_manager, main_armature_entity_link) = combatants
                .get_mut(*target_entity)
                .expect("to have the combatant");

            let species = species_query
                .get(skeleton_entity.0)
                .expect("to have a species");
            animation_manager
                .active_states
                .insert(ActionSequenceStates::HitRecovery, Some(current_time));

            // damage number
            let font_handle = asset_pack
                .font_files
                .get("FiraSans-Regular.ttf")
                .expect("to have loaded the font");

            let main_armature_scene_aabb = scenes_with_aabbs
                .get(main_armature_entity_link.0)
                .expect("to have an aabb for the main armature");
            let mut hp_change_text_start_location = Transform::from_xyz(0.0, 0.0, 0.0);
            hp_change_text_start_location.translation.y = main_armature_scene_aabb.max.y * 0.75;

            let billboard_entity_commands = commands.spawn(BillboardTextBundle {
                transform: hp_change_text_start_location.with_scale(Vec3::splat(0.003)),
                text: Text::from_sections([TextSection {
                    value: format!("{}", damage),
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

            let mut target_skeleton_commands = commands.entity(skeleton_entity.0);
            target_skeleton_commands.add_child(billboard_entity);

            let mut destination = hp_change_text_start_location.clone();
            destination.translation.y = main_armature_scene_aabb.max.y + 0.75;

            animation_manager.hp_change_numbers.push(HpChangeNumber {
                value: *damage,
                home_location: hp_change_text_start_location,
                destination,
                entity: billboard_entity,
                time_started: current_time,
            });

            // animation
            let animation_player_link = animation_player_links
                .get(skeleton_entity.0)
                .expect("to have linked the skeleton to it's animation player");
            let mut animation_player = animation_players
                .get_mut(animation_player_link.0)
                .expect("to have a valid animation player entity in the link");

            let anim_name = species.0.animation_name(AnimationType::HitRecovery);

            let animation_handle = animations
                .0
                .get(&anim_name)
                .expect("to have a the animation");
            animation_player
                .start_with_transition(animation_handle.clone(), Duration::from_millis(500));
        }
    }
}
