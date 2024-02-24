pub mod abilities;
pub mod award_levelups;
pub mod combat_attributes;
pub mod combatant_classes;
pub mod combatant_traits;
mod equip_item;
mod get_total_elemental_affinites;
mod get_total_physical_damage_type_affinities;
use self::combatant_classes::CombatantClass;
use self::combatant_traits::CombatantTraits;
use crate::combat::combat_actions::CombatAction;
mod get_equipped_item;
mod get_equipped_weapon_properties;
mod get_total_attributes;
pub mod inventory;
mod unequip_item;
use crate::combat::combat_actions::CombatActionTarget;
use crate::combat::magical_elements::MagicalElements;
use crate::items::equipment::EquipmentProperties;
use crate::items::equipment::EquipmentTypes;
mod set_new_ability_target_preferences;
use self::abilities::CombatantAbility;
use self::abilities::CombatantAbilityNames;
use self::combat_attributes::CombatAttributes;
use self::inventory::Inventory;
use crate::app_consts::error_messages;
use crate::errors::AppError;
use crate::items::equipment::EquipmentSlots;
use crate::items::Item;
use crate::status_effects::StatusEffects;
use crate::utils::add_i16_to_u16_and_clamp_to_max;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CombatantControlledBy {
    AI,
    Player(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExperiencePoints {
    pub current: u16,
    pub required_for_next_level: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CombatantProperties {
    pub combatant_class: CombatantClass,
    pub inherent_attributes: HashMap<CombatAttributes, u16>,
    pub specced_attributes: HashMap<CombatAttributes, u16>,
    pub level: u8,
    pub experience_points: ExperiencePoints,
    pub unspent_attribute_points: u8,
    pub unspent_ability_points: u8,
    pub hit_points: u16,
    pub mana: u16,
    pub status_effects: Vec<StatusEffects>,
    pub equipment: HashMap<EquipmentSlots, Item>,
    pub inventory: Inventory,
    pub abilities: HashMap<CombatantAbilityNames, CombatantAbility>,
    pub traits: Vec<CombatantTraits>,
    pub inherent_elemental_affinities: HashMap<MagicalElements, i16>,
    pub selected_combat_action: Option<CombatAction>,
    pub combat_action_targets: Option<CombatActionTarget>,
    pub controlled_by: CombatantControlledBy,
}

impl CombatantProperties {
    pub fn new(
        combatant_class: &CombatantClass,
        abilities: HashMap<CombatantAbilityNames, CombatantAbility>,
        controlled_by: CombatantControlledBy,
    ) -> CombatantProperties {
        CombatantProperties {
            combatant_class: combatant_class.clone(),
            inherent_attributes: HashMap::new(),
            specced_attributes: HashMap::new(),
            level: 1,
            hit_points: 1,
            experience_points: ExperiencePoints {
                current: 0,
                required_for_next_level: Some(100),
            },
            unspent_attribute_points: 0,
            unspent_ability_points: 0,
            mana: 0,
            status_effects: vec![],
            equipment: HashMap::new(),
            inventory: Inventory::new(),
            abilities,
            traits: Vec::new(),
            inherent_elemental_affinities: HashMap::new(),
            selected_combat_action: None,
            combat_action_targets: None,
            controlled_by,
        }
    }

    pub fn set_hp_and_mp_to_max(&mut self) {
        let total_attributes = self.get_total_attributes();
        let max_hp_option = total_attributes.get(&CombatAttributes::Hp);
        if let Some(max_hp) = max_hp_option {
            self.hit_points = *max_hp;
        }
        let max_mana_option = total_attributes.get(&CombatAttributes::Mp);
        if let Some(max_mana) = max_mana_option {
            self.mana = *max_mana
        }
    }

    pub fn can_use_item(&self, item: &Item) -> bool {
        let total_character_attributes = self.get_total_attributes();
        item.requirements_satisfied_by_attributes(&total_character_attributes)
    }

    pub fn clamp_curr_hp_to_max(&mut self) {
        // @TODO optimize to only add up HP
        let total_attributes = self.get_total_attributes();
        match total_attributes.get(&CombatAttributes::Hp) {
            Some(max_hp) => {
                if max_hp < &self.hit_points {
                    self.hit_points = *max_hp
                }
            }
            None => (),
        }
    }
    pub fn clamp_curr_mp_to_max(&mut self) {
        // @TODO optimize to only add up HP
        let total_attributes = self.get_total_attributes();
        match total_attributes.get(&CombatAttributes::Mp) {
            Some(max_mp) => {
                if max_mp < &self.mana {
                    self.mana = *max_mp
                }
            }
            None => (),
        }
    }

    pub fn get_mut_ability_if_owned<'a>(
        &'a mut self,
        ability_name: &CombatantAbilityNames,
    ) -> Result<&'a mut CombatantAbility, AppError> {
        self.abilities
            .get_mut(ability_name)
            .ok_or_else(|| AppError {
                error_type: crate::errors::AppErrorTypes::InvalidInput,
                message: error_messages::ABILITY_NOT_OWNED.to_string(),
            })
    }

    pub fn get_ability_if_owned<'a>(
        &'a self,
        ability_name: &CombatantAbilityNames,
    ) -> Result<&'a CombatantAbility, AppError> {
        self.abilities.get(ability_name).ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::InvalidInput,
            message: error_messages::ABILITY_NOT_OWNED.to_string(),
        })
    }

    pub fn get_ability_cost_if_owned<'a>(
        &'a self,
        ability_name: &CombatantAbilityNames,
    ) -> Result<u8, AppError> {
        let ability = self.abilities.get(ability_name).ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::InvalidInput,
            message: error_messages::ABILITY_NOT_OWNED.to_string(),
        })?;
        let ability_attributes = ability_name.get_attributes();
        let base_mp_cost = ability_attributes.mana_cost;
        let mana_cost_level_multiplier = ability_attributes.mana_cost_level_multiplier;
        let level_adjusted_mp_cost = ability.level * (base_mp_cost * mana_cost_level_multiplier);
        Ok(level_adjusted_mp_cost)
    }

    pub fn change_hp(&mut self, hp_change: i16) -> u16 {
        let combatant_total_attributes = self.get_total_attributes();
        let max_hp = combatant_total_attributes
            .get(&CombatAttributes::Hp)
            .unwrap_or_else(|| &0);
        let new_hp = add_i16_to_u16_and_clamp_to_max(self.hit_points, hp_change, *max_hp);
        self.hit_points = new_hp;
        new_hp
    }

    pub fn change_mp(&mut self, mp_change: i16) -> u16 {
        let combatant_total_attributes = self.get_total_attributes();
        let max_mp = combatant_total_attributes
            .get(&CombatAttributes::Mp)
            .unwrap_or_else(|| &0);
        let new_mp = add_i16_to_u16_and_clamp_to_max(self.mana, mp_change, *max_mp);
        self.mana = new_mp;
        new_mp
    }

    pub fn get_weapon_in_slot<'a>(
        &'a self,
        slot: &EquipmentSlots,
    ) -> Option<&'a EquipmentProperties> {
        if let Some(equipment_properties) = self.get_equipped_item(&slot) {
            match &equipment_properties.equipment_type {
                EquipmentTypes::OneHandedMeleeWeapon(_, _)
                | EquipmentTypes::TwoHandedMeleeWeapon(_, _)
                | EquipmentTypes::TwoHandedRangedWeapon(_, _) => Some(&equipment_properties),
                _ => return None,
            }
        } else {
            None
        }
    }
}
