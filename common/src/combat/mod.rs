use crate::character::abilities::{CharacterAbilities, CharacterAbility};
use crate::errors::{AppError, AppErrorTypes};
use crate::status_effects::StatusEffects;
use std::result::Result::Err;
use std::vec;

use crate::character::Character;
use crate::items::{Item, ItemProperties};

pub enum CombatAction {
    UseCharacterAbility(CharacterAbility),
    UseItem(Item),
}

pub enum CombatActionEffect {
    CurrentHpChange(i16),
    CurrentMpChange(i16),
    StatusEffectGained(StatusEffects),
    StatusEffectLost(StatusEffects),
    EndTurn,
}

pub struct CombatEvent {
    pub target_id: u32,
    pub action_effects: Vec<CombatActionEffect>,
}

impl Character {
    pub fn perform_combat_action(
        &self,
        combat_action: CombatAction,
    ) -> Result<Vec<CombatEvent>, AppError> {
        if self.current_room.monster.is_none() {
            return Err(AppError {
                error_type: AppErrorTypes::InvalidInput,
                message: "Can't perform a combat action without an opponent".to_string(),
            });
        }

        let mut combat_events: Vec<CombatEvent> = vec![];

        match combat_action {
            CombatAction::UseItem(item) => match item.item_properties {
                ItemProperties::Consumable(properties) => {
                    if properties.uses_remaining < 1 {
                        return Err(AppError {
                            error_type: AppErrorTypes::InsufficientResources,
                            message: "Can't use an item with no uses remaining".to_string(),
                        });
                    } else {
                        return Ok(vec![]);
                    }
                }
                ItemProperties::Equipment(_properties) => {
                    return Err(AppError {
                        error_type: AppErrorTypes::InvalidInput,
                        message: "Can't use an equipment item, must use consumable".to_string(),
                    });
                }
            },
            CombatAction::UseCharacterAbility(ability) => {
                if ability.mana_cost as u16 > self.mana.current {
                    return Err(AppError {
                        error_type: AppErrorTypes::InvalidInput,
                        message: "Not enough mana to use ability".to_string(),
                    });
                } else {
                    match ability.ability_type {
                        CharacterAbilities::Attack => {
                            let target_id = self
                                .current_room
                                .monster
                                .as_ref()
                                .expect(
                                    "monster should exist because we return early if it doesn't",
                                )
                                .entity_properties
                                .id;

                            let damage = 10;
                            let action_effect = CombatActionEffect::CurrentHpChange(damage * -1);
                            combat_events.push(CombatEvent {
                                target_id,
                                action_effects: vec![action_effect],
                            });
                        }
                        _ => {
                            println!("some unhandled action taken")
                        }
                    }

                    return Ok(vec![]);
                }
            }
        }
    }
}
