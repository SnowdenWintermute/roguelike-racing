use crate::yew_app::store::game_store::GameStore;
use common::app_consts::INT_TO_MP_RATIO;
use common::app_consts::VIT_TO_HP_RATIO;
use common::combatants::combat_attributes::CombatAttributes;
use common::dungeon_rooms::DungeonRoomTypes;
use common::errors::AppError;
use common::game::getters::get_character;
use common::game::getters::get_mut_character;
use common::packets::CharacterAndDirection;
use common::packets::CharacterId;
use gloo::console::log;
use std::collections::HashSet;
use std::collections::VecDeque;
use yewdux::Dispatch;

pub fn new_dungeon_room_types_on_current_floor(
    game_dispatch: Dispatch<GameStore>,
    packet: VecDeque<Option<DungeonRoomTypes>>,
) -> Result<(), AppError> {
    log!("got new room types list");
    game_dispatch.reduce_mut(|store| {
        let party = store.get_current_party_mut()?;
        party.client_current_floor_rooms_list = packet;
        party.rooms_explored.on_current_floor = 0;
        Ok(())
    })
}

pub fn handle_player_toggled_ready_to_explore(
    game_dispatch: Dispatch<GameStore>,
    username: String,
) -> Result<(), AppError> {
    log!("player toggled ready to explore");
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let party = store.get_current_party_mut()?;
        if party.players_ready_to_descend.contains(&username) {
            party.players_ready_to_descend.remove(&username);
        }
        if party.players_ready_to_explore.contains(&username) {
            party.players_ready_to_explore.remove(&username);
        } else {
            party.players_ready_to_explore.insert(username.clone());
        };
        Ok(())
    })
}

pub fn handle_player_toggled_ready_to_descend(
    game_dispatch: Dispatch<GameStore>,
    username: String,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let party = store.get_current_party_mut()?;
        if party.players_ready_to_explore.contains(&username) {
            party.players_ready_to_explore.remove(&username);
        }
        if party.players_ready_to_descend.contains(&username) {
            party.players_ready_to_descend.remove(&username);
        } else {
            party.players_ready_to_descend.insert(username.clone());
        };
        Ok(())
    })
}

pub fn character_cycled_targets_handler(
    game_dispatch: Dispatch<GameStore>,
    packet: CharacterAndDirection,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|game_store| -> Result<(), AppError> {
        let party = game_store.get_current_party()?;
        let party_id = party.id;
        let game = game_store.get_current_game()?;
        let character = get_character(game, party_id, packet.character_id)?;
        let username = character.name_of_controlling_user.clone();
        let game = game_store.get_current_game_mut()?;
        game.cycle_character_targets(
            party_id,
            Some(HashSet::from([packet.character_id])), // trust that server sends valid packets
            &username,
            packet.character_id,
            &packet.direction,
        )?;

        Ok(())
    })
}

pub fn character_cycled_targeting_schemes_handler(
    game_dispatch: Dispatch<GameStore>,
    character_id: CharacterId,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|game_store| -> Result<(), AppError> {
        let party = game_store.get_current_party()?;
        let party_id = party.id;
        let game = game_store.get_current_game()?;
        let character = get_character(game, party_id, character_id)?;
        let username = character.name_of_controlling_user.clone();
        let game = game_store.get_current_game_mut()?;
        game.cycle_targeting_schemes(
            party_id,
            Some(HashSet::from([character_id])), // trust that server sends valid packets
            &username,
            character_id,
        )?;

        Ok(())
    })
}

pub fn character_spent_attribute_point_handler(
    game_dispatch: Dispatch<GameStore>,
    character_id: CharacterId,
    attribute: &CombatAttributes,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|game_store| {
        let party = game_store.get_current_party()?;
        let party_id = party.id;
        let game = game_store.get_current_game_mut()?;
        let character = get_mut_character(game, party_id, character_id)?;
        character.combatant_properties.unspent_attribute_points -= 1;
        let attribute_to_increment = character
            .combatant_properties
            .specced_attributes
            .entry(*attribute)
            .or_insert(0);
        *attribute_to_increment += 1;

        match &attribute {
            CombatAttributes::Intelligence => {
                character.combatant_properties.mana += INT_TO_MP_RATIO
            }
            CombatAttributes::Vitality => {
                character.combatant_properties.hit_points += VIT_TO_HP_RATIO
            }
            _ => (),
        }

        Ok(())
    })
}
