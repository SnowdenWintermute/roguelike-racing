use core::fmt;
use serde::Deserialize;
use serde::Serialize;
use strum_macros::EnumIter;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Affix {
    Prefix(PrefixTypes, u8),
    Suffix(SuffixTypes, u8),
}

impl fmt::Display for Affix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Affix::Prefix(prefix_type, tier) => write!(f, "{prefix_type} tier {tier}",),
            Affix::Suffix(suffix_type, tier) => write!(f, "{suffix_type} tier {tier}",),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, EnumIter, PartialEq, Eq, Clone, Hash)]
pub enum PrefixTypes {
    Mp,
    ArmorClass,
    Accuracy,
    PercentDamage,
    LifeSteal,
    Resilience,
    Evasion,
    Obscurity,
    ArmorPenetration,
}

impl fmt::Display for PrefixTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PrefixTypes::Mp => write!(f, "Mp"),
            PrefixTypes::ArmorClass => write!(f, "Armor Class"),
            PrefixTypes::Accuracy => write!(f, "Accuracy"),
            PrefixTypes::PercentDamage => write!(f, "Percent Damage"),
            PrefixTypes::LifeSteal => write!(f, "Life Steal"),
            PrefixTypes::Resilience => write!(f, "Resilience"),
            PrefixTypes::Evasion => write!(f, "Evasion"),
            PrefixTypes::Obscurity => write!(f, "Obscurity"),
            PrefixTypes::ArmorPenetration => write!(f, "Armor Pen."),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, EnumIter, PartialEq, Eq, Clone, Hash)]
pub enum SuffixTypes {
    Strength,
    Intelligence,
    Dexterity,
    Vitality,
    AllBase,
    Hp,
    Focus,
    Damage,
    Durability,
}

impl fmt::Display for SuffixTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SuffixTypes::Strength => write!(f, "Strength"),
            SuffixTypes::Intelligence => write!(f, "Intelligence"),
            SuffixTypes::Dexterity => write!(f, "Dexterity"),
            SuffixTypes::Vitality => write!(f, "Vitality"),
            SuffixTypes::AllBase => write!(f, "All Base Attributes"),
            SuffixTypes::Hp => write!(f, "Hp"),
            SuffixTypes::Focus => write!(f, "Focus"),
            SuffixTypes::Damage => write!(f, "Damage"),
            SuffixTypes::Durability => write!(f, "Durability"),
        }
    }
}
