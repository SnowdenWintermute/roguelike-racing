use super::EquipmentBlueprint;
use crate::items::equipment::{
    affixes::{PrefixTypes, SuffixTypes},
    armor_properties::ArmorProperties,
    body_armors::BodyArmors,
    equipment_generation::{
        equipment_generation_templates::{
            body_armor_generation_templates::BODY_ARMOR_GENERATION_TEMPLATES,
            body_armor_possible_affixes::{
                BODY_ARMOR_POSSIBLE_PREFIXES_AND_TIERS, BODY_ARMOR_POSSIBLE_SUFFIXES_AND_TIERS,
            },
        },
        generate_base_equipment::BaseEquipment,
    },
    EquipmentTypes,
};
use rand::Rng;

pub fn body_armor_blueprint_from_base_item<'a>(base_item: BodyArmors) -> EquipmentBlueprint<'a> {
    let template = BODY_ARMOR_GENERATION_TEMPLATES
        .get(&base_item)
        .expect("a generation template should exist for each base item type");
    let base_ac = rand::thread_rng().gen_range(template.ac_range.min..=template.ac_range.max);
    let equipment_type =
        EquipmentTypes::BodyArmor(base_item, ArmorProperties::new(template.category, base_ac));
    let possible_prefixes: Vec<&(PrefixTypes, u8)> =
        BODY_ARMOR_POSSIBLE_PREFIXES_AND_TIERS.iter().collect();
    let possible_suffixes: Vec<&(SuffixTypes, u8)> =
        BODY_ARMOR_POSSIBLE_SUFFIXES_AND_TIERS.iter().collect();

    EquipmentBlueprint::new(
        equipment_type,
        template.template_properties.clone(),
        possible_prefixes,
        possible_suffixes,
    )
}
