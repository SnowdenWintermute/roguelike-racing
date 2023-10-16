use common::{
    errors::AppError,
    game::{getters::get_mut_party, getters::get_mut_player, RoguelikeRacerGame},
    packets::server_to_client::{
        AdventuringPartyCreation, NewCharacterInParty, PlayerAdventuringPartyChange,
    },
};
use leptos::*;

pub fn handle_adventuring_party_created(
    game: RwSignal<Option<RoguelikeRacerGame>>,
    party_creation: AdventuringPartyCreation,
) -> Result<(), AppError> {
    game.try_update(move |game_state| -> Result<(), AppError> {
        if let Some(game) = game_state {
            game.add_adventuring_party(party_creation.party_name, party_creation.party_id);
            game.put_player_in_adventuring_party(
                party_creation.party_id,
                party_creation.username_created_by.clone(),
            )?;
        }
        Ok(())
    })
    .ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ClientError,
        message: "Client error".to_string(),
    })?
}

pub fn handle_adventuring_party_removed(game: RwSignal<Option<RoguelikeRacerGame>>, party_id: u32) {
    game.update(move |game_state| {
        if let Some(game) = game_state {
            game.adventuring_parties.remove(&party_id);
        };
    })
}

pub fn handle_player_changed_adventuring_party(
    game: RwSignal<Option<RoguelikeRacerGame>>,
    update: PlayerAdventuringPartyChange,
) {
    game.update(move |game_state| {
        if let Some(game) = game_state {
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
        }
    })
}

pub fn handle_character_creation(
    game: RwSignal<Option<RoguelikeRacerGame>>,
    character_creation: NewCharacterInParty,
) -> Result<(), AppError> {
    game.try_update(move |game_state| -> Result<(), AppError> {
        if let Some(game) = game_state {
            let party = get_mut_party(game, character_creation.party_id)?;
            party.add_player_character(
                character_creation.character_id,
                character_creation.combatant_class.clone(),
                &character_creation.character_name,
            )?;

            let player = get_mut_player(game, character_creation.username.clone())?;
            match &mut player.character_ids {
                None => player.character_ids = Some(vec![character_creation.character_id]),
                Some(ids) => ids.push(character_creation.character_id),
            }
        }
        Ok(())
    })
    .ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ClientError,
        message: "Client error".to_string(),
    })?
}
