use super::process_active_model_actions::ModelActionSystemParams;
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
    mut start_new_floating_text_event_reader: EventReader<StartNewAttackReactionEvent>,
    mut bevy_transmitter: ResMut<BevyTransmitter>,
    mut model_action_params: ModelActionSystemParams,
) {
    // start evade or hit recovery animation
    //   push hit recovery model_action to target
    let target_entity = model_action_params
        .combatants_by_id
        .0
        .get(entity_id)
        .expect("to have registered the entity");
    let mut target_combatant = model_action_params
        .combatants_query
        .get_mut(*target_entity)
        .expect("combatant entity to have an animation manager");
    target_combatant
        .model_action_queue
        .0
        .push_back(CombatantModelActions::HitRecovery);
    // EVADE

    let target_entity = model_action_params
        .combatants_by_id
        .0
        .get(entity_id)
        .expect("to have registered the entity");
    let mut target_combatant = model_action_params
        .combatants_query
        .get_mut(*target_entity)
        .expect("combatant entity to have an animation manager");
    target_combatant
        .model_action_queue
        .0
        .push_back(CombatantModelActions::Evade);
    // start floating text
    // notify yew of hp changes

    let target_entity = model_action_params
        .combatants_by_id
        .0
        .get(entity_id)
        .expect("to have registered the entity");
    let mut target_combatant = model_action_params
        .combatants_query
        .get_mut(*target_entity)
        .expect("target combatant to be in the query");
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
