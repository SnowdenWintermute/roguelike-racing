use super::model_actions::CombatantModelActions;
use super::process_active_model_actions::ModelActionSystemParams;
use super::FloatingText;
use super::FloatingTextComponent;
use crate::bevy_app::asset_loader_plugin::MyAssets;
use crate::bevy_app::modular_character_plugin::update_scene_aabbs::SceneAabb;
use crate::bevy_app::modular_character_plugin::StartNewAttackReactionEvent;
use crate::comm_channels::BevyTransmitter;
use crate::comm_channels::MessageFromBevy;
use bevy::prelude::*;
use bevy_mod_billboard::BillboardDepth;
use bevy_mod_billboard::BillboardTextBundle;
use js_sys::Date;

#[derive(Debug, Clone)]
pub enum AttackResult {
    HpChange(i16),
    Evade,
}

pub fn handle_new_attack_reaction_events(
    mut commands: Commands,
    mut start_new_attack_reactions_event_reader: EventReader<StartNewAttackReactionEvent>,
    bevy_transmitter: ResMut<BevyTransmitter>,
    mut model_action_params: ModelActionSystemParams,
    asset_pack: Res<MyAssets>,
    scenes_with_aabbs: Query<&SceneAabb>,
) {
    for event in start_new_attack_reactions_event_reader.read() {
        let StartNewAttackReactionEvent {
            entity_id,
            attack_result,
        } = event;

        let target_entity = model_action_params
            .combatants_by_id
            .0
            .get(entity_id)
            .expect("to have registered the entity");
        let mut target_combatant = model_action_params
            .combatants_query
            .get_mut(*target_entity)
            .expect("combatant entity to have an animation manager");

        let (text, color) = match attack_result {
            AttackResult::HpChange(number) => {
                // start hit recovery model action
                target_combatant
                    .model_action_queue
                    .0
                    .push_back(CombatantModelActions::HitRecovery);
                // send message to yew
                let _result = bevy_transmitter
                    .0
                    .send(MessageFromBevy::HpChangeById(*entity_id, *number));
                // return text to float
                (
                    number.abs().to_string(),
                    if *number > 0 {
                        Vec3::new(0.0, 1.0, 0.0)
                    } else {
                        Vec3::new(1.0, 1.0, 1.0)
                    },
                )
            }
            AttackResult::Evade => {
                // start evade model action
                target_combatant
                    .model_action_queue
                    .0
                    .push_back(CombatantModelActions::Evade);
                // send message to yew (for combat log)
                let _result = bevy_transmitter
                    .0
                    .send(MessageFromBevy::CombatantEvadedAttack(*entity_id));
                // return text to float
                (String::from("Evaded"), Vec3::new(1.0, 1.0, 1.0))
            }
        };

        let font_handle = asset_pack
            .font_files
            .get("FiraSans-Regular.ttf")
            .expect("to have loaded the font");

        let main_armature_entity_link = target_combatant.armature_link;
        let main_armature_scene_aabb = scenes_with_aabbs
            .get(main_armature_entity_link.0)
            .expect("to have an aabb for the main armature");
        let mut hp_change_text_start_location = Transform::from_xyz(0.0, 0.0, 0.0);
        hp_change_text_start_location.translation.y = main_armature_scene_aabb.max.y * 0.75;

        let billboard_entity_commands = commands.spawn(BillboardTextBundle {
            transform: hp_change_text_start_location.with_scale(Vec3::splat(0.0125)),
            text: Text::from_sections([TextSection {
                value: format!("{}", text),
                style: TextStyle {
                    font_size: 50.0,
                    font: font_handle.clone(),
                    color: Color::rgb_from_array(color),
                },
            }]),
            billboard_depth: BillboardDepth(false),
            ..Default::default()
        });
        let billboard_entity = billboard_entity_commands.id();

        let target_skeleton_entity = target_combatant.skeleton_entity.0;
        let mut target_skeleton_commands = commands.entity(target_skeleton_entity);
        target_skeleton_commands.add_child(billboard_entity);

        let mut destination = hp_change_text_start_location.clone();
        destination.translation.y = main_armature_scene_aabb.max.y + 1.5;

        let new_floating_text = FloatingText {
            value: text,
            home_location: hp_change_text_start_location,
            destination,
            billboard_entity,
            time_started: Date::new_0().get_time() as u64,
            color,
        };

        if let Some(mut floating_text_component) = target_combatant.floating_text_option {
            floating_text_component.0.push(new_floating_text);
        } else {
            commands
                .entity(*target_entity)
                .insert(FloatingTextComponent(Vec::from([new_floating_text])));
        }
    }
}
