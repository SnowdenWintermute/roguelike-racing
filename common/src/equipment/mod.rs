#![allow(dead_code)]
use rand::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::game::Game;
use crate::primatives::{EntityProperties, MaxAndCurrent};

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum StatTypes {
    Dexterity,
    Strength,
    Intelligence,
}

#[derive(Debug)]
pub enum EquipmentSlots {
    LeftHand,
    RightHand,
    Head,
    Body,
    LeftRing,
    RightRing,
    Amulet,
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq)]
pub enum ItemTypes {
    BodyArmor,
    Helmet,
    Ring,
    Amulet,
    OneHandedWeapon,
    TwoHandedWeapon,
    Shield,
    Consumable,
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq)]
pub enum ConsumableTypes {
    RoomFinder,
    RepairKit,
    UpgradeKit,
    SmokeBomb,
    MilkDrink,
    FruitDrink,
    MonsterScanner,
    Antidote,
    Grenade,
}

#[derive(Debug)]
pub struct ItemBaseStats {
    damage: u16,
    armor_class: u16,
    durability: Option<MaxAndCurrent<u16>>,
}

#[derive(Debug)]
pub struct ItemBonusStats {
    dexterity: u16,
    strength: u16,
    intelligence: u16,
}

#[derive(Debug)]
pub struct Item {
    entity_properties: EntityProperties,
    item_level: u8,
    item_type: ItemTypes,
    base_stats: Option<ItemBaseStats>,
    bonus_stats: Option<ItemBonusStats>,
    consumable_type: Option<ConsumableTypes>,
    uses_remaining: Option<u8>,
}

impl Item {
    pub fn generate(mut game: Game, level: u16) -> Item {
        let mut rng = rand::thread_rng();
        let item_types: Vec<_> = ItemTypes::iter().collect();
        let item_type = *item_types.choose(&mut rand::thread_rng()).unwrap();
        let stat_types: Vec<_> = StatTypes::iter().collect();
        let bonus_stat_type = *stat_types.choose(&mut rand::thread_rng()).unwrap();
        let mut name = String::from("");

        let mut base_stats = None;
        let mut bonus_stats = None;
        let mut consumable_type = None;
        let mut uses_remaining: Option<u8> = None;

        if item_type == ItemTypes::Consumable {
            let consumable_types: Vec<_> = ConsumableTypes::iter().collect();
            consumable_type = Some(*consumable_types.choose(&mut rand::thread_rng()).unwrap());
            uses_remaining = Some(1);
        }

        if item_type != ItemTypes::Consumable {
            let durability = rng.gen_range(1..=level) * 5;

            // DETERMINE BASE STATS
            let mut armor_class = 0;
            let mut damage = 0;
            match item_type {
                ItemTypes::Helmet | ItemTypes::Shield => armor_class = rng.gen_range(1..=level),
                ItemTypes::BodyArmor => armor_class = rng.gen_range(level..=level * 2),
                _ => (),
            };

            match item_type {
                ItemTypes::OneHandedWeapon => damage = rng.gen_range(1..=level),
                ItemTypes::TwoHandedWeapon => damage = rng.gen_range(level..=level * 2),
                _ => (),
            }

            base_stats = Some(ItemBaseStats {
                damage,
                armor_class,
                durability: Some(MaxAndCurrent {
                    max: durability,
                    current: durability,
                }),
            });

            // DETERMINE BONUS STATS
            let mut dexterity = 0;
            let mut strength = 0;
            let mut intelligence = 0;
            let mut bonus_stat_amount = 0;

            match item_type {
                ItemTypes::Consumable => (),
                ItemTypes::Ring | ItemTypes::Amulet | ItemTypes::TwoHandedWeapon => {
                    bonus_stat_amount = rng.gen_range(level..level * 2)
                }
                _ => bonus_stat_amount = rng.gen_range(1..level),
            }

            match bonus_stat_type {
                StatTypes::Dexterity => dexterity = bonus_stat_amount,
                StatTypes::Strength => strength = bonus_stat_amount,
                StatTypes::Intelligence => intelligence = bonus_stat_amount,
            }

            bonus_stats = Some(ItemBonusStats {
                dexterity,
                strength,
                intelligence,
            });
        }

        Item {
            entity_properties: EntityProperties {
                id: game.get_next_entity_id(),
                name: "name".to_owned(),
            },
            item_level: level as u8,
            base_stats,
            bonus_stats,
            item_type,
            consumable_type,
            uses_remaining,
        }
    }
}
