#![allow(dead_code)]
use rand::{distributions::Standard, prelude::*};
use std::{fmt::Arguments, process::Output};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::scratch_paper::{Character, EntityProperties, Game, MaxAndCurrent};

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum StatTypes {
    Dexterity,
    Strength,
    Intelligence,
}

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

pub struct ItemBaseStats {
    damage: u16,
    armor_class: u16,
    durability: Option<MaxAndCurrent<u16>>,
}

pub struct ItemBonusStats {
    dexterity: u16,
    strength: u16,
    intelligence: u16,
}

pub struct Item {
    entity_properties: EntityProperties,
    item_type: ItemTypes,
    base_stats: Option<ItemBaseStats>,
    bonus_stats: Option<ItemBonusStats>,
    uses_remaining: Option<u8>,
    use_item: Option<Box<dyn Fn(Self) -> ()>>,
}

impl Item {
    fn generate(&self, game: Game, level: u16) -> Item {
        let mut rng = rand::thread_rng();
        let item_types: Vec<_> = ItemTypes::iter().collect();
        let item_type = *item_types.choose(&mut rand::thread_rng()).unwrap();
        let stat_types: Vec<_> = StatTypes::iter().collect();
        let bonus_stat_type = *stat_types.choose(&mut rand::thread_rng()).unwrap();
        let mut name = String::from("");

        let mut base_stats = None;
        let mut bonus_stats = None;
        let mut uses_remaining: Option<u8> = None;
        let mut use_item: Option<Box<dyn Fn(Item) -> ()>> = None;

        if item_type == ItemTypes::Consumable {
            //
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
            base_stats,
            bonus_stats,
            item_type,
            uses_remaining: None,
            use_item: Some(Box::new(|item| {
                let random_thing = "lolol";
                match item.base_stats {
                    Some(base_stats) => {
                        println!("item damage: {}, {}", base_stats.damage, random_thing)
                    }
                    None => (),
                }
            })),
        }
    }
}
