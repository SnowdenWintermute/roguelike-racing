use super::{
    affixes::{Affix, PrefixTypes, SuffixTypes},
    body_armor::{
        body_armor_generation_templates::BODY_ARMOR_GENERATION_TEMPLATES,
        body_armor_possible_affixes::{
            BODY_ARMOR_POSSIBLE_PREFIXES_AND_TIERS, BODY_ARMOR_POSSIBLE_SUFFIXES_AND_TIERS,
        },
        BodyArmors,
    },
    equipment::{EquipmentProperties, EquipmentTypes},
    generate_equipment_affixes::generate_equipment_affixes,
    generate_equipment_attributes::{self, generate_equipment_attributes},
    generate_equipment_durability::generate_equipment_durability,
    select_random_affix_types::select_random_affix_types,
};
use crate::{combatants::CombatAttributes, primatives::MaxAndCurrent};
use rand::{seq::SliceRandom, Rng};
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

pub fn generate_equipment_properties(
    equipment_type: EquipmentTypes,
    level: u8,
    requirements: &HashMap<CombatAttributes, u8>,
    max_durability: u8,
    base_ac: Option<u8>,
    base_damage: Option<u8>,
    possible_prefixes: &Vec<&(PrefixTypes, u8)>,
    possible_suffixes: &Vec<&(SuffixTypes, u8)>,
    num_prefixes: u8,
    num_suffixes: u8,
) -> EquipmentProperties {
    let requirements = requirements.clone();
    let durability = generate_equipment_durability(max_durability);
    let prefix_types_and_tiers = select_random_affix_types(&possible_prefixes, num_prefixes);
    let suffix_types_and_tiers = select_random_affix_types(&possible_suffixes, num_suffixes);
    let affixes = generate_equipment_affixes(prefix_types_and_tiers, suffix_types_and_tiers, level);
    let attributes = generate_equipment_attributes(&affixes);

    EquipmentProperties {
        equipment_type,
        durability,
        base_ac,
        base_damage,
        attributes,
        requirements,
        affixes,
    }
}
