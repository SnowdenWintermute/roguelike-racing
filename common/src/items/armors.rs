use once_cell::sync::Lazy;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Hash, Eq, PartialEq, Copy, Clone, Debug)]
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
    OhmushellMail,
    BreastPlate,
    FieldPlate,
    GothicPlate,
    FullPlate,
    ShardPlate,
}

pub static ARMOR_LEVEL_RANGES: Lazy<HashMap<Armors, (u16, u16)>> = Lazy::new(|| {
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
            Armors::RingMail => m.insert(Armors::RingMail, (2, 5)),
            Armors::ChainMail => m.insert(Armors::ChainMail, (3, 6)),
            Armors::ScaleMail => m.insert(Armors::ScaleMail, (4, 7)),
            Armors::SplintMail => m.insert(Armors::SplintMail, (5, 9)),
            Armors::OhmushellMail => m.insert(Armors::OhmushellMail, (8, 10)),
            Armors::BreastPlate => m.insert(Armors::BreastPlate, (2, 4)),
            Armors::FieldPlate => m.insert(Armors::FieldPlate, (3, 6)),
            Armors::GothicPlate => m.insert(Armors::GothicPlate, (5, 8)),
            Armors::FullPlate => m.insert(Armors::FullPlate, (6, 8)),
            Armors::ShardPlate => m.insert(Armors::ShardPlate, (8, 10)),
        };
        i += 1;
    }
    m
});

pub static ARMOR_BY_LEVEL: Lazy<HashMap<u16, Vec<Armors>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    let armor_level_ranges: Vec<(&Armors, &(u16, u16))> = ARMOR_LEVEL_RANGES.iter().collect();
    let mut i = 1;
    while i <= 10 {
        let mut v = Vec::new();
        let mut j = 1;
        while j < armor_level_ranges.len() {
            let armor_and_range = armor_level_ranges[j];
            if i >= armor_and_range.1 .0 && i <= armor_and_range.1 .1 {
                let cloned_armor = armor_and_range.0.clone();
                v.push(cloned_armor);
            }
            j += 1;
        }
        m.insert(i, v);
        i += 1;
    }
    m
});
