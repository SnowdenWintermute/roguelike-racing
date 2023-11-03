use super::{
    affixes::{Affix, PrefixTypes, SuffixTypes},
    armors::{
        ArmorCategories, Armors, ARMOR_GENERATION_TEMPLATES, ARMOR_POSSIBLE_PREFIXES_AND_TIERS,
        ARMOR_POSSIBLE_SUFFIXES_AND_TIERS,
    },
    equipment::{EquipmentProperties, EquipmentTypes},
    generate_equipment_affixes::generate_equipment_affixes,
};
use crate::{combatants::CombatAttributes, primatives::MaxAndCurrent};
use rand::{seq::SliceRandom, Rng};
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

pub fn generate_armor_properties(base_armor: Armors, level: u8) -> EquipmentProperties {
    let generation_template = ARMOR_GENERATION_TEMPLATES
        .get(&base_armor)
        .expect("a generation template should exist for each base armor type");

    let min_starting_durability = 1 + generation_template.max_durability / 4;
    let max_starting_durability = 3 * generation_template.max_durability / 4;
    let current_durability =
        rand::thread_rng().gen_range(min_starting_durability..=max_starting_durability);
    let durability = Some(MaxAndCurrent {
        current: current_durability,
        max: generation_template.max_durability,
    });
    let num_prefixes = 1;
    let num_suffixes = 1;

    let equipment_type = EquipmentTypes::BodyArmor(base_armor, generation_template.category);
    let requirements = generation_template.requirements.clone();
    let possible_prefixes: Vec<&(PrefixTypes, u8)> =
        ARMOR_POSSIBLE_PREFIXES_AND_TIERS.iter().collect();
    let possible_suffixes: Vec<&(SuffixTypes, u8)> =
        ARMOR_POSSIBLE_SUFFIXES_AND_TIERS.iter().collect();

    let prefix_types_and_tiers = select_random_affix_types(&possible_prefixes, num_prefixes);
    let suffix_types_and_tiers = select_random_affix_types(&possible_suffixes, num_suffixes);
    let affixes = generate_equipment_affixes(prefix_types_and_tiers, suffix_types_and_tiers, level);

    EquipmentProperties {
        equipment_type,
        durability,
        attributes: todo!(),
        requirements,
    }
}

pub fn select_random_affix_types<T: Clone>(
    possible_affix_types: &Vec<&T>,
    num_affixes: u8,
) -> Vec<T> {
    let mut affix_types_to_return = Vec::new();
    if num_affixes < 1 {
        return affix_types_to_return;
    }
    let mut remaining_affixes_possible = possible_affix_types.clone();
    for i in 0..=num_affixes {
        // this shouldn't happen if we don't allow items with a higher number of prefixes than the
        // number of prefix types, but just in case we'll exit early
        if remaining_affixes_possible.len() < 1 {
            break;
        }
        let random_affix_index = rand::thread_rng().gen_range(0..remaining_affixes_possible.len());
        let affix_type = remaining_affixes_possible
            .remove(random_affix_index)
            .clone();
        affix_types_to_return.push(affix_type);
    }
    affix_types_to_return
}
