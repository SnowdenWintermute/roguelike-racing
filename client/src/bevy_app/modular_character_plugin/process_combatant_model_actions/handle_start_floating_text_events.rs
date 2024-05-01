use super::process_active_model_actions::ModelActionSystemParams;
use super::FloatingText;
use crate::bevy_app::asset_loader_plugin::MyAssets;
use crate::bevy_app::modular_character_plugin::update_scene_aabbs::SceneAabb;
use crate::bevy_app::modular_character_plugin::StartNewFloatingTextEvent;
use bevy::prelude::*;
use bevy_mod_billboard::BillboardDepth;
use bevy_mod_billboard::BillboardTextBundle;
use js_sys::Date;

pub fn handle_start_floating_text_events(
    mut commands: Commands,
    mut model_action_params: ModelActionSystemParams,
    scenes_with_aabbs: Query<&SceneAabb>,
    asset_pack: Res<MyAssets>,
    mut start_floating_text_event_reader: EventReader<StartNewFloatingTextEvent>,
) {
    let font_handle = asset_pack
        .font_files
        .get("FiraSans-Regular.ttf")
        .expect("to have loaded the font");

    for StartNewFloatingTextEvent {
        combatant_entity,
        text,
        color,
        distance_to_travel,
        time_to_live,
        size,
    } in start_floating_text_event_reader.read()
    {
        let mut combatant = model_action_params
            .combatants_query
            .get_mut(*combatant_entity)
            .expect("to have a valid entity");

        let main_armature_entity_link = combatant.armature_link;
        let main_armature_scene_aabb = scenes_with_aabbs
            .get(main_armature_entity_link.0)
            .expect("to have an aabb for the main armature");
        let mut floating_text_start_location = Transform::from_xyz(0.0, 0.0, 0.0);
        floating_text_start_location.translation.y = main_armature_scene_aabb.max.y * 0.75;
        let font_size = match size {
            Some(size) => *size,
            None => 40.0,
        };

        let billboard_entity_commands = commands.spawn(BillboardTextBundle {
            transform: floating_text_start_location.with_scale(Vec3::splat(0.0125)),
            text: Text::from_sections([TextSection {
                value: format!("{}", text),
                style: TextStyle {
                    font_size,
                    font: font_handle.clone(),
                    color: Color::rgb_from_array(*color),
                },
            }]),
            billboard_depth: BillboardDepth(false),
            ..Default::default()
        });
        let billboard_entity = billboard_entity_commands.id();

        let target_skeleton_entity = combatant.skeleton_entity.0;
        let mut target_skeleton_commands = commands.entity(target_skeleton_entity);
        target_skeleton_commands.add_child(billboard_entity);

        let mut destination = floating_text_start_location.clone();
        let destination = if *distance_to_travel > 0.0 {
            destination.translation.y = main_armature_scene_aabb.max.y + distance_to_travel;
            Some(destination)
        } else {
            None
        };

        let new_floating_text = FloatingText {
            home_location: floating_text_start_location,
            destination,
            billboard_entity,
            time_started: Date::new_0().get_time() as u64,
            time_to_live: *time_to_live,
        };

        combatant
            .floating_text_component
            .0
            .insert(billboard_entity, new_floating_text);
    }
}
