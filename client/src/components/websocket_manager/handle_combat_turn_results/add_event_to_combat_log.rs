// use crate::components::mesh_manager::ClientCombatantEvent;
// use common::combat::battle::Battle;
// use common::errors::AppError;
// use common::game::RoguelikeRacerGame;
// use yew::AttrValue;

// pub fn get_combat_log_entry(
//     game: &RoguelikeRacerGame,
//     associated_combatant_id: u32,
//     combatant_event: &ClientCombatantEvent,
//     party_id: u32,
//     battle_option: Option<&Battle>,
// ) -> Result<AttrValue, AppError> {
//     let entry = match combatant_event {
//         ClientCombatantEvent::HpChange(hp_change) => {
//             let (entity_properties, _) = game.get_combatant_by_id(&associated_combatant_id)?;
//             AttrValue::from(match hp_change.signum() {
//                 -1 => format!("{} took {} damage", entity_properties.name, hp_change * -1),
//                 1 => format!("{} was healed for {} hp", entity_properties.name, hp_change),
//                 _ => format!("impossible number"),
//             })
//         }
//         ClientCombatantEvent::Died(hp_change) => {
//             let (entity_properties, _) = game.get_combatant_by_id(&associated_combatant_id)?;
//             AttrValue::from(format!("took {} damage and {} died", hp_change * -1,entity_properties.name))
//         }
//         ClientCombatantEvent::TookAction(action_result) => {
//             let target_ids = game.get_ids_from_ability_target(
//                 party_id,
//                 battle_option,
//                 &action_result.targets,
//                 associated_combatant_id,
//             )?;
//             let mut target_names = vec![];
//             for (i, id) in target_ids.iter().enumerate() {
//                 let (entity_properties, _) = game.get_combatant_by_id(id)?;
//                 target_names.push(entity_properties.name.as_str());
//                 if i != (target_ids.len() - 1) {
//                     target_names.push(", ");
//                 }
//             }

//             let target_names_as_string = target_names.join("");

//             let associated_user = game.get_combatant_by_id(&associated_combatant_id)?;
//             let ended_turn_text = if action_result.ends_turn {
//                 ", ending their turn"
//             } else {
//                 ""
//             };
//             AttrValue::from(format!(
//                 "{} used {} targeting {}{}",
//                 associated_user.0.name,
//                 action_result.action,
//                 target_names_as_string,
//                 ended_turn_text
//             ))
//         }
//     };

//     Ok(entry)
// }
