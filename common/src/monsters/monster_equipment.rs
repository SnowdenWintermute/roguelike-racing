use super::monster_types::MonsterTypes;
use crate::game::id_generator::IdGenerator;
use crate::items::equipment::pre_made_items::PreMadeItems;
use crate::items::equipment::EquipmentSlots;
use crate::items::Item;
use std::collections::HashMap;

impl MonsterTypes {
    pub fn get_equipment(&self, id_generator: &mut IdGenerator) -> HashMap<EquipmentSlots, Item> {
        match self {
            MonsterTypes::SkeletonArcher => HashMap::from([(
                EquipmentSlots::MainHand,
                PreMadeItems::SkeletonArcherShortBow.get_item(id_generator),
            )]),
            MonsterTypes::Scavenger | MonsterTypes::Vulture => HashMap::from([
                (
                    EquipmentSlots::MainHand,
                    PreMadeItems::AnimalClaw.get_item(id_generator),
                ),
                (
                    EquipmentSlots::OffHand,
                    PreMadeItems::AnimalClaw.get_item(id_generator),
                ),
            ]),

            MonsterTypes::Zombie => HashMap::from([
                (
                    EquipmentSlots::MainHand,
                    PreMadeItems::Fist.get_item(id_generator),
                ),
                (
                    EquipmentSlots::OffHand,
                    PreMadeItems::Fist.get_item(id_generator),
                ),
            ]),
            MonsterTypes::MetallicGolem => HashMap::from([
                (
                    EquipmentSlots::MainHand,
                    PreMadeItems::Stab.get_item(id_generator),
                ),
                (
                    EquipmentSlots::OffHand,
                    PreMadeItems::Fist.get_item(id_generator),
                ),
            ]),
            _ => HashMap::new(),
        }
    }
}
