use super::monster_types::MonsterTypes;
use crate::combat::hp_change_source_types::HpChangeSource;
use crate::combat::hp_change_source_types::HpChangeSourceCategories;
use crate::combat::hp_change_source_types::MeleeOrRanged;
use crate::combat::hp_change_source_types::PhysicalDamageTypes;
use crate::game::id_generator::IdGenerator;
use crate::items::equipment::two_handed_ranged_weapons::TwoHandedRangedWeapons;
use crate::items::equipment::weapon_properties::WeaponProperties;
use crate::items::equipment::EquipmentProperties;
use crate::items::equipment::EquipmentSlots;
use crate::items::equipment::EquipmentTypes;
use crate::items::Item;
use crate::items::ItemProperties;
use crate::primatives::EntityProperties;
use crate::primatives::Range;
use std::collections::HashMap;

impl MonsterTypes {
    pub fn get_equipment(&self, id_generator: &mut IdGenerator) -> HashMap<EquipmentSlots, Item> {
        match self {
            MonsterTypes::SkeletonArcher => HashMap::from([(
                EquipmentSlots::MainHand,
                Item {
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
            )]),
            _ => HashMap::new(),
        }
    }
}
