use super::combat_log_message::CombatLogMessage;
use super::combat_log_message::CombatLogMessageStyle;
use common::combat::combat_actions::CombatAction;
use common::combat::ActionResult;
use common::combatants::abilities::CombatantAbilityNames;
use common::errors::AppError;
use common::game::RoguelikeRacerGame;
use common::items::consumables::ConsumableTypes;
use js_sys::Date;
use std::collections::HashSet;
use yew::AttrValue;

pub fn create_logs_from_action_result(
    game: &RoguelikeRacerGame,
    action_result: &ActionResult,
) -> Result<Vec<CombatLogMessage>, AppError> {
    let mut messages = Vec::new();
    let (action_user_entity, action_user_combatant) =
        game.get_combatant_by_id(&action_result.user_id)?;
    let action_user_name = action_user_entity.name.clone();
    let timestamp = Date::new_0().get_time() as u64;

    if let Some(misses) = &action_result.misses_by_entity_id {
        for target_id in misses {
            let (target_entity, _) = game.get_combatant_by_id(&target_id)?;
            let message =
                AttrValue::from(format!("{action_user_name} misses {}", target_entity.name));
            messages.push(CombatLogMessage {
                message,
                style: CombatLogMessageStyle::Basic,
                timestamp,
            });
        }
    }

    match &action_result.action {
        CombatAction::AbilityUsed(ability_name) => match ability_name {
            CombatantAbilityNames::Attack => (),
            CombatantAbilityNames::AttackMeleeMainhand
            | CombatantAbilityNames::AttackMeleeOffhand
            | CombatantAbilityNames::AttackRangedMainhand => {
                if let Some(hp_changes) = &action_result.hp_changes_by_entity_id {
                    for (target_id, hp_change) in hp_changes {
                        let message = format_attack_hp_change_message(
                            game,
                            &action_user_name,
                            target_id,
                            hp_change,
                            &action_result.crits_by_entity_id,
                        )?;
                        messages.push(CombatLogMessage {
                            message,
                            style: CombatLogMessageStyle::Basic,
                            timestamp,
                        });
                    }
                }
            }
            CombatantAbilityNames::Fire
            | CombatantAbilityNames::Ice
            | CombatantAbilityNames::Healing => {
                let message = AttrValue::from(format!("{action_user_name} casts {ability_name}"));
                messages.push(CombatLogMessage {
                    message,
                    style: CombatLogMessageStyle::Basic,
                    timestamp,
                });

                if let Some(hp_changes) = &action_result.hp_changes_by_entity_id {
                    for (target_id, hp_change) in hp_changes {
                        let message = format_ability_hp_change_message(
                            game,
                            target_id,
                            hp_change,
                            &action_result.crits_by_entity_id,
                        )?;
                        messages.push(CombatLogMessage {
                            message,
                            style: CombatLogMessageStyle::Basic,
                            timestamp,
                        });
                    }
                }
            }
        },
        CombatAction::ConsumableUsed(item_id) => {
            let consumable = action_user_combatant.inventory.get_consumable(item_id)?;
            match consumable.consumable_type {
                ConsumableTypes::HpAutoinjector => {
                    if let Some(hp_changes) = &action_result.hp_changes_by_entity_id {
                        for (target_id, hp_change) in hp_changes {
                            let target_text = if target_id == &action_user_entity.id {
                                "themselves".to_string()
                            } else {
                                let (target_entity, _) = game.get_combatant_by_id(&target_id)?;
                                target_entity.name.clone()
                            };
                            let message = AttrValue::from(format!(
                                "{action_user_name} injects {target_text}, healing {} HP",
                                hp_change
                            ));
                            messages.push(CombatLogMessage {
                                message,
                                style: CombatLogMessageStyle::Basic,
                                timestamp,
                            });
                        }
                    }
                }
                ConsumableTypes::MpAutoinjector => {
                    if let Some(mp_changes) = &action_result.mp_changes_by_entity_id {
                        for (target_id, mp_change) in mp_changes {
                            let target_text = if target_id == &action_user_entity.id {
                                "themselves".to_string()
                            } else {
                                let (target_entity, _) = game.get_combatant_by_id(&target_id)?;
                                target_entity.name.clone()
                            };
                            let message = AttrValue::from(format!(
                                "{action_user_name} injects {target_text}, restoring {} MP",
                                mp_change
                            ));
                            messages.push(CombatLogMessage {
                                message,
                                style: CombatLogMessageStyle::Basic,
                                timestamp,
                            });
                        }
                    }
                }
                _ => (),
            }
        }
    }

    Ok(messages)
}

fn format_attack_hp_change_message(
    game: &RoguelikeRacerGame,
    action_user_name: &String,
    target_id: &u32,
    hp_change: &i16,
    crits_by_entity_id: &Option<HashSet<u32>>,
) -> Result<AttrValue, AppError> {
    let (target_entity, _) = game.get_combatant_by_id(&target_id)?;
    let (healed_or_damaged, points_of_damage_or_hp) = if hp_change > &0 {
        ("heals", "HP")
    } else {
        ("hits", "points of damage")
    };

    let crit_text = if let Some(crits) = crits_by_entity_id {
        if crits.contains(&target_entity.id) {
            "scores a critical hit and "
        } else {
            ""
        }
    } else {
        ""
    };

    Ok(AttrValue::from(format!(
        "{action_user_name} {crit_text}{healed_or_damaged} {} for {} {points_of_damage_or_hp}",
        target_entity.name,
        hp_change.abs()
    )))
}

fn format_ability_hp_change_message(
    game: &RoguelikeRacerGame,
    target_id: &u32,
    hp_change: &i16,
    crits_by_entity_id: &Option<HashSet<u32>>,
) -> Result<AttrValue, AppError> {
    let (target_entity, _) = game.get_combatant_by_id(&target_id)?;

    let crit_text = if let Some(crits) = crits_by_entity_id {
        if crits.contains(&target_entity.id) {
            "Critical strike! "
        } else {
            ""
        }
    } else {
        ""
    };
    let (healed_or_damaged, points_of_damage_or_hp) = if hp_change > &0 {
        ("is healed for", "HP")
    } else {
        ("takes", "points of damage")
    };

    Ok(AttrValue::from(format!(
        "{crit_text}{} {healed_or_damaged} {} {points_of_damage_or_hp}",
        target_entity.name,
        hp_change.abs()
    )))
}
