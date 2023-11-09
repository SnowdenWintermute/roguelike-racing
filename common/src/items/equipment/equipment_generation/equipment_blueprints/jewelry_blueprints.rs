use super::EquipmentBlueprint;
use crate::{
    app_consts::DEEPEST_FLOOR,
    items::equipment::{
        affixes::{PrefixTypes, SuffixTypes},
        equipment_generation::{
            equipment_generation_template_properties::EquipmentGenerationTemplateProperties,
            equipment_generation_templates::jewelry_possible_affixes::{
                JEWELRY_POSSIBLE_PREFIXES_AND_TIERS, JEWELRY_POSSIBLE_SUFFIXES_AND_TIERS,
            },
        },
        jewelries::Jewelries,
        EquipmentTypes,
    },
    primatives::Range,
};
use rand::Rng;
use std::collections::HashMap;

pub fn jewelry_blueprint_from_base_item<'a>(_base_item: Jewelries) -> EquipmentBlueprint<'a> {
    let template_properties = EquipmentGenerationTemplateProperties {
        level_range: Range::new(1, DEEPEST_FLOOR),
        max_durability: None,
        requirements: HashMap::new(),
        affix_modifiers: None,
        traits: None,
    };
    let possible_prefixes: Vec<&(PrefixTypes, u8)> =
        JEWELRY_POSSIBLE_PREFIXES_AND_TIERS.iter().collect();
    let possible_suffixes: Vec<&(SuffixTypes, u8)> =
        JEWELRY_POSSIBLE_SUFFIXES_AND_TIERS.iter().collect();
    let possible_equipment_types = vec![
        EquipmentTypes::Ring,
        EquipmentTypes::Ring,
        EquipmentTypes::Amulet,
    ];
    let equipment_type_index = rand::thread_rng().gen_range(0..possible_equipment_types.len());
    let equipment_type = possible_equipment_types[equipment_type_index].clone();

    EquipmentBlueprint::new(
        equipment_type,
        template_properties.clone(),
        possible_prefixes,
        possible_suffixes,
    )
}
