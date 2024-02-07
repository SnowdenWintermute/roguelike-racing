pub mod abilities;
pub mod combat_attributes;
pub mod combatant_traits;
mod equip_item;
pub mod get_base_ability_damage_bonus;
mod get_total_elemental_affinites;
use self::combatant_traits::CombatantTraits;
mod get_equipped_item;
mod get_equipped_weapon_properties;
mod get_total_attributes;
mod get_weapon_damage_and_hit_chance;
pub mod inventory;
mod unequip_item;
use crate::combat::combat_actions::CombatActionTarget;
use crate::combat::combat_actions::FriendOrFoe;
use crate::combat::combat_actions::TargetingScheme;
use crate::combat::magical_elements::MagicalElements;
use crate::items::equipment::unarmed::FIST;
use crate::items::equipment::weapon_properties::WeaponProperties;
use crate::items::equipment::EquipmentProperties;
use crate::items::equipment::EquipmentTypes;
pub mod get_weapon_properties_traits_and_base_bonus_damage;
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
use core::fmt;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CombatantClass {
    Warrior,
    Mage,
    Rogue,
    None,
}

impl fmt::Display for CombatantClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CombatantClass::Warrior => write!(f, "Warrior"),
            CombatantClass::Mage => write!(f, "Mage"),
            CombatantClass::Rogue => write!(f, "Rogue"),
            CombatantClass::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CombatActionTargetPreferences {
    pub friendly_single: Option<u32>,
    pub hostile_single: Option<u32>,
    pub category_of_last_target: Option<FriendOrFoe>,
    pub category_of_last_area: Option<FriendOrFoe>,
    pub targeting_scheme_preference: TargetingScheme,
}

impl Default for CombatActionTargetPreferences {
    fn default() -> Self {
        CombatActionTargetPreferences {
            friendly_single: None,
            hostile_single: None,
            category_of_last_target: None,
            category_of_last_area: None,
            targeting_scheme_preference: TargetingScheme::Single,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CombatantControlledBy {
    AI,
    Player(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CombatantProperties {
    pub combatant_class: CombatantClass,
    pub inherent_attributes: HashMap<CombatAttributes, u16>,
    pub hit_points: u16,
    pub mana: u16,
    pub status_effects: Vec<StatusEffects>,
    pub equipment: HashMap<EquipmentSlots, Item>,
    pub inventory: Inventory,
    pub abilities: HashMap<CombatantAbilityNames, CombatantAbility>,
    pub traits: Vec<CombatantTraits>,
    pub inherent_elemental_affinities: HashMap<MagicalElements, i16>,
    pub selected_consumable: Option<u32>,
    pub selected_ability_name: Option<CombatantAbilityNames>,
    pub combat_action_targets: Option<CombatActionTarget>,
    pub combat_action_target_preferences: CombatActionTargetPreferences,
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
            hit_points: 0,
            mana: 0,
            status_effects: vec![],
            equipment: HashMap::new(),
            inventory: Inventory::new(),
            abilities,
            traits: Vec::new(),
            inherent_elemental_affinities: HashMap::new(),
            selected_consumable: None,
            selected_ability_name: None,
            combat_action_targets: None,
            combat_action_target_preferences: CombatActionTargetPreferences::default(),
            controlled_by,
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
            .get(&CombatAttributes::Hp)
            .unwrap_or_else(|| &0);
        let new_mp = add_i16_to_u16_and_clamp_to_max(self.mana, mp_change, *max_mp);
        self.mana = new_mp;
        new_mp
    }

    pub fn get_weapon_in_slot<'a>(
        &'a self,
        slot: EquipmentSlots,
    ) -> Option<&'a EquipmentProperties> {
        if let Some(equipment_properties) = self.get_equipped_item(&slot) {
            match &equipment_properties.equipment_type {
                EquipmentTypes::OneHandedMeleeWeapon(_, _)
                | EquipmentTypes::TwoHandedMeleeWeapon(_, _)
                | EquipmentTypes::TwoHandedRangedWeapon(_, _) => Some(&equipment_properties),
                _ => return None,
            }
        } else {
            match slot {
                EquipmentSlots::MainHand | EquipmentSlots::OffHand => match FIST.equipment_type {
                    EquipmentTypes::OneHandedMeleeWeapon(_, _) => Some(&FIST),
                    _ => None,
                },
                _ => None,
            }
        }
    }
}
