mod turn_order;
use self::turn_order::CombatantTurnTracker;
use crate::adventuring_party::AdventuringParty;
use crate::app_consts::error_messages;
use crate::combatants::abilities::{AbilityTarget, CombatantAbility, CombatantAbilityNames};
use crate::combatants::CombatantProperties;
use crate::errors::{AppError, AppErrorTypes};
use crate::game::RoguelikeRacerGame;
use crate::items::consumables::ConsumableTypes;
use crate::items::Item;
use crate::primatives::EntityProperties;
use crate::status_effects::StatusEffects;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Battle {
    pub id: u32,
    pub group_a: BattleGroup,
    pub group_b: BattleGroup,
    pub combatant_turn_trackers: Vec<CombatantTurnTracker>,
}

impl Battle {
    pub fn combatant_is_first_in_turn_order(&self, entity_id: u32) -> bool {
        let first_turn_tracker_option = self.combatant_turn_trackers.first();
        if let Some(first_turn_tracker) = first_turn_tracker_option {
            return first_turn_tracker.entity_id == entity_id;
        } else {
            return false;
        }
    }

    pub fn get_ally_ids_and_opponent_ids_option(
        &self,
        combatant_id: u32,
    ) -> Result<(Vec<u32>, Option<Vec<u32>>), AppError> {
        let opponent_ids_option = if self.group_a.combatant_ids.contains(&combatant_id) {
            Some(self.group_b.combatant_ids.clone())
        } else if self.group_b.combatant_ids.contains(&combatant_id) {
            Some(self.group_a.combatant_ids.clone())
        } else {
            None
        };

        let ally_ids = if self.group_a.combatant_ids.contains(&combatant_id) {
            Some(self.group_a.combatant_ids.clone())
        } else if self.group_b.combatant_ids.contains(&combatant_id) {
            Some(self.group_b.combatant_ids.clone())
        } else {
            None
        }
        .ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::Generic,
            message: error_messages::ALLY_COMBATANTS_NOT_FOUND.to_string(),
        })?;

        Ok((ally_ids, opponent_ids_option))
    }

    pub fn is_id_of_existing_opponent(&self, combatant_id: u32, target_id: u32) -> bool {
        let mut to_return = false;
        if let Ok((ally_ids, opponent_ids_option)) =
            self.get_ally_ids_and_opponent_ids_option(combatant_id)
        {
            if let Some(opponent_ids) = opponent_ids_option {
                to_return = opponent_ids.contains(&target_id)
            }
        }
        to_return
    }
}

impl RoguelikeRacerGame {
    pub fn initiate_battle(
        &mut self,
        group_a: BattleGroup,
        group_b: BattleGroup,
    ) -> Result<(), AppError> {
        let turn_trackers = self.get_battle_turn_order(&group_a, &group_b)?;
        let mut battle = Battle {
            id: self.id_generator.get_next_entity_id(),
            group_a,
            group_b,
            combatant_turn_trackers: turn_trackers,
        };

        self.battles.insert(battle.id, battle);

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum BattleGroupTypes {
    #[default]
    PlayerControlled,
    ComputerControlled,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BattleGroup {
    pub name: String,
    pub party_id: u32,
    pub combatant_ids: Vec<u32>,
    pub group_type: BattleGroupTypes,
}

impl BattleGroup {
    pub fn get_battle_group_entity_belongs_to(
        ally_battle_group: BattleGroup,
        opponent_battle_group_option: Option<BattleGroup>,
        id: &u32,
    ) -> Result<BattleGroup, AppError> {
        if ally_battle_group.combatant_ids.contains(id) {
            Ok(ally_battle_group)
        } else if let Some(opponent_battle_group) = opponent_battle_group_option {
            if opponent_battle_group.combatant_ids.contains(id) {
                Ok(opponent_battle_group)
            } else {
                Err(AppError {
                    error_type: AppErrorTypes::Generic,
                    message: error_messages::COMBATANT_NOT_FOUND.to_string(),
                })
            }
        } else {
            Err(AppError {
                error_type: AppErrorTypes::Generic,
                message: error_messages::COMBATANT_NOT_FOUND.to_string(),
            })
        }
    }
}

impl RoguelikeRacerGame {
    pub fn get_mut_combatant_in_battle_group_by_id(
        &mut self,
        battle_group: &BattleGroup,
        id: u32,
    ) -> Result<(&mut EntityProperties, &mut CombatantProperties), AppError> {
        let party_option = self.adventuring_parties.get_mut(&battle_group.party_id);
        if let Some(party) = party_option {
            party.get_mut_combatant_by_id(id)
        } else {
            return Err(AppError {
                error_type: AppErrorTypes::ServerError,
                message: error_messages::PARTY_NOT_FOUND.to_string(),
            });
        }
    }
}

#[derive(Debug)]
pub enum CombatAction {
    AbilityUsed(CombatantAbility, AbilityTarget),
    ItemUsed(Item, AbilityTarget),
}

#[derive(Debug)]
pub enum CombatActionEffect {
    AbilityUsed(CombatantAbilityNames, Vec<u32>),
    ConsumableUsed(ConsumableTypes, Vec<u32>),
    CurrentHpChange(i16, u32),
    CurrentMpChange(i16, u32),
    StatusEffectGained(StatusEffects, u32),
    StatusEffectLost(StatusEffects, u32),
    CombatantDeath(u32),
    EndTurn,
}

impl AdventuringParty {
    pub fn get_combat_action_effects(
        &mut self,
        action: CombatAction,
        combatant_id: u32,
    ) -> Result<Vec<CombatActionEffect>, AppError> {
        match action {
            CombatAction::AbilityUsed(ability, targets) => {
                return self.get_ability_used_combat_action_effects(
                    combatant_id,
                    &ability,
                    &targets,
                );
            }
            CombatAction::ItemUsed(_, _) => todo!(),
        }
    }
}

impl RoguelikeRacerGame {
    pub fn get_ability_used_combat_action_effects(
        &self,
        ability_user_id: u32,
        ability: &CombatantAbility,
        ability_target: &AbilityTarget,
        ally_battle_group: BattleGroup,
        opponent_battle_group_option: Option<BattleGroup>,
    ) -> Result<Vec<CombatActionEffect>, AppError> {
        let mut effects = vec![];
        let ability_user_party = self
            .adventuring_parties
            .get(&ally_battle_group.party_id)
            .ok_or_else(|| AppError {
                error_type: AppErrorTypes::Generic,
                message: error_messages::PARTY_NOT_FOUND.to_string(),
            })?;

        let ability_user = ability_user_party.get_combatant_by_id(ability_user_id)?;
        match ability.ability_name {
            CombatantAbilityNames::Attack => match ability_target {
                AbilityTarget::Single(id) => {
                    let target_battle_group = BattleGroup::get_battle_group_entity_belongs_to(
                        ally_battle_group,
                        opponent_battle_group_option,
                        id,
                    )?;
                    let (targeted_entity_properties, targeted_combatant_properties) =
                        self.get_mut_combatant_in_battle_group_by_id(&target_battle_group, *id)?;
                    effects.push(CombatActionEffect::AbilityUsed(
                        CombatantAbilityNames::Attack,
                        vec![*id],
                    ));
                    effects.push(CombatActionEffect::CurrentHpChange(-10, *id));
                }
                _ => {
                    return Err(AppError {
                        error_type: AppErrorTypes::InvalidInput,
                        message: error_messages::INVALID_TARGETING_SCHEME.to_string(),
                    })
                }
            },
            CombatantAbilityNames::ArmorBreak => todo!(),
            CombatantAbilityNames::HeatLance => todo!(),
            CombatantAbilityNames::Fire => todo!(),
            CombatantAbilityNames::RainStorm => todo!(),
            CombatantAbilityNames::Heal => todo!(),
        }

        Ok(effects)
    }
}

// characters should be able to use abilities on their allies when not in combat
// ability single targets validatable based on target current hp (rez spell)
// target groups such as "friendly" or "hostile" should be easily resolvable to combatant ids
// regardless of whether the user is a player or ai
