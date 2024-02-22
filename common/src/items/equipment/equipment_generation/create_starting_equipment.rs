#![allow(unused)]
use crate::combat::hp_change_source_types::HpChangeSource;
use crate::combat::hp_change_source_types::HpChangeSourceCategories;
use crate::combat::hp_change_source_types::MeleeOrRanged;
use crate::combat::hp_change_source_types::PhysicalDamageTypes;
use crate::combat::magical_elements::MagicalElements;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::combatants::combatant_classes::CombatantClass;
use crate::game::id_generator::IdGenerator;
use crate::items::equipment::affixes::Affix;
use crate::items::equipment::affixes::PrefixTypes;
use crate::items::equipment::affixes::SuffixTypes;
use crate::items::equipment::armor_properties::ArmorCategories;
use crate::items::equipment::armor_properties::ArmorProperties;
use crate::items::equipment::body_armors::BodyArmors;
use crate::items::equipment::head_gears::HeadGears;
use crate::items::equipment::one_handed_melee_weapons::OneHandedMeleeWeapons;
use crate::items::equipment::shield_properties::ShieldProperties;
use crate::items::equipment::shield_properties::ShieldSizes;
use crate::items::equipment::shields::Shields;
use crate::items::equipment::two_handed_melee_weapons::TwoHandedMeleeWeapons;
use crate::items::equipment::weapon_properties::WeaponProperties;
use crate::items::equipment::EquipmentProperties;
use crate::items::equipment::EquipmentSlots;
use crate::items::equipment::EquipmentTypes;
use crate::items::Item;
use crate::items::ItemProperties;
use crate::primatives::EntityProperties;
use crate::primatives::MaxAndCurrent;
use crate::primatives::Range;
use std::collections::HashMap;
use std::hash::Hash;

pub fn create_starting_equipment(
    id_generator: &mut IdGenerator,
    combatant_class: &CombatantClass,
) -> HashMap<EquipmentSlots, Item> {
    #[allow(unused_variables, dead_code, unused)]
    let mut starting_equipment = HashMap::new();

    let main_hand_properties = match combatant_class {
        CombatantClass::Warrior => EquipmentProperties::new(
            EquipmentTypes::OneHandedMeleeWeapon(
                OneHandedMeleeWeapons::Stick,
                WeaponProperties {
                    damage_classifications: vec![HpChangeSource::new(
                        HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                        Some(PhysicalDamageTypes::Piercing),
                        None,
                    )],
                    damage: Range::new(1, 4),
                },
            ),
            Some(MaxAndCurrent::new(2, 2)),
            HashMap::new(),
            vec![],
            None,
        ),
        CombatantClass::Mage => EquipmentProperties::new(
            EquipmentTypes::TwoHandedMeleeWeapon(
                TwoHandedMeleeWeapons::BoStaff,
                WeaponProperties {
                    damage_classifications: vec![HpChangeSource::new(
                        HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                        Some(PhysicalDamageTypes::Blunt),
                        None,
                    )],
                    damage: Range::new(1, 3),
                },
            ),
            Some(MaxAndCurrent::new(2, 2)),
            HashMap::new(),
            vec![],
            None,
        ),
        CombatantClass::Rogue => EquipmentProperties::new(
            EquipmentTypes::OneHandedMeleeWeapon(
                OneHandedMeleeWeapons::Dagger,
                WeaponProperties {
                    damage_classifications: vec![HpChangeSource::new(
                        HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                        Some(PhysicalDamageTypes::Blunt),
                        None,
                    )],
                    damage: Range::new(1, 3),
                },
            ),
            Some(MaxAndCurrent::new(2, 2)),
            HashMap::new(),
            vec![],
            None,
        ),
    };

    let offhand_weapon_option = if combatant_class == &CombatantClass::Rogue {
        Some(main_hand_properties.clone())
    } else {
        None
    };

    let weapon_name = match combatant_class {
        CombatantClass::Warrior => "Mud Soaked Stick",
        CombatantClass::Mage => "Rotting Branch",
        CombatantClass::Rogue => "Butter Knife",
    };

    let main_hand_entity_properties = EntityProperties {
        id: id_generator.get_next_entity_id(),
        name: weapon_name.to_string(),
    };

    let main_hand = Item {
        entity_properties: main_hand_entity_properties,
        item_level: 0,
        requirements: None,
        item_properties: ItemProperties::Equipment(main_hand_properties),
    };

    if let Some(offhand) = offhand_weapon_option {
        let offhand_entity_properties = EntityProperties {
            id: id_generator.get_next_entity_id(),
            name: weapon_name.to_string(),
        };
        let off_hand = Item {
            entity_properties: offhand_entity_properties,
            item_level: 0,
            requirements: None,
            item_properties: ItemProperties::Equipment(offhand),
        };
        starting_equipment.insert(EquipmentSlots::OffHand, off_hand);
    }

    starting_equipment.insert(EquipmentSlots::MainHand, main_hand);

    starting_equipment
}
