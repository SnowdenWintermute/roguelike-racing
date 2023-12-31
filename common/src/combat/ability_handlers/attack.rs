use crate::{
    app_consts::error_messages,
    combat::{battle::Battle, CombatActionResult},
    combatants::abilities::{AbilityTarget, CombatantAbilityNames},
    errors::AppError,
    game::RoguelikeRacerGame,
    items::equipment::EquipmentSlots,
};

impl RoguelikeRacerGame {
    pub fn attack_handler(
        &mut self,
        ability_user_id: u32,
        ability_name: &CombatantAbilityNames,
        ability_target: &AbilityTarget,
        battle_option: Option<&Battle>,
    ) -> Result<Vec<CombatActionResult>, AppError> {
        // get the battle
        let battle = battle_option.ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::Generic,
            message: error_messages::INVALID_ABILITY_CONTEXT.to_string(),
        })?;

        let target_entity_id = match ability_target {
            AbilityTarget::Single(id) => id,
            _ => {
                return Err(AppError {
                    error_type: crate::errors::AppErrorTypes::InvalidInput,
                    message: error_messages::INVALID_TARGETS_SELECTED.to_string(),
                })
            }
        };

        // get their targeted entity
        let (_, target_combatant_properties) =
            self.get_mut_combatant_by_id(&battle, target_entity_id)?;
        // get ability user entity
        let (_, user_combatant_properties) =
            self.get_mut_combatant_by_id(&battle, &ability_user_id)?;
        let user_total_attributes = user_combatant_properties.get_total_attributes();

        let mh_weapon_option =
            user_combatant_properties.get_equipped_weapon_properties(&EquipmentSlots::MainHand);
        let oh_weapon_option =
            user_combatant_properties.get_equipped_weapon_properties(&EquipmentSlots::OffHand);

        // WIELDING 2H WEAPON
        if let Some((mh_weapon_properties, _)) = mh_weapon_option {
            // match mh_weapon_properties.eq
        };
        // WIELDING 1h MainHand
        // WIELDING 1h OffHand
        // UNARMED

        // for each weapon create a result, if no weapon use base damage
        // give double base bonus damage to 2h weapons
        //
        // check accuracy vs evasion to determine if attack is a hit or a miss
        // if hit, get the incomming damage before defense
        // roll the incomming damage to see if it is a crit
        // take the final incomming damage vs the target's defensive stats
        // calculate damage
        // return result
        todo!();
    }
}
