use crate::app_consts::error_messages;
use crate::combat::combat_actions::CombatActionHpChangeProperties;
use crate::combat::magical_elements::MagicalElements;
use crate::combatants::combatant_traits::CombatantTraits;
use crate::combatants::CombatantProperties;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
use crate::items::equipment::EquipmentSlots;
use crate::primatives::WeaponSlot;

impl RoguelikeRacerGame {
    pub fn add_element_from_weapon_to_hp_change_properties(
        &self,
        weapon_slot: &WeaponSlot,
        user_combatant_properties: &CombatantProperties,
        target_entity_ids: &Vec<u32>,
        hp_change_properties: &mut CombatActionHpChangeProperties,
    ) -> Result<(), AppError> {
        let equipment_option = match weapon_slot {
            WeaponSlot::MainHand => {
                user_combatant_properties.get_weapon_in_slot(&EquipmentSlots::MainHand)
            }
            WeaponSlot::OffHand => {
                user_combatant_properties.get_weapon_in_slot(&EquipmentSlots::OffHand)
            }
        };
        Ok(if let Some(equipment_properties) = equipment_option {
            let weapon_properties = equipment_properties.get_equipment_weapon_properties()?;
            let mut elements_to_select_from = vec![];
            for hp_change_source in &weapon_properties.damage_classifications {
                match &hp_change_source.element {
                    Some(element) => elements_to_select_from.push(element.clone()),
                    None => (),
                }
            }
            // CHECK ELEMENTAL WEAKNESSES AGAINST ONLY 1 TARGET ID
            let first_target_id = target_entity_ids.first().ok_or_else(|| AppError {
                error_type: crate::errors::AppErrorTypes::Generic,
                message: error_messages::NO_VALID_TARGETS_FOUND.to_string(),
            })?;
            let (_, target_combatant_properties) = self.get_combatant_by_id(&first_target_id)?;
            let mut weakest_affinity_with_weapon_elemental_options: Option<(MagicalElements, i16)> =
                None;
            if elements_to_select_from.len() > 0 {
                for combatant_trait in &target_combatant_properties.traits {
                    match combatant_trait {
                        CombatantTraits::ElementalAffinityPercent(element, affinity_percentage) => {
                            if elements_to_select_from.contains(&element) {
                                if let Some((_, weakest_percentage)) =
                                    weakest_affinity_with_weapon_elemental_options
                                {
                                    if *affinity_percentage < weakest_percentage {
                                        weakest_affinity_with_weapon_elemental_options =
                                            Some((element.clone(), *affinity_percentage))
                                    }
                                } else {
                                    weakest_affinity_with_weapon_elemental_options =
                                        Some((element.clone(), *affinity_percentage))
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
            if let Some((element, _)) = weakest_affinity_with_weapon_elemental_options {
                hp_change_properties.source_properties.element = Some(element);
            }
        })
    }
}
