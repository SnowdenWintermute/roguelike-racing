use crate::app_consts::error_messages::INVALID_EQUIPMENT_EQUIPPED;
use crate::combat::battle::Battle;
use crate::combat::combat_actions::CombatAction;
use crate::combat::combat_actions::CombatActionTarget;
use crate::combat::ActionResult;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
use crate::items::equipment::EquipmentProperties;
use crate::items::equipment::EquipmentSlots;
use crate::items::equipment::EquipmentTypes;

impl RoguelikeRacerGame {
    pub fn attack_handler(
        &self,
        action_user_id: u32,
        targets: &CombatActionTarget,
        battle_option: Option<&Battle>,
        ally_ids: Vec<u32>,
    ) -> Result<Vec<ActionResult>, AppError> {
        // ALL TARGET VALIDATION SHOULD BE DONE ALREADY WHEN SELECTING THE ABILITY
        let (_, combatant_properties) = self.get_combatant_by_id(&action_user_id)?;
        let combatant_level = combatant_properties.level;

        let mut action_results = vec![];
        // DETERMINE USER WEAPONS
        let (_, user_combatant_properties) = self.get_combatant_by_id(&action_user_id)?;
        let mh_equipment_option =
            user_combatant_properties.get_equipped_item(&EquipmentSlots::MainHand);
        let oh_equipment_option =
            user_combatant_properties.get_equipped_item(&EquipmentSlots::OffHand);
        // check for sheilds since they can't be used to attack
        let mh_weapon_option = EquipmentProperties::get_weapon_equipment_properties_option_from_equipment_properties_option(mh_equipment_option);
        let oh_weapon_option = EquipmentProperties::get_weapon_equipment_properties_option_from_equipment_properties_option(oh_equipment_option);

        let mut mh_attack_ends_turn = EquipmentProperties::is_shield(oh_equipment_option)
            || if let Some(mh_weapon_properties) = mh_weapon_option {
                mh_weapon_properties.is_two_handed()
            } else {
                false
            };

        // MAIN HAND RESULT
        let mh_attack_ability_name = if let Some(mh_weapon_properties) = mh_weapon_option {
            match &mh_weapon_properties.equipment_type {
                EquipmentTypes::TwoHandedRangedWeapon(..) => {
                    CombatantAbilityNames::AttackRangedMainhand
                }
                EquipmentTypes::TwoHandedMeleeWeapon(..) => {
                    CombatantAbilityNames::AttackMeleeMainhand
                }
                EquipmentTypes::OneHandedMeleeWeapon(..) => {
                    CombatantAbilityNames::AttackMeleeMainhand
                }
                _ => {
                    return Err(AppError {
                        error_type: crate::errors::AppErrorTypes::Generic,
                        message: INVALID_EQUIPMENT_EQUIPPED.to_string(),
                    })
                }
            }
        } else {
            CombatantAbilityNames::AttackMeleeMainhand
        };
        let mh_attack_action = CombatAction::AbilityUsed(mh_attack_ability_name.clone());
        let mh_attack_ability_attributes = mh_attack_ability_name.get_attributes();
        let mut mh_attack_result = self.calculate_action_hp_and_mp_changes(
            mh_attack_action,
            action_user_id,
            targets,
            battle_option,
            ally_ids.clone(),
            Some((
                combatant_level,
                mh_attack_ability_attributes.base_hp_change_values_level_multiplier,
            )),
        )?;

        // targets were killed, don't calc off hand swing
        let mut all_damaged_targets_died = true;
        if let Some(hp_changes) = &mh_attack_result.hp_changes_by_entity_id {
            for (combatant_id, hp_change) in hp_changes {
                if let Ok((_, combatant_properties)) = self.get_combatant_by_id(&combatant_id) {
                    if combatant_properties.hit_points as i16 + *hp_change > 0 {
                        all_damaged_targets_died = false;
                        break;
                    }
                }
            }
        }

        mh_attack_ends_turn = mh_attack_ends_turn | all_damaged_targets_died;
        mh_attack_result.ends_turn = mh_attack_ends_turn;
        action_results.push(mh_attack_result);

        // OFF HAND RESULT IF ANY
        if !mh_attack_ends_turn && !EquipmentProperties::is_shield(oh_equipment_option) {
            let oh_attack_ability_name = if let Some(oh_weapon_properties) = oh_weapon_option {
                match &oh_weapon_properties.equipment_type {
                    EquipmentTypes::OneHandedMeleeWeapon(..) => {
                        CombatantAbilityNames::AttackMeleeOffhand
                    }
                    _ => {
                        return Err(AppError {
                            error_type: crate::errors::AppErrorTypes::Generic,
                            message: INVALID_EQUIPMENT_EQUIPPED.to_string(),
                        })
                    }
                }
            } else {
                CombatantAbilityNames::AttackMeleeOffhand
            };
            let oh_attack_action = CombatAction::AbilityUsed(oh_attack_ability_name.clone());
            let oh_attack_ability_attributes = oh_attack_ability_name.get_attributes();
            let oh_attack_result = self.calculate_action_hp_and_mp_changes(
                oh_attack_action,
                action_user_id,
                targets,
                battle_option,
                ally_ids,
                Some((
                    combatant_level,
                    oh_attack_ability_attributes.base_hp_change_values_level_multiplier,
                )),
            )?;

            action_results.push(oh_attack_result);
        }
        Ok(action_results)
    }
}
