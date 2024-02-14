pub mod create_starting_equipment;
mod equipment_blueprints;
mod equipment_generation_template_properties;
mod equipment_generation_templates;
mod generate_affixes;
mod generate_base_equipment;
mod generate_durability;
mod generate_equipment_combat_attributes;
mod generate_equipment_traits;
mod generate_weapon_damage_classifications;
mod make_indestructable_if_max_tier_durablity;
pub mod name_equipment;
pub mod print_random_equipments;
mod roll_equipment_properties_from_blueprint;
mod select_random_affix_types;
use rand::Rng;

use self::equipment_blueprints::body_armor_blueprints::body_armor_blueprint_from_base_item;
use self::equipment_blueprints::head_gear_blueprints::head_gear_blueprint_from_base_item;
use self::equipment_blueprints::jewelry_blueprints::jewelry_blueprint_from_base_item;
use self::equipment_blueprints::one_handed_melee_weapon_blueprints::one_handed_melee_weapon_blueprint_from_base_item;
use self::equipment_blueprints::shield_blueprints::shield_blueprint_from_base_item;
use self::equipment_blueprints::two_handed_melee_weapon_blueprints::two_handed_melee_weapon_blueprint_from_base_item;
use self::equipment_blueprints::two_handed_ranged_weapon_blueprints::two_handed_ranged_weapon_blueprint_from_base_item;
use self::equipment_blueprints::EquipmentBlueprint;
use self::generate_base_equipment::generate_base_equipment;
use self::generate_base_equipment::BaseEquipment;
use self::generate_weapon_damage_classifications::generate_weapon_damage_classifications;
use self::roll_equipment_properties_from_blueprint::roll_equipment_properties_from_blueprint;
use super::EquipmentProperties;
use crate::combatants::combat_attributes::CombatAttributes;
use std::collections::HashMap;

pub struct EquipmentPropertiesAndRequirements {
    pub equipment_properties: EquipmentProperties,
    pub requirements: Option<HashMap<CombatAttributes, u8>>,
}

pub fn generate_equipment_properties_from_base_item(
    level: u8,
) -> EquipmentPropertiesAndRequirements {
    let base_item = generate_base_equipment(level);
    // let base_item = BaseEquipment::OneHandedMeleeWeapon(OneHandedMeleeWeapons::RuneSword);
    // determine num prefixes and suffixes
    let mut rng = rand::thread_rng();
    let affix_randomizer_number = rng.gen_range(1..=100);
    let (num_prefixes, num_suffixes) =
        if affix_randomizer_number > 0 && affix_randomizer_number < 22 {
            (1, 0)
        } else if affix_randomizer_number >= 22 && affix_randomizer_number < 84 {
            (0, 1)
        } else {
            (1, 1)
        };

    let blueprint = match base_item {
        BaseEquipment::BodyArmor(base_item) => body_armor_blueprint_from_base_item(base_item),
        BaseEquipment::HeadGear(base_item) => head_gear_blueprint_from_base_item(base_item),
        BaseEquipment::Jewelry(base_item) => jewelry_blueprint_from_base_item(base_item),
        BaseEquipment::Shield(base_item) => shield_blueprint_from_base_item(base_item),
        BaseEquipment::OneHandedMeleeWeapon(base_item) => {
            one_handed_melee_weapon_blueprint_from_base_item(base_item)
        }
        BaseEquipment::TwoHandedMeleeWeapon(base_item) => {
            two_handed_melee_weapon_blueprint_from_base_item(base_item)
        }
        BaseEquipment::TwoHandedRangedWeapon(base_item) => {
            two_handed_ranged_weapon_blueprint_from_base_item(base_item)
        }
    };

    let EquipmentBlueprint {
        equipment_type,
        template_properties,
        possible_suffixes,
        possible_prefixes,
    } = blueprint;

    roll_equipment_properties_from_blueprint(
        equipment_type,
        level,
        &template_properties,
        &possible_prefixes,
        &possible_suffixes,
        num_prefixes,
        num_suffixes,
    )
}
