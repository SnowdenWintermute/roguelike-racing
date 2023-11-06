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
    headgear::{
        headgear_generation_templates::HEADGEAR_GENERATION_TEMPLATES,
        headgear_possible_affixes::{
            HEADGEAR_POSSIBLE_PREFIXES_AND_TIERS, HEADGEAR_POSSIBLE_SUFFIXES_AND_TIERS,
        },
    },
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
                &generation_template.template_properties,
                base_ac,
                None,
                &possible_prefixes,
                &possible_suffixes,
                num_prefixes,
                num_suffixes,
            )
        }
        BaseItem::HeadGear(base_headgear) => {
            let generation_template = HEADGEAR_GENERATION_TEMPLATES
                .get(&base_headgear)
                .expect("a generation template should exist for each base armor type");
            let equipment_type =
                EquipmentTypes::HeadGear(base_headgear, generation_template.category);
            let possible_prefixes: Vec<&(PrefixTypes, u8)> =
                HEADGEAR_POSSIBLE_PREFIXES_AND_TIERS.iter().collect();
            let possible_suffixes: Vec<&(SuffixTypes, u8)> =
                HEADGEAR_POSSIBLE_SUFFIXES_AND_TIERS.iter().collect();
            let base_ac = match &generation_template.template_properties.ac_range {
                Some(ac_range) => Some(rand::thread_rng().gen_range(ac_range.min..=ac_range.max)),
                None => None,
            };

            generate_equipment_properties(
                equipment_type,
                level,
                &generation_template.template_properties,
                base_ac,
                None,
                &possible_prefixes,
                &possible_suffixes,
                num_prefixes,
                num_suffixes,
            )
        }
        BaseItem::Jewelry => todo!(),
        BaseItem::Shield => todo!(),
        BaseItem::OneHandedMeleeWeapon => todo!(),
        BaseItem::TwoHandedMeleeWeapon => todo!(),
        BaseItem::TwoHandedRangedWeapon => todo!(),
    }
}
