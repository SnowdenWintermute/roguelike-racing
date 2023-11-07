mod equipment_generation_template_properties;
pub mod equipment_generation_templates;
mod generate_affixes;
mod generate_base_equipment;
mod generate_durability;
mod generate_equipment_combat_attributes;
mod generate_equipment_traits;
mod generate_weapon_damage_classifications;
mod roll_equipment_properties_from_template;
mod select_random_affix_types;
use self::{
    equipment_generation_templates::{
        body_armor_generation_templates::BODY_ARMOR_GENERATION_TEMPLATES,
        body_armor_possible_affixes::{
            BODY_ARMOR_POSSIBLE_PREFIXES_AND_TIERS, BODY_ARMOR_POSSIBLE_SUFFIXES_AND_TIERS,
        },
        head_gear_generation_templates::HEAD_GEAR_GENERATION_TEMPLATES,
        head_gear_possible_affixes::{
            HEAD_GEAR_POSSIBLE_PREFIXES_AND_TIERS, HEAD_GEAR_POSSIBLE_SUFFIXES_AND_TIERS,
        },
        one_handed_melee_weapon_generation_templates::ONE_HANDED_MELEE_WEAPON_GENERATION_TEMPLATES,
        one_handed_melee_weapon_possible_affixes::{
            ONE_HANDED_MELEE_WEAPONS_POSSIBLE_PREFIXES_AND_TIERS,
            ONE_HANDED_MELEE_WEAPONS_POSSIBLE_SUFFIXES_AND_TIERS,
        },
    },
    generate_base_equipment::{generate_base_equipment, BaseEquipment},
    generate_weapon_damage_classifications::generate_weapon_damage_classifications,
    roll_equipment_properties_from_template::roll_equipment_properties_from_template,
};
use super::{
    affixes::{PrefixTypes, SuffixTypes},
    armor::ArmorProperties,
    weapons::WeaponProperties,
    EquipmentProperties, EquipmentTypes,
};
use crate::{combatants::CombatAttributes, items::Item, primatives::MaxAndCurrent};
use rand::prelude::*;
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub fn generate_equipment_properties_from_base_item(level: u8) -> EquipmentProperties {
    // GEN BASE ITEM
    let base_item = generate_base_equipment(level);
    let num_prefixes = 1;
    let num_suffixes = 1;

    match base_item {
        BaseEquipment::Armor(base_item) => {
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

            roll_equipment_properties_from_template(
                equipment_type,
                level,
                &template.template_properties,
                &possible_prefixes,
                &possible_suffixes,
                num_prefixes,
                num_suffixes,
            )
        }
        BaseEquipment::HeadGear(base_item) => {
            let template = HEAD_GEAR_GENERATION_TEMPLATES
                .get(&base_item)
                .expect("a generation template should exist for each base armor type");
            let base_ac =
                rand::thread_rng().gen_range(template.ac_range.min..=template.ac_range.max);
            let equipment_type = EquipmentTypes::HeadGear(
                base_item,
                ArmorProperties::new(template.category, base_ac),
            );
            let possible_prefixes: Vec<&(PrefixTypes, u8)> =
                HEAD_GEAR_POSSIBLE_PREFIXES_AND_TIERS.iter().collect();
            let possible_suffixes: Vec<&(SuffixTypes, u8)> =
                HEAD_GEAR_POSSIBLE_SUFFIXES_AND_TIERS.iter().collect();

            roll_equipment_properties_from_template(
                equipment_type,
                level,
                &template.template_properties,
                &possible_prefixes,
                &possible_suffixes,
                num_prefixes,
                num_suffixes,
            )
        }
        BaseEquipment::Jewelry => todo!(),
        BaseEquipment::Shield => todo!(),
        BaseEquipment::OneHandedMeleeWeapon(base_item) => {
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

            roll_equipment_properties_from_template(
                equipment_type,
                level,
                &template.template_properties,
                &possible_prefixes,
                &possible_suffixes,
                num_prefixes,
                num_suffixes,
            )
        }
        BaseEquipment::TwoHandedMeleeWeapon => todo!(),
        BaseEquipment::TwoHandedRangedWeapon => todo!(),
    }
}
