use rand::Rng;

use super::EquipmentBlueprint;
use crate::items::equipment::{
    affixes::{PrefixTypes, SuffixTypes},
    equipment_generation::equipment_generation_templates::{
        shield_generation_templates::SHIELD_GENERATION_TEMPLATES,
        shield_possible_affixes::{
            SHIELD_POSSIBLE_PREFIXES_AND_TIERS, SHIELD_POSSIBLE_SUFFIXES_AND_TIERS,
        },
    },
    shield_properties::ShieldProperties,
    shields::Shields,
    EquipmentTypes,
};

pub fn shield_blueprint_from_base_item<'a>(base_item: Shields) -> EquipmentBlueprint<'a> {
    let template = SHIELD_GENERATION_TEMPLATES
        .get(&base_item)
        .expect("a generation template should exist for each base item type");
    let base_ac = rand::thread_rng().gen_range(template.ac_range.min..=template.ac_range.max);
    let equipment_type =
        EquipmentTypes::Shield(base_item, ShieldProperties::new(template.size, base_ac));
    let possible_prefixes: Vec<&(PrefixTypes, u8)> =
        SHIELD_POSSIBLE_PREFIXES_AND_TIERS.iter().collect();
    let possible_suffixes: Vec<&(SuffixTypes, u8)> =
        SHIELD_POSSIBLE_SUFFIXES_AND_TIERS.iter().collect();

    EquipmentBlueprint::new(
        equipment_type,
        template.template_properties.clone(),
        possible_prefixes,
        possible_suffixes,
    )
}
