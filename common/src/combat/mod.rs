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
use std::collections::HashMap;

// combat takes place between groups of combatants
// combatants' physical positions in battle can be expressed by their entity ids in combatant
// position vectors, or "groups" which have a group name
// in order to facilitate pvp, combatant groups are stored in "combat arenas"
// adventuring parties store a ref to a combat arena when they enter combat
// groups hold refs to the adventuring party they were generated from
// groups of player character combatants are named after their party name
// groups of monster combatants are named after the adventuring party

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Battle {
    pub id: u32,
    pub group_a: BattleGroup,
    pub group_b: BattleGroup,
    pub combatant_turn_trackers: Option<Vec<CombatantTurnTracker>>,
}

impl Battle {
    pub fn combatant_is_first_in_turn_order(&self, entity_id: u32) -> bool {
        match &self.combatant_turn_trackers {
            Some(trackers) => match trackers.get(0) {
                Some(combat_turn_tracker) => combat_turn_tracker.entity_id == entity_id,
                None => false,
            },
            None => false,
        }
    }
}

impl RoguelikeRacerGame {
    pub fn initiate_battle(
        &mut self,
        group_a: BattleGroup,
        group_b: BattleGroup,
    ) -> Result<(), AppError> {
        let mut battle = Battle {
            id: self.id_generator.get_next_entity_id(),
            group_a,
            group_b,
            combatant_turn_trackers: None,
        };

        let turn_trackers = self.get_combat_turn_order(&battle)?;
        battle.combatant_turn_trackers = Some(turn_trackers);
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

    pub fn get_ability_used_combat_action_effects(
        &mut self,
        combatant_id: u32,
        ability: &CombatantAbility,
        _: &AbilityTarget,
    ) -> Result<Vec<CombatActionEffect>, AppError> {
        let effects = vec![];
        let _ = self.get_combatant_by_id(combatant_id)?;
        // get their arena
        // get their group
        // the group that isn't theirs is the "hostile" group
        match ability.ability_name {
            CombatantAbilityNames::Attack => {
                //
            }
            CombatantAbilityNames::ArmorBreak => todo!(),
            CombatantAbilityNames::HeatLance => todo!(),
            CombatantAbilityNames::Fire => todo!(),
            CombatantAbilityNames::RainStorm => todo!(),
            CombatantAbilityNames::Heal => todo!(),
        }

        Ok(effects)
    }
}
