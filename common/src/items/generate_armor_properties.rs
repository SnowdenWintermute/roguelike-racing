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
    generate_equipment_properties::{self, generate_equipment_properties},
    select_random_affix_types::select_random_affix_types,
};
use crate::{combatants::CombatAttributes, primatives::MaxAndCurrent};
use rand::{seq::SliceRandom, Rng};
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

pub fn generate_armor_properties(base_armor: BodyArmors, level: u8) -> EquipmentProperties {
    let generation_template = BODY_ARMOR_GENERATION_TEMPLATES
        .get(&base_armor)
        .expect("a generation template should exist for each base armor type");

    let durability = generate_equipment_durability(generation_template.max_durability);
    let num_prefixes = 1;
    let num_suffixes = 1;

    let equipment_type = EquipmentTypes::BodyArmor(base_armor, generation_template.category);
    let requirements = generation_template.requirements.clone();
    let possible_prefixes: Vec<&(PrefixTypes, u8)> =
        BODY_ARMOR_POSSIBLE_PREFIXES_AND_TIERS.iter().collect();
    let possible_suffixes: Vec<&(SuffixTypes, u8)> =
        BODY_ARMOR_POSSIBLE_SUFFIXES_AND_TIERS.iter().collect();

    let prefix_types_and_tiers = select_random_affix_types(&possible_prefixes, num_prefixes);
    let suffix_types_and_tiers = select_random_affix_types(&possible_suffixes, num_suffixes);
    let affixes = generate_equipment_affixes(prefix_types_and_tiers, suffix_types_and_tiers, level);
    let attributes = generate_equipment_attributes(&affixes);

    EquipmentProperties {
        equipment_type,
        durability,
        attributes,
        requirements,
        affixes,
    }
}
