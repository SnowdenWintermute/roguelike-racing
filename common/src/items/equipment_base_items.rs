use super::Item;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use rand::prelude::*;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

static GLOBAL_DATA: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    m
});

#[derive(EnumIter)]
pub enum EquipmentCategories {
    Armor,
    Jewelry,
    MeleeWeapon,
    RangedWeapon,
    Shield,
}

impl Item {
    pub fn generate_base_item(level: u16) {
        let mut rng = rand::thread_rng();
        let categories: Vec<EquipmentCategories> = EquipmentCategories::iter().collect();
        let category = categories.choose(&mut rand::thread_rng()).unwrap();
        match category {
            EquipmentCategories::Armor => {
                let possible_base_armors = ARMOR_BY_LEVEL.get(&level);
            }
            EquipmentCategories::Jewelry => todo!(),
            EquipmentCategories::MeleeWeapon => todo!(),
            EquipmentCategories::RangedWeapon => todo!(),
            EquipmentCategories::Shield => todo!(),
        }
    }
}

static ARMOR_LEVEL_RANGES: Lazy<HashMap<Armors, (u16, u16)>> = Lazy::new(|| {
    let mut m = HashMap::new();
    let armors: Vec<Armors> = Armors::iter().collect();
    let mut i = 0;
    while i < armors.len() {
        let armor = armors[i];
        match armor {
            Armors::Rags => m.insert(Armors::Rags, (1, 3)),
            Armors::Cape => m.insert(Armors::Cape, (1, 4)),
            Armors::Cloak => m.insert(Armors::Cloak, (3, 7)),
            Armors::Robe => m.insert(Armors::Robe, (6, 9)),
            Armors::Kevlar => m.insert(Armors::Kevlar, (8, 10)),
            Armors::LeatherArmor => m.insert(Armors::LeatherArmor, (1, 5)),
            Armors::HardLeatherArmor => m.insert(Armors::HardLeatherArmor, (3, 7)),
            Armors::StuddedLeatherArmor => m.insert(Armors::StuddedLeatherArmor, (4, 10)),
            Armors::DemonsaurArmor => m.insert(Armors::DemonsaurArmor, (8, 10)),
            Armors::RingMail => m.insert(Armors::RingMail, (1, 1)),
            Armors::ChainMail => m.insert(Armors::ChainMail, (1, 1)),
            Armors::ScaleMail => m.insert(Armors::ScaleMail, (1, 1)),
            Armors::SplintMail => m.insert(Armors::SplintMail, (1, 1)),
            Armors::Ohmushell => m.insert(Armors::Ohmushell, (1, 1)),
            Armors::BreastPlate => m.insert(Armors::BreastPlate, (1, 1)),
            Armors::FieldPlate => m.insert(Armors::FieldPlate, (1, 1)),
            Armors::GothicPlate => m.insert(Armors::GothicPlate, (1, 1)),
            Armors::FullPlate => m.insert(Armors::FullPlate, (1, 1)),
            Armors::ShardPlate => m.insert(Armors::ShardPlate, (1, 1)),
        };
        i += 1;
    }

    m
});

static ARMOR_BY_LEVEL: Lazy<HashMap<u16, Vec<Armors>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    let armor_level_ranges: Vec<(&Armors, &(u16, u16))> = ARMOR_LEVEL_RANGES.iter().collect();
    let mut i = 0;
    while i <= 10 {
        let mut v = Vec::new();
        let mut j = 0;
        while j < armor_level_ranges.len() {
            let armor_and_range = armor_level_ranges[j];
            if i >= armor_and_range.1 .0 && i <= armor_and_range.1 .1 {
                let cloned_armor = armor_and_range.0.clone();
                v.push(cloned_armor);
            }
        }
        m.insert(i, v);
        i += 1;
    }

    m
});

// const static EQUIPMENT_BY_LEVEL: HashMap<u16, > :
#[derive(EnumIter, Hash, Eq, PartialEq, Copy, Clone)]
pub enum Armors {
    Rags,
    Cape,
    Cloak,
    Robe,
    Kevlar,
    LeatherArmor,
    HardLeatherArmor,
    StuddedLeatherArmor,
    DemonsaurArmor,
    RingMail,
    ChainMail,
    ScaleMail,
    SplintMail,
    Ohmushell,
    BreastPlate,
    FieldPlate,
    GothicPlate,
    FullPlate,
    ShardPlate,
}

pub enum Weapons {
    Club,
    Mace,
    Morningstar,
    WarHammer,
    Stick,
    Staff,
    Maul,
    ShortSword,
    Sabre,
    Blade,
    BroadSword,
    BastardSword,
    TwoHandedSword,
    Katana,
    GreatAxe,
    Spear,
    Pike,
    ShortBow,
    HuntersBow,
    LongBow,
    CompositeBow,
    WarBow,
}
