use self::abilities::CombatantAbility;
use self::abilities::CombatantAbilityNames;
use crate::app_consts::DEX_TO_ACCURACY_RATIO;
use crate::app_consts::OFF_HAND_ACCURACY_MODIFIER;
use crate::app_consts::OFF_HAND_DAMAGE_MODIFIER;
use crate::items::equipment::trait_effects::get_weapon_percent_damage_increase_trait_damage_modifier::get_weapon_percent_damage_increase_trait_damage_modifier;
use crate::items::equipment::weapon_properties::WeaponProperties;
use crate::items::equipment::EquipmentSlots;
use crate::items::equipment::EquipmentTraits;
use crate::items::equipment::EquipmentTypes;
use crate::items::Item;
use crate::items::ItemProperties;
use crate::primatives::Range;
use crate::status_effects::StatusEffects;
use core::fmt;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
pub mod abilities;
mod get_default_target_ids;
mod last_targets_are_still_valid;

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

#[derive(
    Debug, EnumIter, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize, PartialOrd, Ord,
)]
pub enum CombatAttributes {
    Damage,
    ArmorPenetration,
    Accuracy,
    Focus,
    ArmorClass,
    Evasion,
    Obscurity,
    Hp,
    Mp,
    Dexterity,
    Intelligence,
    Strength,
    Vitality,
    Resilience,
}

pub const CORE_ATTRIBUTES: [CombatAttributes; 5] = [
    CombatAttributes::Dexterity,
    CombatAttributes::Intelligence,
    CombatAttributes::Strength,
    CombatAttributes::Vitality,
    CombatAttributes::Resilience,
];

impl fmt::Display for CombatAttributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CombatAttributes::Damage => write!(f, "Damage"),
            CombatAttributes::ArmorClass => write!(f, "Armor Class"),
            CombatAttributes::Dexterity => write!(f, "Dexterity"),
            CombatAttributes::Strength => write!(f, "Strength"),
            CombatAttributes::Intelligence => write!(f, "Intelligence"),
            CombatAttributes::Vitality => write!(f, "Vitality"),
            CombatAttributes::Resilience => write!(f, "Resilience"),
            CombatAttributes::Accuracy => write!(f, "Accuracy"),
            CombatAttributes::Focus => write!(f, "Focus"),
            CombatAttributes::Evasion => write!(f, "Evasion"),
            CombatAttributes::Obscurity => write!(f, "Obscurity"),
            CombatAttributes::Hp => write!(f, "HP"),
            CombatAttributes::Mp => write!(f, "MP"),
            CombatAttributes::ArmorPenetration => write!(f, "Armor Pen."),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CombatantProperties {
    pub combatant_class: CombatantClass,
    pub inherent_attributes: HashMap<CombatAttributes, u16>,
    pub hit_points: u16,
    pub mana: u16,
    pub status_effects: Vec<StatusEffects>,
    pub equipment: HashMap<EquipmentSlots, Item>,
    pub abilities: HashMap<CombatantAbilityNames, CombatantAbility>,
    // pub traits: HashSet<CombatantTraits>
    pub selected_item_slot: Option<u8>,
    pub selected_ability_name: Option<CombatantAbilityNames>,
    pub ability_target_ids: Option<Vec<u32>>,
}

impl CombatantProperties {
    pub fn new(
        combatant_class: &CombatantClass,
        abilities: HashMap<CombatantAbilityNames, CombatantAbility>,
    ) -> CombatantProperties {
        CombatantProperties {
            combatant_class: combatant_class.clone(),
            inherent_attributes: HashMap::new(),
            hit_points: 0,
            mana: 0,
            status_effects: vec![],
            equipment: HashMap::new(),
            abilities,
            selected_item_slot: None,
            selected_ability_name: None,
            ability_target_ids: None,
        }
    }

    pub fn get_total_attributes(&self) -> HashMap<CombatAttributes, u16> {
        let mut total_attributes = HashMap::new();
        for attribute in CombatAttributes::iter() {
            total_attributes.insert(attribute, 0);
        }

        add_attributes_to_accumulator(&self.inherent_attributes, &mut total_attributes);

        for (_slot, item) in &self.equipment {
            match &item.item_properties {
                crate::items::ItemProperties::Consumable(_) => (),
                crate::items::ItemProperties::Equipment(equipment) => {
                    add_attributes_to_accumulator(&equipment.attributes, &mut total_attributes);
                    let base_ac = equipment.get_base_armor_class();
                    total_attributes
                        .entry(CombatAttributes::ArmorClass)
                        .and_modify(|value| *value += base_ac as u16)
                        .or_insert(base_ac as u16);
                }
            }
        }
        // after adding up attributes, determine if any equipped item still doesn't meet attribute
        // requirements, if so, remove it's attributes from the total
        for (_slot, item) in &self.equipment {
            let equipped_item_is_usable =
                item.requirements_satisfied_by_attributes(&total_attributes);
            if !equipped_item_is_usable {
                match &item.item_properties {
                    crate::items::ItemProperties::Consumable(_) => (),
                    crate::items::ItemProperties::Equipment(equipment) => {
                        remove_attributes_from_accumulator(
                            &equipment.attributes,
                            &mut total_attributes,
                        );
                        let base_ac = equipment.get_base_armor_class();
                        total_attributes
                            .entry(CombatAttributes::ArmorClass)
                            .and_modify(|value| *value -= base_ac as u16);
                    }
                }
            }
        }

        // derive accuracy from +acc, inherant, and all Dex
        let total_dex_option = total_attributes.get(&CombatAttributes::Dexterity);
        let total_acc = total_attributes
            .get(&CombatAttributes::Accuracy)
            .unwrap_or_else(|| &0);
        if let Some(dex) = total_dex_option {
            let accuracy_from_dex = DEX_TO_ACCURACY_RATIO * dex;
            total_attributes.insert(CombatAttributes::Accuracy, total_acc + accuracy_from_dex);
        }

        total_attributes
    }

    pub fn get_equipped_weapon_properties(
        &self,
        slot: &EquipmentSlots,
    ) -> Option<(&WeaponProperties, &Option<Vec<EquipmentTraits>>)> {
        match self.equipment.get(slot) {
            Some(item) => match &item.item_properties {
                ItemProperties::Consumable(_) => None,
                ItemProperties::Equipment(properties) => match &properties.equipment_type {
                    EquipmentTypes::OneHandedMeleeWeapon(_, weapon_properties)
                    | EquipmentTypes::TwoHandedMeleeWeapon(_, weapon_properties)
                    | EquipmentTypes::TwoHandedRangedWeapon(_, weapon_properties) => {
                        Some((&weapon_properties, &properties.traits))
                    }
                    _ => None,
                },
            },
            None => None,
        }
    }

    pub fn get_weapon_damage_and_hit_chance(
        weapon_properties: &WeaponProperties,
        traits: &Option<Vec<EquipmentTraits>>,
        combatant_base_damage: u16,
        accuracy: u16,
        is_off_hand: bool,
    ) -> (Range<u16>, u16) {
        let percent_damage_increase_from_trait =
            get_weapon_percent_damage_increase_trait_damage_modifier(traits);
        let mut modified_min = weapon_properties.damage.min as f32 + combatant_base_damage as f32;
        let mut modified_max = weapon_properties.damage.max as f32 + combatant_base_damage as f32;
        modified_min *= percent_damage_increase_from_trait;
        modified_max *= percent_damage_increase_from_trait;
        let mut modified_acc = accuracy as f32;

        if is_off_hand {
            modified_min *= OFF_HAND_DAMAGE_MODIFIER;
            modified_max *= OFF_HAND_DAMAGE_MODIFIER;
            modified_acc *= OFF_HAND_ACCURACY_MODIFIER;
        }

        (
            Range::new(modified_min as u16, modified_max as u16),
            modified_acc as u16,
        )
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
}

pub fn add_attributes_to_accumulator(
    attr: &HashMap<CombatAttributes, u16>,
    acc: &mut HashMap<CombatAttributes, u16>,
) {
    for (attribute, number) in attr {
        if let Some(value) = acc.get_mut(attribute) {
            *value += number
        }
    }
}

pub fn remove_attributes_from_accumulator(
    attr: &HashMap<CombatAttributes, u16>,
    acc: &mut HashMap<CombatAttributes, u16>,
) {
    for (attribute, number) in attr {
        if let Some(value) = acc.get_mut(attribute) {
            *value -= number
        }
    }
}
