#![allow(unused)]
use crate::combat::hp_change_source_types::HpChangeSource;
use crate::combat::hp_change_source_types::HpChangeSourceCategories;
use crate::combatants::combat_attributes::CombatAttributes;
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

pub fn create_starting_equipment(id_generator: &mut IdGenerator) -> HashMap<EquipmentSlots, Item> {
    #[allow(unused_variables, dead_code, unused)]
    let mut starting_equipment = HashMap::new();

    let chest_properties = EquipmentProperties {
        equipment_type: EquipmentTypes::BodyArmor(
            BodyArmors::Rags,
            ArmorProperties {
                armor_category: ArmorCategories::Cloth,
                armor_class: 1,
            },
        ),
        durability: Some(MaxAndCurrent::new(4, 4)),
        attributes: HashMap::new(),
        affixes: vec![],
        traits: None,
    };

    let chest_entity_properties = EntityProperties {
        id: id_generator.get_next_entity_id(),
        name: "Old Sack".to_string(),
    };

    let _chest = Item {
        entity_properties: chest_entity_properties,
        item_level: 0,
        requirements: None,
        item_properties: ItemProperties::Equipment(chest_properties),
    };

    let mut head_properties = EquipmentProperties {
        equipment_type: EquipmentTypes::HeadGear(
            HeadGears::Cap,
            ArmorProperties {
                armor_category: ArmorCategories::Cloth,
                armor_class: 1,
            },
        ),
        durability: Some(MaxAndCurrent::new(2, 2)),
        attributes: HashMap::new(),
        affixes: vec![Affix::Suffix(SuffixTypes::Intelligence, 1)],
        traits: None,
    };

    head_properties
        .attributes
        .insert(CombatAttributes::Intelligence, 1);

    let head_entity_properties = EntityProperties {
        id: id_generator.get_next_entity_id(),
        name: "Newspaper Hat".to_string(),
    };

    let mut head = Item {
        entity_properties: head_entity_properties,
        item_level: 0,
        requirements: None,
        item_properties: ItemProperties::Equipment(head_properties),
    };
    let mut requirements = HashMap::new();
    requirements.insert(CombatAttributes::Dexterity, 2);
    head.requirements = Some(requirements);

    let main_hand_properties = EquipmentProperties {
        equipment_type: EquipmentTypes::OneHandedMeleeWeapon(
            OneHandedMeleeWeapons::Stick,
            WeaponProperties {
                damage_classifications: vec![HpChangeSource::new(
                    HpChangeSourceCategories::PhysicalDamage,
                    None,
                    None,
                )],
                damage: Range::new(1, 4),
            },
        ),
        durability: Some(MaxAndCurrent::new(2, 2)),
        attributes: HashMap::new(),
        affixes: vec![],
        traits: None,
    };

    let main_hand_entity_properties = EntityProperties {
        id: id_generator.get_next_entity_id(),
        name: "Mud Soaked Branch".to_string(),
    };

    let main_hand = Item {
        entity_properties: main_hand_entity_properties,
        item_level: 0,
        requirements: None,
        item_properties: ItemProperties::Equipment(main_hand_properties),
    };

    // let off_hand_properties = EquipmentProperties {
    //     equipment_type: EquipmentTypes::OneHandedMeleeWeapon(
    //         OneHandedMeleeWeapons::Stick,
    //         WeaponProperties {
    //             damage_classifications: vec![HpChangeSource::new(HpChangeSourceCategories::PhysicalDamage, Some(HpChangeSourceSubCategories::Blunt)],
    //             damage: Range::new(1, 4),
    //         },
    //     ),
    //     durability: Some(MaxAndCurrent::new(2, 2)),
    //     attributes: HashMap::new(),
    //     affixes: vec![],
    //     traits: None,
    // };
    // let off_hand_entity_properties = EntityProperties {
    //     id: id_generator.get_next_entity_id(),
    //     name: "Rotted Stick".to_string(),
    // };
    let off_hand_properties = EquipmentProperties {
        equipment_type: EquipmentTypes::Shield(
            Shields::MakeshiftBuckler,
            ShieldProperties {
                size: ShieldSizes::Small,
                armor_class: 5,
            },
        ),
        durability: Some(MaxAndCurrent::new(2, 2)),
        attributes: HashMap::new(),
        affixes: vec![],
        traits: None,
    };

    let off_hand_entity_properties = EntityProperties {
        id: id_generator.get_next_entity_id(),
        name: "Pot Lid".to_string(),
    };

    let off_hand = Item {
        entity_properties: off_hand_entity_properties,
        item_level: 0,
        requirements: None,
        item_properties: ItemProperties::Equipment(off_hand_properties),
    };

    let mut right_ring_properties = EquipmentProperties {
        equipment_type: EquipmentTypes::Ring,
        durability: None,
        attributes: HashMap::new(),
        affixes: vec![Affix::Prefix(PrefixTypes::Mp, 1)],
        traits: None,
    };

    right_ring_properties
        .attributes
        .insert(CombatAttributes::Mp, 2);

    let right_ring_entity_properties = EntityProperties {
        id: id_generator.get_next_entity_id(),
        name: "Rubber Band".to_string(),
    };

    let right_ring = Item {
        entity_properties: right_ring_entity_properties,
        item_level: 0,
        requirements: None,
        item_properties: ItemProperties::Equipment(right_ring_properties),
    };

    let mut left_ring_properties = EquipmentProperties {
        equipment_type: EquipmentTypes::Ring,
        durability: None,
        attributes: HashMap::new(),
        affixes: vec![Affix::Suffix(SuffixTypes::Hp, 1)],
        traits: None,
    };

    left_ring_properties
        .attributes
        .insert(CombatAttributes::Hp, 2);

    let left_ring_entity_properties = EntityProperties {
        id: id_generator.get_next_entity_id(),
        name: "Milk Carton Seal".to_string(),
    };

    let left_ring = Item {
        entity_properties: left_ring_entity_properties,
        item_level: 0,
        requirements: None,
        item_properties: ItemProperties::Equipment(left_ring_properties),
    };

    let mut amulet_properties = EquipmentProperties {
        equipment_type: EquipmentTypes::Amulet,
        durability: None,
        attributes: HashMap::new(),
        affixes: vec![Affix::Prefix(PrefixTypes::Evasion, 1)],
        traits: None,
    };

    amulet_properties
        .attributes
        .insert(CombatAttributes::Evasion, 1);

    let amulet_entity_properties = EntityProperties {
        id: id_generator.get_next_entity_id(),
        name: "Plastic Lei".to_string(),
    };

    let _amulet = Item {
        entity_properties: amulet_entity_properties,
        item_level: 0,
        requirements: None,
        item_properties: ItemProperties::Equipment(amulet_properties),
    };

    // starting_equipment.insert(EquipmentSlots::Body, chest);
    // starting_equipment.insert(EquipmentSlots::Head, head);
    // starting_equipment.insert(EquipmentSlots::MainHand, main_hand);
    // starting_equipment.insert(EquipmentSlots::OffHand, off_hand);
    // starting_equipment.insert(EquipmentSlots::RightRing, right_ring);
    // starting_equipment.insert(EquipmentSlots::LeftRing, left_ring);
    // starting_equipment.insert(EquipmentSlots::Amulet, amulet);

    starting_equipment
}
