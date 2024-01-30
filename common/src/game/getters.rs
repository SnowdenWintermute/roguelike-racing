use super::RoguelikeRacerGame;
use super::RoguelikeRacerPlayer;
use crate::adventuring_party::AdventuringParty;
use crate::app_consts::error_messages;
use crate::character::Character;
use crate::combat::battle::Battle;
use crate::combat::combat_actions::CombatActionTarget;
use crate::combat::combat_actions::FriendOrFoe;
use crate::errors::AppError;
use crate::errors::AppErrorTypes;

pub fn get_mut_player<'a>(
    game: &'a mut RoguelikeRacerGame,
    username: &String,
) -> Result<&'a mut RoguelikeRacerPlayer, AppError> {
    let player = game.players.get_mut(username).ok_or_else(|| AppError {
        error_type: AppErrorTypes::ServerError,
        message: error_messages::PLAYER_NOT_FOUND.to_string(),
    })?;
    Ok(player)
}

pub fn get_player<'a>(
    game: &'a RoguelikeRacerGame,
    username: String,
) -> Result<&'a RoguelikeRacerPlayer, AppError> {
    let player = game.players.get(&username).ok_or_else(|| AppError {
        error_type: AppErrorTypes::ServerError,
        message: error_messages::PLAYER_NOT_FOUND.to_string(),
    })?;
    Ok(player)
}

pub fn get_mut_party<'a>(
    game: &'a mut RoguelikeRacerGame,
    party_id: u32,
) -> Result<&'a mut AdventuringParty, AppError> {
    let party = game
        .adventuring_parties
        .get_mut(&party_id)
        .ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::ServerError,
            message: error_messages::PARTY_NOT_FOUND.to_string(),
        })?;
    Ok(party)
}

pub fn get_party<'a>(
    game: &'a RoguelikeRacerGame,
    party_id: u32,
) -> Result<&'a AdventuringParty, AppError> {
    let party = game
        .adventuring_parties
        .get(&party_id)
        .ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::ServerError,
            message: error_messages::PARTY_NOT_FOUND.to_string(),
        })?;
    Ok(party)
}

pub fn get_mut_character<'a>(
    game: &'a mut RoguelikeRacerGame,
    party_id: u32,
    character_id: u32,
) -> Result<&'a mut Character, AppError> {
    let party = get_mut_party(game, party_id)?;
    let character = party
        .characters
        .get_mut(&character_id)
        .ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::ServerError,
            message: error_messages::CHARACTER_NOT_FOUND.to_string(),
        })?;
    Ok(character)
}

pub fn get_character<'a>(
    game: &'a RoguelikeRacerGame,
    party_id: u32,
    character_id: u32,
) -> Result<&'a Character, AppError> {
    let party = get_party(game, party_id)?;
    let character = party
        .characters
        .get(&character_id)
        .ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::ServerError,
            message: error_messages::CHARACTER_NOT_FOUND.to_string(),
        })?;
    Ok(character)
}

pub fn get_ally_ids_and_opponent_ids_option(
    ally_ids: &Vec<u32>,
    battle_option: Option<&Battle>,
    combatant_id: u32,
) -> Result<(Vec<u32>, Option<Vec<u32>>), AppError> {
    if let Some(battle) = battle_option {
        battle.get_ally_ids_and_opponent_ids_option(combatant_id)
    } else {
        Ok((ally_ids.to_vec(), None))
    }
}

impl RoguelikeRacerGame {
    pub fn get_battle_option(
        &self,
        battle_id_option: &Option<u32>,
    ) -> Result<Option<Battle>, AppError> {
        if let Some(battle_id) = battle_id_option {
            Ok(Some(
                self.battles
                    .get(&battle_id)
                    .ok_or_else(|| AppError {
                        error_type: AppErrorTypes::ServerError,
                        message: error_messages::BATTLE_NOT_FOUND.to_string(),
                    })?
                    .clone(),
            ))
        } else {
            Ok(None)
        }
    }
    pub fn get_ids_from_ability_target(
        &self,
        party_id: u32,
        battle_option: Option<&Battle>,
        ability_target: &CombatActionTarget,
        ability_user_id: u32,
    ) -> Result<Vec<u32>, AppError> {
        let ids = match ability_target {
            CombatActionTarget::Single(id) => vec![*id],
            CombatActionTarget::Group(friend_or_foe) => match friend_or_foe {
                FriendOrFoe::Friendly => {
                    if let Some(battle) = battle_option {
                        let (ally_battle_group, _) =
                            battle.get_ally_and_enemy_battle_groups(&ability_user_id)?;
                        ally_battle_group.combatant_ids
                    } else {
                        let party = get_party(self, party_id)?;
                        party.character_positions.clone()
                    }
                }
                FriendOrFoe::Hostile => {
                    let battle = battle_option.ok_or_else(|| AppError {
                        error_type: AppErrorTypes::ClientError,
                        message: error_messages::BATTLE_NOT_FOUND.to_string(),
                    })?;
                    let (_, enemy_battle_group) =
                        battle.get_ally_and_enemy_battle_groups(&ability_user_id)?;
                    enemy_battle_group.combatant_ids
                }
            },
            CombatActionTarget::All => {
                if let Some(battle) = battle_option {
                    let (ally_battle_group, enemy_battle_group) =
                        battle.get_ally_and_enemy_battle_groups(&ability_user_id)?;
                    let mut ally_ids = ally_battle_group.combatant_ids.clone();
                    ally_ids.append(&mut enemy_battle_group.combatant_ids.clone());
                    ally_ids
                } else {
                    let party = get_party(self, party_id)?;
                    party.character_positions.clone()
                }
            }
        };
        Ok(ids)
    }
}
