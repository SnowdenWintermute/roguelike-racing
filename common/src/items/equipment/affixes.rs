use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;
use strum_macros::EnumIter;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Affix {
    Prefix(PrefixTypes, u8),
    Suffix(SuffixTypes, u8),
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
