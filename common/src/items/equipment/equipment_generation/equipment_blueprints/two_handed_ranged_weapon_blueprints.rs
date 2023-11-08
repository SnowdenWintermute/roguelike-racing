use crate::items::equipment::{
    affixes::{PrefixTypes, SuffixTypes},
    equipment_generation::{
        equipment_generation_templates::{
            two_handed_ranged_weapon_generation_templates::TWO_HANDED_RANGED_WEAPON_GENERATION_TEMPLATES,
            two_handed_ranged_weapon_possible_affixes::{
                TWO_HANDED_RANGED_WEAPONS_POSSIBLE_PREFIXES_AND_TIERS,
                TWO_HANDED_RANGED_WEAPONS_POSSIBLE_SUFFIXES_AND_TIERS,
            },
        },
        generate_weapon_damage_classifications,
    },
    two_handed_ranged_weapons::TwoHandedRangedWeapons,
    weapon_properties::WeaponProperties,
    EquipmentTypes,
};

use super::EquipmentBlueprint;

pub fn two_handed_ranged_weapon_blueprint_from_base_item<'a>(
    base_item: TwoHandedRangedWeapons,
) -> EquipmentBlueprint<'a> {
    let template = TWO_HANDED_RANGED_WEAPON_GENERATION_TEMPLATES
        .get(&base_item)
        .expect("a generation template should exist for each base item type");
    let possible_prefixes: Vec<&(PrefixTypes, u8)> =
        TWO_HANDED_RANGED_WEAPONS_POSSIBLE_PREFIXES_AND_TIERS
            .iter()
            .collect();
    let possible_suffixes: Vec<&(SuffixTypes, u8)> =
        TWO_HANDED_RANGED_WEAPONS_POSSIBLE_SUFFIXES_AND_TIERS
            .iter()
            .collect();

    let damage_classifications = generate_weapon_damage_classifications(
        &template.possbile_damage_classifications,
        template.num_damage_classifications,
    );

    let equipment_type = EquipmentTypes::TwoHandedRangedWeapon(
        base_item,
        WeaponProperties::new(damage_classifications, template.damage.clone()),
    );

    EquipmentBlueprint::new(
        equipment_type,
        template.template_properties.clone(),
        possible_prefixes,
        possible_suffixes,
    )
}
