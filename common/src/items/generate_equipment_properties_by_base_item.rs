use super::{
    affixes::{PrefixTypes, SuffixTypes},
    body_armor::{
        body_armor_generation_templates::BODY_ARMOR_GENERATION_TEMPLATES,
        body_armor_possible_affixes::{
            BODY_ARMOR_POSSIBLE_PREFIXES_AND_TIERS, BODY_ARMOR_POSSIBLE_SUFFIXES_AND_TIERS,
        },
        ArmorProperties,
    },
    equipment::EquipmentTypes,
    equipment_base_items::BaseItem,
    generate_equipment_properties::generate_equipment_properties,
    generate_weapon_damage_classifications::generate_weapon_damage_classifications,
    headgear::{
        headgear_generation_templates::HEADGEAR_GENERATION_TEMPLATES,
        headgear_possible_affixes::{
            HEADGEAR_POSSIBLE_PREFIXES_AND_TIERS, HEADGEAR_POSSIBLE_SUFFIXES_AND_TIERS,
        },
    },
    one_handed_melee_weapons::{
        one_handed_melee_weapon_generation_templates::ONE_HANDED_MELEE_WEAPON_GENERATION_TEMPLATES,
        one_handed_melee_weapons_possible_affixes::{
            ONE_HANDED_MELEE_WEAPONS_POSSIBLE_PREFIXES_AND_TIERS,
            ONE_HANDED_MELEE_WEAPONS_POSSIBLE_SUFFIXES_AND_TIERS,
        },
        WeaponProperties,
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
        BaseItem::Armor(base_item) => {
            let template = BODY_ARMOR_GENERATION_TEMPLATES
                .get(&base_item)
                .expect("a generation template should exist for each base armor type");
            let base_ac =
                rand::thread_rng().gen_range(template.ac_range.min..=template.ac_range.max);
            let equipment_type = EquipmentTypes::BodyArmor(
                base_item,
                ArmorProperties::new(template.category, base_ac),
            );
            let possible_prefixes: Vec<&(PrefixTypes, u8)> =
                BODY_ARMOR_POSSIBLE_PREFIXES_AND_TIERS.iter().collect();
            let possible_suffixes: Vec<&(SuffixTypes, u8)> =
                BODY_ARMOR_POSSIBLE_SUFFIXES_AND_TIERS.iter().collect();

            generate_equipment_properties(
                equipment_type,
                level,
                &template.template_properties,
                &possible_prefixes,
                &possible_suffixes,
                num_prefixes,
                num_suffixes,
            )
        }
        BaseItem::HeadGear(base_item) => {
            let template = HEADGEAR_GENERATION_TEMPLATES
                .get(&base_item)
                .expect("a generation template should exist for each base armor type");
            let base_ac =
                rand::thread_rng().gen_range(template.ac_range.min..=template.ac_range.max);
            let equipment_type = EquipmentTypes::HeadGear(
                base_item,
                ArmorProperties::new(template.category, base_ac),
            );
            let possible_prefixes: Vec<&(PrefixTypes, u8)> =
                HEADGEAR_POSSIBLE_PREFIXES_AND_TIERS.iter().collect();
            let possible_suffixes: Vec<&(SuffixTypes, u8)> =
                HEADGEAR_POSSIBLE_SUFFIXES_AND_TIERS.iter().collect();

            generate_equipment_properties(
                equipment_type,
                level,
                &template.template_properties,
                &possible_prefixes,
                &possible_suffixes,
                num_prefixes,
                num_suffixes,
            )
        }
        BaseItem::Jewelry => todo!(),
        BaseItem::Shield => todo!(),
        BaseItem::OneHandedMeleeWeapon(base_item) => {
            let template = ONE_HANDED_MELEE_WEAPON_GENERATION_TEMPLATES
                .get(&base_item)
                .expect("a generation template should exist for each base armor type");
            let possible_prefixes: Vec<&(PrefixTypes, u8)> =
                ONE_HANDED_MELEE_WEAPONS_POSSIBLE_PREFIXES_AND_TIERS
                    .iter()
                    .collect();
            let possible_suffixes: Vec<&(SuffixTypes, u8)> =
                ONE_HANDED_MELEE_WEAPONS_POSSIBLE_SUFFIXES_AND_TIERS
                    .iter()
                    .collect();

            let damage_classifications = generate_weapon_damage_classifications(
                &template.possbile_damage_classifications,
                template.num_damage_classifications,
            );

            let equipment_type = EquipmentTypes::OneHandedMeleeWeapon(
                base_item,
                WeaponProperties::new(damage_classifications, template.damage.clone()),
            );

            generate_equipment_properties(
                equipment_type,
                level,
                &template.template_properties,
                &possible_prefixes,
                &possible_suffixes,
                num_prefixes,
                num_suffixes,
            )
        }
        BaseItem::TwoHandedMeleeWeapon => todo!(),
        BaseItem::TwoHandedRangedWeapon => todo!(),
    }
}
