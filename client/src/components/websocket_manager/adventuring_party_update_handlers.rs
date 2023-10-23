use common::{
    errors::AppError,
    game::{
        getters::{get_mut_party, get_mut_player},
        RoguelikeRacerGame,
    },
    packets::server_to_client::{
        AdventuringPartyCreation, NewCharacterInParty, PlayerAdventuringPartyChange,
    },
};
use gloo::console::log;

use crate::store::game_store::GameStore;

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
            let _ = game.put_player_in_adventuring_party(party_id, update.username.clone());
        }
    } else {
        if let Ok(player) = get_mut_player(game, update.username.clone()) {
            player.party_id = None;
        };
    };
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
    party.add_player_character(
        character_creation.character_id,
        character_creation.combatant_class.clone(),
        &character_creation.character_name,
        character_creation.username.clone(),
    )?;

    let player = get_mut_player(game, character_creation.username.clone())?;
    match &mut player.character_ids {
        None => player.character_ids = Some(vec![character_creation.character_id]),
        Some(ids) => ids.push(character_creation.character_id),
    }
    Ok(())
}
