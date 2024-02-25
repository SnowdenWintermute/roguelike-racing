use std::collections::HashMap;

use super::one_handed_melee_weapons::OneHandedMeleeWeapons;
use crate::combat::hp_change_source_types::HpChangeSource;
use crate::combat::hp_change_source_types::HpChangeSourceCategories;
use crate::combat::hp_change_source_types::MeleeOrRanged;
use crate::combat::hp_change_source_types::PhysicalDamageTypes;
use crate::game::id_generator::IdGenerator;
use crate::items::equipment::two_handed_ranged_weapons::TwoHandedRangedWeapons;
use crate::items::equipment::weapon_properties::WeaponProperties;
use crate::items::equipment::EquipmentProperties;
use crate::items::equipment::EquipmentTypes;
use crate::items::Item;
use crate::items::ItemProperties;
use crate::primatives::EntityProperties;
use crate::primatives::Range;

pub enum PreMadeItems {
    SkeletonArcherShortBow,
    AnimalClaw,
    Fist,
    Stab,
}

impl PreMadeItems {
    pub fn get_item(&self, id_generator: &mut IdGenerator) -> Item {
        match self {
            PreMadeItems::SkeletonArcherShortBow => Item {
                entity_properties: EntityProperties {
                    id: id_generator.get_next_entity_id(),
                    name: "Skeleton Archer's Bow".to_string(),
                },
                item_level: 1,
                requirements: None,
                item_properties: ItemProperties::Equipment(EquipmentProperties {
                    equipment_type: EquipmentTypes::TwoHandedRangedWeapon(
                        TwoHandedRangedWeapons::ShortBow,
                        WeaponProperties {
                            damage_classifications: vec![HpChangeSource {
                                category: HpChangeSourceCategories::PhysicalDamage(
                                    MeleeOrRanged::Ranged,
                                ),
                                sub_category: Some(PhysicalDamageTypes::Piercing),
                                element: None,
                            }],
                            damage: Range::new(1, 4),
                        },
                    ),
                    durability: None,
                    attributes: HashMap::new(),
                    affixes: Vec::new(),
                    traits: None,
                }),
            },
            PreMadeItems::AnimalClaw => Item {
                entity_properties: EntityProperties {
                    id: id_generator.get_next_entity_id(),
                    name: "Animal Claw".to_string(),
                },
                item_level: 1,
                requirements: None,
                item_properties: ItemProperties::Equipment(EquipmentProperties {
                    equipment_type: EquipmentTypes::OneHandedMeleeWeapon(
                        OneHandedMeleeWeapons::Dagger,
                        WeaponProperties {
                            damage_classifications: vec![HpChangeSource {
                                category: HpChangeSourceCategories::PhysicalDamage(
                                    MeleeOrRanged::Melee,
                                ),
                                sub_category: Some(PhysicalDamageTypes::Slashing),
                                element: None,
                            }],
                            damage: Range::new(1, 4),
                        },
                    ),
                    durability: None,
                    attributes: HashMap::new(),
                    affixes: Vec::new(),
                    traits: None,
                }),
            },
            PreMadeItems::Fist => Item {
                entity_properties: EntityProperties {
                    id: id_generator.get_next_entity_id(),
                    name: "Fist".to_string(),
                },
                item_level: 1,
                requirements: None,
                item_properties: ItemProperties::Equipment(EquipmentProperties {
                    equipment_type: EquipmentTypes::OneHandedMeleeWeapon(
                        OneHandedMeleeWeapons::Stick,
                        WeaponProperties {
                            damage_classifications: vec![HpChangeSource {
                                category: HpChangeSourceCategories::PhysicalDamage(
                                    MeleeOrRanged::Melee,
                                ),
                                sub_category: Some(PhysicalDamageTypes::Blunt),
                                element: None,
                            }],
                            damage: Range::new(1, 4),
                        },
                    ),
                    durability: None,
                    attributes: HashMap::new(),
                    affixes: Vec::new(),
                    traits: None,
                }),
            },
            PreMadeItems::Stab => Item {
                entity_properties: EntityProperties {
                    id: id_generator.get_next_entity_id(),
                    name: "Stabber".to_string(),
                },
                item_level: 1,
                requirements: None,
                item_properties: ItemProperties::Equipment(EquipmentProperties {
                    equipment_type: EquipmentTypes::OneHandedMeleeWeapon(
                        OneHandedMeleeWeapons::Dagger,
                        WeaponProperties {
                            damage_classifications: vec![HpChangeSource {
                                category: HpChangeSourceCategories::PhysicalDamage(
                                    MeleeOrRanged::Melee,
                                ),
                                sub_category: Some(PhysicalDamageTypes::Piercing),
                                element: None,
                            }],
                            damage: Range::new(1, 4),
                        },
                    ),
                    durability: None,
                    attributes: HashMap::new(),
                    affixes: Vec::new(),
                    traits: None,
                }),
            },
        }
    }
}
