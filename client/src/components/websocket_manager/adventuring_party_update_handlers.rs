use crate::store::game_store::GameStore;
use common::{
    app_consts::error_messages,
    errors::AppError,
    game::getters::{get_mut_party, get_mut_player},
    packets::server_to_client::{
        AdventuringPartyCreation, NewCharacterInParty, PlayerAdventuringPartyChange,
        PlayerCharacterDeletion,
    },
};
use std::collections::HashSet;

pub fn handle_adventuring_party_created(
    game_state: &mut GameStore,
    party_creation: AdventuringPartyCreation,
) -> Result<(), AppError> {
    let game = game_state.game.as_mut().ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ClientError,
        message: "Client error".to_string(),
    })?;

    game.add_adventuring_party(party_creation.party_name, party_creation.party_id);
    game.put_player_in_adventuring_party(
        party_creation.party_id,
        party_creation.username_created_by.clone(),
    )?;
    Ok(())
}

pub fn handle_player_changed_adventuring_party(
    game_state: &mut GameStore,
    update: PlayerAdventuringPartyChange,
) -> Result<(), AppError> {
    let game = game_state.game.as_mut().ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ClientError,
        message: "Client error".to_string(),
    })?;
    let _ = game.remove_player_from_adventuring_party(update.username.clone());
    if let Some(party_id) = update.party_id {
        if let Some(_party) = game.adventuring_parties.get(&party_id) {
            return game.put_player_in_adventuring_party(party_id, update.username.clone());
        }
    }
    Ok(())
}

pub fn handle_character_creation(
    game_state: &mut GameStore,
    character_creation: NewCharacterInParty,
) -> Result<(), AppError> {
    let game = game_state.game.as_mut().ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ClientError,
        message: "Client error".to_string(),
    })?;
    let party = get_mut_party(game, character_creation.party_id)?;
    let character_id = character_creation.character.entity_properties.id;
    party.characters.insert(
        character_creation.character.entity_properties.id,
        character_creation.character,
    );

    let player = get_mut_player(game, character_creation.username.clone())?;
    match &mut player.character_ids {
        None => {
            let mut new_ids = HashSet::new();
            new_ids.insert(character_id);
            player.character_ids = Some(new_ids);
        }
        Some(ids) => {
            ids.insert(character_id);
        }
    }

    Ok(())
}

pub fn handle_character_deletion(
    game_state: &mut GameStore,
    character_deletion: PlayerCharacterDeletion,
) -> Result<(), AppError> {
    let game = game_state.game.as_mut().ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ClientError,
        message: "Client error".to_string(),
    })?;
    let party = get_mut_party(game, character_deletion.party_id)?;
    party.characters.remove(&character_deletion.character_id);
    let player = get_mut_player(game, character_deletion.username.clone())?;
    let player_character_ids_option = player.character_ids.clone();
    let mut player_character_ids = player_character_ids_option.ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ServerError,
        message: error_messages::PLAYER_HAS_NO_CHARACTERS.to_string(),
    })?;

    player_character_ids.remove(&character_deletion.character_id);

    let player = get_mut_player(game, character_deletion.username.clone())?;
    if player_character_ids.len() > 1 {
        player.character_ids = Some(player_character_ids);
    } else {
        player.character_ids = None
    }

    Ok(())
}