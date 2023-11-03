use crate::{app_consts::DEEPEST_FLOOR, combatants::CombatAttributes, primatives::Range};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::affixes::{PrefixTypes, SuffixTypes};

#[derive(Serialize, Deserialize, EnumIter, Hash, Eq, PartialEq, Copy, Clone, Debug, Default)]
pub enum ArmorCategories {
    #[default]
    Cloth,
    Leather,
    Mail,
    Plate,
}

#[derive(Serialize, Deserialize, Default, EnumIter, Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Armors {
    #[default]
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

pub struct ArmorGenerationTemplate {
    pub level_range: Range<u8>,
    pub category: ArmorCategories,
    pub max_durability: u8,
    pub requirements: HashMap<CombatAttributes, u16>,
}

impl ArmorGenerationTemplate {
    pub fn new(
        min_level: u8,
        max_level: u8,
        max_durability: u8,
        category: ArmorCategories,
        requirements: HashMap<CombatAttributes, u16>,
    ) -> ArmorGenerationTemplate {
        ArmorGenerationTemplate {
            level_range: Range::new(min_level, max_level),
            max_durability,
            category,
            requirements,
        }
    }
}

pub static ARMOR_GENERATION_TEMPLATES: Lazy<HashMap<Armors, ArmorGenerationTemplate>> =
    Lazy::new(|| {
        let mut m = HashMap::new();
        let armors: Vec<Armors> = Armors::iter().collect();
        let mut i = 0;
        while i < armors.len() {
            let armor = armors[i];
            let mut requirements = HashMap::new();
            match armor {
                Armors::Rags => {
                    let mut requirements = HashMap::new();
                    m.insert(
                        Armors::Rags,
                        ArmorGenerationTemplate::new(1, 3, 6, ArmorCategories::Cloth, requirements),
                    )
                }
                Armors::Cape => m.insert(
                    Armors::Cape,
                    ArmorGenerationTemplate::new(1, 4, 12, ArmorCategories::Cloth, requirements),
                ),
                Armors::Cloak => m.insert(
                    Armors::Cloak,
                    ArmorGenerationTemplate::new(3, 7, 18, ArmorCategories::Cloth, requirements),
                ),
                Armors::Robe => {
                    requirements.insert(CombatAttributes::Intelligence, 5);
                    m.insert(
                        Armors::Robe,
                        ArmorGenerationTemplate::new(
                            6,
                            9,
                            24,
                            ArmorCategories::Cloth,
                            requirements,
                        ),
                    )
                }
                Armors::Kevlar => {
                    requirements.insert(CombatAttributes::Intelligence, 10);
                    m.insert(
                        Armors::Kevlar,
                        ArmorGenerationTemplate::new(
                            8,
                            10,
                            30,
                            ArmorCategories::Cloth,
                            requirements,
                        ),
                    )
                }
                Armors::LeatherArmor => {
                    requirements.insert(CombatAttributes::Dexterity, 3);
                    m.insert(
                        Armors::LeatherArmor,
                        ArmorGenerationTemplate::new(
                            1,
                            5,
                            8,
                            ArmorCategories::Leather,
                            requirements,
                        ),
                    )
                }
                Armors::HardLeatherArmor => {
                    requirements.insert(CombatAttributes::Dexterity, 5);

                    m.insert(
                        Armors::HardLeatherArmor,
                        ArmorGenerationTemplate::new(
                            3,
                            7,
                            15,
                            ArmorCategories::Leather,
                            requirements,
                        ),
                    )
                }
                Armors::StuddedLeatherArmor => {
                    requirements.insert(CombatAttributes::Dexterity, 7);
                    m.insert(
                        Armors::StuddedLeatherArmor,
                        ArmorGenerationTemplate::new(
                            4,
                            10,
                            24,
                            ArmorCategories::Leather,
                            requirements,
                        ),
                    )
                }
                Armors::DemonsaurArmor => {
                    requirements.insert(CombatAttributes::Dexterity, 15);
                    m.insert(
                        Armors::DemonsaurArmor,
                        ArmorGenerationTemplate::new(
                            8,
                            10,
                            40,
                            ArmorCategories::Leather,
                            requirements,
                        ),
                    )
                }
                Armors::RingMail => {
                    requirements.insert(CombatAttributes::Strength, 3);
                    requirements.insert(CombatAttributes::Dexterity, 3);
                    m.insert(
                        Armors::RingMail,
                        ArmorGenerationTemplate::new(2, 5, 12, ArmorCategories::Mail, requirements),
                    )
                }
                Armors::ChainMail => {
                    requirements.insert(CombatAttributes::Strength, 3);
                    requirements.insert(CombatAttributes::Dexterity, 3);
                    requirements.insert(CombatAttributes::Intelligence, 3);
                    m.insert(
                        Armors::ChainMail,
                        ArmorGenerationTemplate::new(3, 6, 18, ArmorCategories::Mail, requirements),
                    )
                }
                Armors::ScaleMail => {
                    requirements.insert(CombatAttributes::Dexterity, 5);
                    requirements.insert(CombatAttributes::Intelligence, 5);
                    m.insert(
                        Armors::ScaleMail,
                        ArmorGenerationTemplate::new(4, 7, 24, ArmorCategories::Mail, requirements),
                    )
                }
                Armors::SplintMail => {
                    requirements.insert(CombatAttributes::Strength, 7);
                    requirements.insert(CombatAttributes::Intelligence, 7);
                    m.insert(
                        Armors::SplintMail,
                        ArmorGenerationTemplate::new(5, 9, 30, ArmorCategories::Mail, requirements),
                    )
                }
                Armors::OhmushellMail => {
                    requirements.insert(CombatAttributes::Strength, 10);
                    requirements.insert(CombatAttributes::Dexterity, 10);
                    requirements.insert(CombatAttributes::Intelligence, 10);
                    m.insert(
                        Armors::OhmushellMail,
                        ArmorGenerationTemplate::new(
                            8,
                            10,
                            50,
                            ArmorCategories::Mail,
                            requirements,
                        ),
                    )
                }
                Armors::BreastPlate => {
                    requirements.insert(CombatAttributes::Strength, 5);
                    m.insert(
                        Armors::BreastPlate,
                        ArmorGenerationTemplate::new(
                            2,
                            4,
                            18,
                            ArmorCategories::Plate,
                            requirements,
                        ),
                    )
                }
                Armors::FieldPlate => {
                    requirements.insert(CombatAttributes::Strength, 7);
                    m.insert(
                        Armors::FieldPlate,
                        ArmorGenerationTemplate::new(
                            3,
                            6,
                            24,
                            ArmorCategories::Plate,
                            requirements,
                        ),
                    )
                }
                Armors::GothicPlate => {
                    requirements.insert(CombatAttributes::Strength, 12);
                    m.insert(
                        Armors::GothicPlate,
                        ArmorGenerationTemplate::new(
                            5,
                            8,
                            30,
                            ArmorCategories::Plate,
                            requirements,
                        ),
                    )
                }
                Armors::FullPlate => {
                    requirements.insert(CombatAttributes::Strength, 15);
                    m.insert(
                        Armors::FullPlate,
                        ArmorGenerationTemplate::new(
                            6,
                            8,
                            50,
                            ArmorCategories::Plate,
                            requirements,
                        ),
                    )
                }
                Armors::ShardPlate => {
                    requirements.insert(CombatAttributes::Strength, 20);
                    m.insert(
                        Armors::ShardPlate,
                        ArmorGenerationTemplate::new(
                            8,
                            10,
                            80,
                            ArmorCategories::Plate,
                            requirements,
                        ),
                    )
                }
            };
            i += 1;
        }
        m
    });

pub static ARMOR_BY_LEVEL: Lazy<HashMap<u8, Vec<Armors>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    let templates: Vec<(&Armors, &ArmorGenerationTemplate)> =
        ARMOR_GENERATION_TEMPLATES.iter().collect();
    let mut i = 1;
    while i <= DEEPEST_FLOOR {
        let mut v = Vec::new();
        let mut j = 1;
        while j < templates.len() {
            let template = templates[j];
            if i >= template.1.level_range.min && i <= template.1.level_range.max {
                let cloned_armor = template.0.clone();
                v.push(cloned_armor);
            }
            j += 1;
        }
        m.insert(i, v);
        i += 1;
    }
    m
});

pub static ARMOR_POSSIBLE_PREFIXES_AND_TIERS: Lazy<Vec<(PrefixTypes, u8)>> = Lazy::new(|| {
    let all_prefix_types: Vec<PrefixTypes> = PrefixTypes::iter().collect();
    let mut possible_prefixes_and_tiers: Vec<(PrefixTypes, u8)> = vec![];
    let mut i = 0;
    while i < all_prefix_types.len() {
        let prefix_type = all_prefix_types[i].clone();
        match prefix_type {
            PrefixTypes::Mp => possible_prefixes_and_tiers.push((prefix_type, 4)),
            PrefixTypes::ArmorClass => possible_prefixes_and_tiers.push((prefix_type, 5)),
            PrefixTypes::Accuracy => (),
            PrefixTypes::PercentDamage => (),
            PrefixTypes::LifeSteal => (),
            PrefixTypes::Resilience => possible_prefixes_and_tiers.push((prefix_type, 5)),
            PrefixTypes::Evasion => possible_prefixes_and_tiers.push((prefix_type, 5)),
            PrefixTypes::Obscurity => possible_prefixes_and_tiers.push((prefix_type, 5)),
            PrefixTypes::ArmorPenetration => (),
        }
        i += 1;
    }

    possible_prefixes_and_tiers
});

pub static ARMOR_POSSIBLE_SUFFIXES_AND_TIERS: Lazy<Vec<(SuffixTypes, u8)>> = Lazy::new(|| {
    let all_suffix_types: Vec<SuffixTypes> = SuffixTypes::iter().collect();
    let mut possible_suffixes_and_tiers: Vec<(SuffixTypes, u8)> = vec![];
    let mut i = 0;
    while i < all_suffix_types.len() {
        let suffix_type = all_suffix_types[i].clone();
        match suffix_type {
            SuffixTypes::Strength => possible_suffixes_and_tiers.push((suffix_type, 4)),
            SuffixTypes::Intelligence => possible_suffixes_and_tiers.push((suffix_type, 4)),
            SuffixTypes::Dexterity => possible_suffixes_and_tiers.push((suffix_type, 4)),
            SuffixTypes::Vitality => possible_suffixes_and_tiers.push((suffix_type, 5)),
            SuffixTypes::AllBase => possible_suffixes_and_tiers.push((suffix_type, 3)),
            SuffixTypes::Hp => possible_suffixes_and_tiers.push((suffix_type, 5)),
            SuffixTypes::Focus => todo!(),
            SuffixTypes::Damage => todo!(),
            SuffixTypes::Durability => possible_suffixes_and_tiers.push((suffix_type, 5)),
        }
        i += 1;
    }

    possible_suffixes_and_tiers
});
