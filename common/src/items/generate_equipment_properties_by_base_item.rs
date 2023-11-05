use super::{
    affixes::{PrefixTypes, SuffixTypes},
    body_armor::{
        body_armor_generation_templates::BODY_ARMOR_GENERATION_TEMPLATES,
        body_armor_possible_affixes::{
            BODY_ARMOR_POSSIBLE_PREFIXES_AND_TIERS, BODY_ARMOR_POSSIBLE_SUFFIXES_AND_TIERS,
        },
    },
    equipment::EquipmentTypes,
    equipment_base_items::BaseItem,
    generate_equipment_properties::generate_equipment_properties,
    EquipmentProperties, Item, ItemProperties,
};
use crate::{combatants::CombatAttributes, primatives::MaxAndCurrent};
use rand::prelude::*;
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub fn generate_equipment_properties_by_base_item(level: u8) -> EquipmentProperties {
    // GEN BASE ITEM
    let base_item = Item::generate_base_item(level);
    let num_prefixes = 1;
    let num_suffixes = 1;

    match base_item {
        BaseItem::Armor(base_armor) => {
            let generation_template = BODY_ARMOR_GENERATION_TEMPLATES
                .get(&base_armor)
                .expect("a generation template should exist for each base armor type");
            let equipment_type =
                EquipmentTypes::BodyArmor(base_armor, generation_template.category);
            let possible_prefixes: Vec<&(PrefixTypes, u8)> =
                BODY_ARMOR_POSSIBLE_PREFIXES_AND_TIERS.iter().collect();
            let possible_suffixes: Vec<&(SuffixTypes, u8)> =
                BODY_ARMOR_POSSIBLE_SUFFIXES_AND_TIERS.iter().collect();
            let base_ac = match &generation_template.template_properties.ac_range {
                Some(ac_range) => Some(rand::thread_rng().gen_range(ac_range.min..=ac_range.max)),
                None => None,
            };

            generate_equipment_properties(
                equipment_type,
                level,
                &generation_template.template_properties.requirements,
                generation_template.template_properties.max_durability,
                base_ac,
                None,
                &possible_prefixes,
                &possible_suffixes,
                num_prefixes,
                num_suffixes,
            )
        }
        BaseItem::Jewelry => todo!(),
        BaseItem::MeleeWeapon => todo!(),
        BaseItem::RangedWeapon => todo!(),
        BaseItem::Shield => todo!(),
        BaseItem::HeadGear(base_headgear) => todo!(),
    }
}
