use crate::combat::ability_handlers::ability_resolution_calculators::calculate_weapon_swing_result::calculate_weapon_swing_result;
use crate::combat::battle::Battle;
use crate::app_consts::error_messages;
use crate::combat::ActionResult;
use crate::combat::combat_actions::{CombatActionTarget, TargetingScheme};
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
use crate::items::equipment::EquipmentSlots;
use crate::items::equipment::unarmed::FIST;

impl RoguelikeRacerGame {
    pub fn attack_handler(
        &self,
        ability_user_id: u32,
        ability_target: &CombatActionTarget,
        battle_option: Option<&Battle>,
    ) -> Result<Vec<ActionResult>, AppError> {
        let mut action_results = vec![];
        let battle = battle_option.ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::Generic,
            message: error_messages::INVALID_USABILITY_CONTEXT.to_string(),
        })?;

        let (ally_ids, opponent_ids_option) =
            battle.get_ally_ids_and_opponent_ids_option(ability_user_id)?;

        let target_entity_id = ability_target.get_targets_if_scheme_valid(
            ally_ids,
            opponent_ids_option,
            vec![TargetingScheme::All, TargetingScheme::Area],
        )?[0];

        let (_, target_combatant_properties) =
            self.get_combatant_in_battle_by_id(&battle, &target_entity_id)?;
        let target_combatant_properties = target_combatant_properties.clone();
        let (_, user_combatant_properties) =
            self.get_combatant_in_battle_by_id(&battle, &ability_user_id)?;
        let user_total_attributes = user_combatant_properties.get_total_attributes();
        let target_total_attributes = target_combatant_properties.get_total_attributes();

        let mh_equipment_option =
            user_combatant_properties.get_equipped_item(&EquipmentSlots::MainHand);
        let oh_equipment_option =
            user_combatant_properties.get_equipped_item(&EquipmentSlots::OffHand);
        // check for sheilds since they can't be used to attack
        let mh_weapon_option = match mh_equipment_option {
            Some(equipment_properties) => match equipment_properties.is_weapon() {
                true => Some(equipment_properties),
                false => None,
            },
            None => None,
        };
        let oh_weapon_option = match oh_equipment_option {
            Some(equipment_properties) => match equipment_properties.is_weapon() {
                true => Some(equipment_properties),
                false => None,
            },
            None => None,
        };

        // WEILDING WEAPON(S)
        if mh_weapon_option.is_some() || oh_weapon_option.is_some() {
            let mh_swing_ends_turn = oh_weapon_option.is_none();
            if let Some(mh_weapon_properties) = mh_weapon_option {
                action_results.push(calculate_weapon_swing_result(
                    ability_user_id,
                    ability_target,
                    target_entity_id,
                    &user_total_attributes,
                    &target_total_attributes,
                    mh_weapon_properties,
                    false,
                    mh_swing_ends_turn,
                )?);
            };
            if let Some(oh_weapon_properties) = oh_weapon_option {
                action_results.push(calculate_weapon_swing_result(
                    ability_user_id,
                    ability_target,
                    target_entity_id,
                    &user_total_attributes,
                    &target_total_attributes,
                    oh_weapon_properties,
                    true,
                    true,
                )?);
            };
        } else {
            // UNARMED
            let mh_swing_ends_turn = oh_equipment_option.is_some();
            if mh_equipment_option.is_none() {
                action_results.push(calculate_weapon_swing_result(
                    ability_user_id,
                    ability_target,
                    target_entity_id,
                    &user_total_attributes,
                    &target_total_attributes,
                    &FIST,
                    false,
                    mh_swing_ends_turn,
                )?);
            }
            if !mh_swing_ends_turn {
                println!("calculating oh unarmed swing: ");
                if oh_equipment_option.is_none() {
                    action_results.push(calculate_weapon_swing_result(
                        ability_user_id,
                        ability_target,
                        target_entity_id,
                        &user_total_attributes,
                        &target_total_attributes,
                        &FIST,
                        true,
                        true,
                    )?);
                }
            }
        }

        Ok(action_results)
    }
}
