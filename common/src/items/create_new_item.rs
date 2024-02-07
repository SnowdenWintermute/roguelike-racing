use super::equipment::EquipmentProperties;
use super::Item;
use super::ItemProperties;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::primatives::EntityProperties;
use std::collections::HashMap;

impl Item {
    pub fn new(
        id: u32,
        name: String,
        item_level: u8,
        requirements: Option<HashMap<CombatAttributes, u8>>,
        item_properties: ItemProperties,
    ) -> Item {
        Item {
            entity_properties: EntityProperties { id, name },
            item_level,
            requirements,
            item_properties,
        }
    }
}
// ItemProperties::Equipment(EquipmentProperties {
//                 equipment_type: EquipmentTypes::OneHandedMeleeWeapon(
//                     OneHandedMeleeWeapons::Stick,
//                     WeaponProperties {
//                         damage_classifications: vec![HpChangeSource {
//                             category: HpChangeSourceCategories::PhysicalDamage,
//                             sub_category: None,
//                             element: None,
//                         }],
//                         damage: Range::new(10, 100),
//                     },
//                 ),
//                 durability: None,
//                 attributes: HashMap::new(),
//                 affixes: vec![],
//                 traits: Some(vec![EquipmentTraits::DamagePercentage(50)]),
//             }),
//         }
