use super::EquipmentBlueprint;
use crate::items::equipment::{
    affixes::{PrefixTypes, SuffixTypes},
    armor_properties::ArmorProperties,
    equipment_generation::{
        equipment_generation_templates::{
            head_gear_generation_templates::HEAD_GEAR_GENERATION_TEMPLATES,
            head_gear_possible_affixes::{
                HEAD_GEAR_POSSIBLE_PREFIXES_AND_TIERS, HEAD_GEAR_POSSIBLE_SUFFIXES_AND_TIERS,
            },
        },
        generate_base_equipment::BaseEquipment,
    },
    head_gears::HeadGears,
    EquipmentTypes,
};
use rand::Rng;

pub fn head_gear_blueprint_from_base_item<'a>(base_item: HeadGears) -> EquipmentBlueprint<'a> {
    let template = HEAD_GEAR_GENERATION_TEMPLATES
        .get(&base_item)
        .expect("a generation template should exist for each base item type");
    let base_ac = rand::thread_rng().gen_range(template.ac_range.min..=template.ac_range.max);
    let equipment_type =
        EquipmentTypes::HeadGear(base_item, ArmorProperties::new(template.category, base_ac));
    let possible_prefixes: Vec<&(PrefixTypes, u8)> =
        HEAD_GEAR_POSSIBLE_PREFIXES_AND_TIERS.iter().collect();
    let possible_suffixes: Vec<&(SuffixTypes, u8)> =
        HEAD_GEAR_POSSIBLE_SUFFIXES_AND_TIERS.iter().collect();

    EquipmentBlueprint::new(
        equipment_type,
        template.template_properties.clone(),
        possible_prefixes,
        possible_suffixes,
    )
}
