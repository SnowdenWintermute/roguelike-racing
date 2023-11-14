use self::abilities::CombatantAbility;
use self::abilities::CombatantAbilityNames;
use crate::items::equipment::EquipmentSlots;
use crate::items::Item;
use crate::primatives::MaxAndCurrent;
use crate::status_effects::StatusEffects;
use core::fmt;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
pub mod abilities;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CombatantClass {
    Warrior,
    Mage,
    Rogue,
    Monster,
}

impl fmt::Display for CombatantClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CombatantClass::Warrior => write!(f, "Warrior"),
            CombatantClass::Mage => write!(f, "Mage"),
            CombatantClass::Rogue => write!(f, "Rogue"),
            CombatantClass::Monster => write!(f, "Monster"),
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
            CombatAttributes::ArmorPenetration => write!(f, "Armor Penetration"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CombatantProperties {
    pub combatant_class: CombatantClass,
    pub inherent_attributes: HashMap<CombatAttributes, u16>,
    pub hit_points: MaxAndCurrent<u16>,
    pub mana: MaxAndCurrent<u16>,
    pub status_effects: Vec<StatusEffects>,
    pub equipment: HashMap<EquipmentSlots, Item>,
    pub abilities: HashMap<CombatantAbilityNames, CombatantAbility>,
    // pub traits: HashSet<CombatantTraits>
    pub target_ids: Option<Vec<u32>>,
    pub selected_ability_slot: Option<u8>,
    pub selected_item_slot: Option<u8>,
}

impl CombatantProperties {
    pub fn new(
        combatant_class: &CombatantClass,
        abilities: HashMap<CombatantAbilityNames, CombatantAbility>,
    ) -> CombatantProperties {
        CombatantProperties {
            combatant_class: combatant_class.clone(),
            inherent_attributes: HashMap::new(),
            hit_points: MaxAndCurrent::new(10, 10),
            mana: MaxAndCurrent::new(10, 10),
            status_effects: vec![],
            equipment: HashMap::new(),
            abilities,
            selected_item_slot: None,
            selected_ability_slot: None,
            target_ids: None,
        }
    }

    pub fn get_total_attributes(&mut self) -> HashMap<CombatAttributes, u16> {
        let mut total_attributes = HashMap::new();
        for attribute in CombatAttributes::iter() {
            total_attributes.insert(attribute, 0);
        }

        add_attributes_to_accumulator(&self.inherent_attributes, &mut total_attributes);

        for (_slot, item) in self.equipment.clone() {
            match item.item_properties {
                crate::items::ItemProperties::Consumable(_) => (),
                crate::items::ItemProperties::Equipment(equipment) => {
                    add_attributes_to_accumulator(&equipment.attributes, &mut total_attributes)
                }
            }
        }

        total_attributes
    }
}

pub fn add_attributes_to_accumulator(
    attr: &HashMap<CombatAttributes, u16>,
    acc: &mut HashMap<CombatAttributes, u16>,
) {
    for (attribute, number) in attr {
        acc.entry(*attribute)
            .and_modify(|v| *v += number)
            .or_insert(*number);
    }
}
