use std::collections::HashMap;

use super::send_client_input::send_client_input;
use crate::yew_app::components::game::combat_log::combat_log_message::CombatLogMessage;
use crate::yew_app::components::game::combat_log::combat_log_message::CombatLogMessageStyle;
use crate::yew_app::store::game_store::GameStore;
use crate::yew_app::store::websocket_store::WebsocketStore;
use common::combatants::award_levelups::award_levelups;
use common::errors::AppError;
use common::packets::client_to_server::PlayerInputs;
use common::packets::server_to_client::BattleConclusion;
use common::packets::server_to_client::BattleEndReportPacket;
use yew::AttrValue;
use yewdux::Dispatch;

pub fn handle_battle_end_report(
    game_dispatch: Dispatch<GameStore>,
    websocket_dispatch: Dispatch<WebsocketStore>,
    packet: BattleEndReportPacket,
) -> Result<(), AppError> {
    websocket_dispatch.reduce_mut(|store| {
        let websocket_option = &store.websocket;
        if let Some(items) = &packet.loot {
            for item in items {
                send_client_input(
                    &websocket_option,
                    PlayerInputs::AcknowledgeReceiptOfItemOnGroundUpdate(item.entity_properties.id),
                );
            }
        }
    });
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        match packet.conclusion {
            BattleConclusion::Victory => {
                store.combat_log.push(CombatLogMessage::new(
                    AttrValue::from("battle ended in victory"),
                    CombatLogMessageStyle::BattleVictory,
                    0,
                ));
                let party = store.get_current_party_mut()?;
                if let Some(mut loot) = packet.loot {
                    party.current_room.items.append(&mut loot)
                }
                for (_, character) in party.characters.iter_mut() {
                    // @TODO - remove this when revive is implemented
                    if character.combatant_properties.hit_points == 0 {
                        character.combatant_properties.hit_points = 1;
                    }
                }
                party.current_room.monsters = None;
                party.battle_id = None;
                let mut entity_ids_and_previous_levels = HashMap::new();
                // HANDLE LEVELUPS
                if let Some(exp_changes) = packet.exp_changes {
                    for exp_change in exp_changes {
                        let entity_id = exp_change.combatant_id;

                        let party = store.get_current_party_mut()?;
                        let (entity_properties, combatant_properties) =
                            party.get_mut_combatant_by_id(&exp_change.combatant_id)?;
                        let combatant_level_before_exp_change = combatant_properties.level;
                        let entity_name = entity_properties.name.clone();
                        entity_ids_and_previous_levels
                            .insert(entity_id, combatant_level_before_exp_change);
                        if exp_change.experience_change > 0 {
                            combatant_properties.experience_points.current +=
                                exp_change.experience_change.abs() as u16;
                        } else {
                            combatant_properties.experience_points.current -=
                                exp_change.experience_change.abs() as u16;
                        }
                        award_levelups(combatant_properties);
                        store.combat_log.push(CombatLogMessage::new(
                            AttrValue::from(format!(
                                "{} gained {} experience points",
                                entity_name, exp_change.experience_change,
                            )),
                            CombatLogMessageStyle::PartyProgress,
                            0,
                        ));
                    }

                    for (id, level) in &entity_ids_and_previous_levels {
                        let party = store.get_current_party_mut()?;
                        let (entity_properties, combatant_properties) =
                            party.get_mut_combatant_by_id(&id)?;
                        let name = entity_properties.name.clone();
                        let new_level = combatant_properties.level;
                        if *level != new_level {
                            store.combat_log.push(CombatLogMessage::new(
                                AttrValue::from(format!("{} is now level {}", name, new_level)),
                                CombatLogMessageStyle::PartyProgress,
                                0,
                            ));
                        }
                    }
                }

                store.current_battle_id = None;
            }
            BattleConclusion::Defeat => {
                let party = store.get_current_party_mut()?;
                party.time_of_wipe = Some(js_sys::Date::now() as u64);
                store.combat_log.push(CombatLogMessage::new(
                    AttrValue::from("battle ended in defeat"),
                    CombatLogMessageStyle::PartyWipe,
                    0,
                ));
            }
        }
        //
        Ok(())
    })
}

// pub fn process_battle_victory_report(game_dispatch: Dispatch<GameStore>) -> Result<(), AppError> {
//     game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
//         if let Some(battle_end_report) = store.current_battle_end_report.clone() {
//             let party = store.get_current_party_mut()?;
//             match battle_end_report.conclusion {
//                 BattleConclusion::Victory => {
//                     party.current_room.monsters = None;
//                     party.battle_id = None;
//                     store.combat_log.push(CombatLogMessage::new(
//                         AttrValue::from("battle ended in victory"),
//                         CombatLogMessageStyle::BattleVictory,
//                         0,
//                     ));
//                     let mut entity_ids_and_previous_levels = HashMap::new();
//                     // HANDLE LEVELUPS
//                     if let Some(exp_changes) = battle_end_report.exp_changes {
//                         for exp_change in exp_changes {
//                             let entity_id = exp_change.combatant_id;
//                             let party = store.get_current_party_mut()?;
//                             let (entity_properties, combatant_properties) =
//                                 party.get_mut_combatant_by_id(&exp_change.combatant_id)?;
//                             let entity_name = entity_properties.name.clone();
//                             entity_ids_and_previous_levels
//                                 .insert(entity_id, combatant_properties.level);
//                             if exp_change.experience_change > 0 {
//                                 combatant_properties.experience_points.current +=
//                                     exp_change.experience_change.abs() as u16;
//                             } else {
//                                 combatant_properties.experience_points.current -=
//                                     exp_change.experience_change.abs() as u16;
//                             }
//                             award_levelups(combatant_properties);

//                             store.combat_log.push(CombatLogMessage::new(
//                                 AttrValue::from(format!(
//                                     "{} gained {} experience points",
//                                     entity_name, exp_change.experience_change,
//                                 )),
//                                 CombatLogMessageStyle::PartyProgress,
//                                 0,
//                             ));
//                         }

//                         for (id, level) in &entity_ids_and_previous_levels {
//                             let party = store.get_current_party_mut()?;
//                             let (entity_properties, combatant_properties) =
//                                 party.get_mut_combatant_by_id(&id)?;
//                             let name = entity_properties.name.clone();
//                             let new_level = combatant_properties.level;
//                             if *level != new_level {
//                                 store.combat_log.push(CombatLogMessage::new(
//                                     AttrValue::from(format!("{} is now level {}", name, new_level)),
//                                     CombatLogMessageStyle::PartyProgress,
//                                     0,
//                                 ));
//                             }
//                         }
//                     }

//                     store.current_battle_id = None;
//                 }
//                 BattleConclusion::Defeat => {
//                     party.time_of_wipe = Some(js_sys::Date::now() as u64);
//                     store.combat_log.push(CombatLogMessage::new(
//                         AttrValue::from("battle ended in defeat"),
//                         CombatLogMessageStyle::PartyWipe,
//                         0,
//                     ));
//                 }
//             }

//             store.current_battle_end_report = None;
//         }
//         Ok(())
//     })
// }
