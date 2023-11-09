use super::equipment_generation_template_properties::EquipmentGenerationTemplateProperties;
use super::generate_affixes::generate_affixes;
use super::generate_durability::generate_durability;
use super::generate_equipment_combat_attributes::generate_equipment_combat_attributes;
use super::generate_equipment_traits::generate_equipment_traits;
use super::make_indestructable_if_max_tier_durablity::make_indestructable_if_max_tier_durability;
use super::select_random_affix_types::select_random_affix_types;
use crate::items::equipment::affixes::PrefixTypes;
use crate::items::equipment::affixes::SuffixTypes;
use crate::items::equipment::EquipmentProperties;
use crate::items::equipment::EquipmentTypes;

pub fn roll_equipment_properties_from_blueprint(
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

    let affixes = generate_affixes(prefix_types_and_tiers, suffix_types_and_tiers, level);
    make_indestructable_if_max_tier_durability(&affixes, &mut durability);
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
