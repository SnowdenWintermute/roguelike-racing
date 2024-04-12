use crate::bevy_app::modular_character_plugin::HomeLocation;
use crate::comm_channels::MessageFromYew;
use crate::frontend_common::CombatantSpecies;
use crate::yew_app::components::mesh_manager::CombatantEventManager;
use crate::yew_app::store::bevy_communication_store::BevyCommunicationStore;
use crate::yew_app::store::game_store::GameStore;
use bevy::transform::components::Transform;
use common::app_consts::error_messages;
use common::app_consts::COMBATANT_POSITION_SPACING_BETWEEN_ROWS;
use common::app_consts::COMBATANT_POSITION_SPACING_SIDE;
use common::dungeon_rooms::DungeonRoom;
use common::errors::AppError;
use common::errors::AppErrorTypes;
use std::f32::consts::PI;
use std::rc::Rc;
use yewdux::Dispatch;

pub fn handle_new_dungeon_room(
    game_dispatch: Dispatch<GameStore>,
    bevy_communication_dispatch: Dispatch<BevyCommunicationStore>,
    packet: DungeonRoom,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|game_store| {
        if let Some(monsters) = &packet.monsters {
            let mut monster_home_location = HomeLocation(Transform::from_xyz(
                -COMBATANT_POSITION_SPACING_SIDE,
                0.0,
                COMBATANT_POSITION_SPACING_BETWEEN_ROWS / 2.0,
            ));

            for (monster_id, monster) in monsters {
                game_store
                    .action_results_manager
                    .combantant_event_managers
                    .insert(*monster_id, CombatantEventManager::new(*monster_id));
                // send messages to spawn enemy combatants
                bevy_communication_dispatch.reduce_mut(|store| -> Result<(), AppError> {
                    let transmitter =
                        store.transmitter_option.as_ref().ok_or_else(|| AppError {
                            error_type: AppErrorTypes::ClientError,
                            message: error_messages::NO_YEW_TRANSMITTER_TO_BEVY.to_string(),
                        })?;
                    let species = CombatantSpecies::Frog;

                    transmitter
                        .send(MessageFromYew::SpawnCharacterWithHomeLocation(
                            *monster_id,
                            monster_home_location.clone(),
                            species,
                        ))
                        .expect("could not send event");
                    Ok(())
                })?;
                monster_home_location.0.translation.x += COMBATANT_POSITION_SPACING_SIDE;
            }
        }
        let party = game_store.get_current_party_mut()?;

        // SPAWN CHARACTER MODELS
        let cloned_character_positions = party.character_positions.clone();
        bevy_communication_dispatch.reduce_mut(|store| -> Result<(), AppError> {
            let transmitter = store.transmitter_option.as_ref().ok_or_else(|| AppError {
                error_type: AppErrorTypes::ClientError,
                message: error_messages::NO_YEW_TRANSMITTER_TO_BEVY.to_string(),
            })?;
            let mut character_home_location = HomeLocation(Transform::from_xyz(
                -COMBATANT_POSITION_SPACING_SIDE,
                0.0,
                -COMBATANT_POSITION_SPACING_BETWEEN_ROWS / 2.0,
            ));
            character_home_location.0.rotate_y(PI);

            for character_id in cloned_character_positions {
                let species = CombatantSpecies::Humanoid;

                transmitter
                    .send(MessageFromYew::SpawnCharacterWithHomeLocation(
                        character_id,
                        character_home_location.clone(),
                        species,
                    ))
                    .expect("could not send event");
                character_home_location.0.translation.x += COMBATANT_POSITION_SPACING_SIDE;
            }
            Ok(())
        })?;
        //

        party.players_ready_to_explore.clear();
        party.players_ready_to_descend.clear();
        let current_room_type = packet.room_type;
        party.current_room = packet;
        party.rooms_explored.on_current_floor += 1;
        let num_rooms_explored_on_current_floor = party.rooms_explored.on_current_floor;
        party.rooms_explored.total += 1;
        let room_to_reveal = party
            .client_current_floor_rooms_list
            .get_mut((num_rooms_explored_on_current_floor - 1).into())
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::CLIENT_LIST_MISSING_ROOM_TYPE.to_string(),
            })?;
        *room_to_reveal = Some(current_room_type);

        Ok(())
    })
}
