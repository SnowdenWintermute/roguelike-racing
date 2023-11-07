use super::{
    equipment_generation_template_properties::EquipmentGenerationTemplateProperties,
    generate_affixes::generate_affixes, generate_durability::generate_durability,
    generate_equipment_combat_attributes::generate_equipment_combat_attributes,
    generate_equipment_traits::generate_equipment_traits,
    select_random_affix_types::select_random_affix_types,
};
use crate::combatants::CombatAttributes;
use crate::items::equipment::affixes::{PrefixTypes, SuffixTypes};
use crate::items::equipment::{EquipmentProperties, EquipmentTypes};
use crate::primatives::{MaxAndCurrent, Range};
use rand::{seq::SliceRandom, Rng};
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

pub fn roll_equipment_properties_from_template(
    equipment_type: EquipmentTypes,
    level: u8,
    template_properties: &EquipmentGenerationTemplateProperties,
    possible_prefixes: &Vec<&(PrefixTypes, u8)>,
    possible_suffixes: &Vec<&(SuffixTypes, u8)>,
    num_prefixes: u8,
    num_suffixes: u8,
) -> EquipmentProperties {
    let requirements = template_properties.requirements.clone();
    let mut durability = generate_durability(template_properties.max_durability);
    let affix_modifiers = template_properties.get_affix_modifers();
    let prefix_types_and_tiers = select_random_affix_types(
        &possible_prefixes,
        num_prefixes,
        affix_modifiers.prefix_tier_overrides,
        &affix_modifiers.prefix_exclusions,
    );
    let suffix_types_and_tiers = select_random_affix_types(
        &possible_suffixes,
        num_suffixes,
        &affix_modifiers.suffix_tier_overrides,
        &affix_modifiers.suffix_exclusions,
    );

    // Make item indestructable if highest tier durability
    for suffix_and_tier in &suffix_types_and_tiers {
        if suffix_and_tier.0 == SuffixTypes::Durability && suffix_and_tier.1 == 5 {
            durability = None
        }
    }

    let affixes = generate_affixes(prefix_types_and_tiers, suffix_types_and_tiers, level);
    let traits = generate_equipment_traits(&affixes);
    let attributes = generate_equipment_combat_attributes(&affixes);

    EquipmentProperties {
        equipment_type,
        durability,
        attributes,
        requirements,
        affixes,
        traits,
    }
}
