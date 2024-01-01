use crate::{
    app_consts::{error_messages, TWO_HANDED_WEAPON_BASE_BONUS_DAMAGE_MODIFIER},
    combat::{battle::Battle, CombatActionResult},
    combatants::{
        abilities::{AbilityTarget, CombatantAbilityNames},
        combat_attributes::CombatAttributes,
        get_base_ability_damage_bonus::DamageSource,
        CombatantProperties,
    },
    errors::AppError,
    game::RoguelikeRacerGame,
    items::equipment::{EquipmentSlots, EquipmentTypes},
};
use rand::Rng;
use std::cmp;

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
        let target_total_attributes = target_combatant_properties.get_total_attributes();

        let mh_weapon_option =
            user_combatant_properties.get_equipped_item(&EquipmentSlots::MainHand);
        let oh_weapon_option =
            user_combatant_properties.get_equipped_item(&EquipmentSlots::OffHand);

        if let Some(mh_weapon_properties) = mh_weapon_option {
            let (base_bonus_damage, weapon_properties) = match mh_weapon_properties.equipment_type {
                EquipmentTypes::OneHandedMeleeWeapon(_, properties) => (
                    CombatantProperties::get_base_ability_damage_bonus(
                        &user_total_attributes,
                        DamageSource::Melee,
                    ),
                    properties,
                ),
                EquipmentTypes::TwoHandedMeleeWeapon(_, properties) => (
                    CombatantProperties::get_base_ability_damage_bonus(
                        &user_total_attributes,
                        DamageSource::Melee,
                    ) * TWO_HANDED_WEAPON_BASE_BONUS_DAMAGE_MODIFIER,
                    properties,
                ),
                EquipmentTypes::TwoHandedRangedWeapon(_, properties) => (
                    CombatantProperties::get_base_ability_damage_bonus(
                        &user_total_attributes,
                        DamageSource::Ranged,
                    ) * TWO_HANDED_WEAPON_BASE_BONUS_DAMAGE_MODIFIER,
                    properties,
                ),
                _ => {
                    return Err(AppError {
                        error_type: crate::errors::AppErrorTypes::Generic,
                        message: error_messages::INVALID_EQUIPMENT_EQUIPPEND.to_string(),
                    })
                }
            };

            // determine hit or miss
            let accuracy = user_total_attributes
                .get(&CombatAttributes::Accuracy)
                .unwrap_or_else(|| &0);
            let target_evasion = target_total_attributes
                .get(&CombatAttributes::Evasion)
                .unwrap_or_else(|| &0);
            let acc_eva_compared = *accuracy as i16 - *target_evasion as i16;
            let chance_to_hit = if acc_eva_compared < 5 {
                5
            } else {
                cmp::min(95, acc_eva_compared)
            };

            let mut rng = rand::thread_rng();
            let evaded = rng.gen_range(1..=100) > chance_to_hit;

            let (weapon_damage_before_defensive_mods, hit_chance_before_defensive_mods) =
                CombatantProperties::get_weapon_damage_and_hit_chance(
                    &weapon_properties,
                    &mh_weapon_properties.traits,
                    base_bonus_damage,
                    *accuracy,
                    false,
                );
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
