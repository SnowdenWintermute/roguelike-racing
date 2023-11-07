use crate::items::equipment::affixes::{PrefixTypes, SuffixTypes};
use once_cell::sync::Lazy;
use strum::IntoEnumIterator;

pub static BODY_ARMOR_POSSIBLE_PREFIXES_AND_TIERS: Lazy<Vec<(PrefixTypes, u8)>> = Lazy::new(|| {
    let all_prefix_types: Vec<PrefixTypes> = PrefixTypes::iter().collect();
    let mut possible_prefixes_and_tiers: Vec<(PrefixTypes, u8)> = vec![];
    let mut i = 0;
    while i < all_prefix_types.len() {
        let prefix_type = all_prefix_types[i].clone();
        let max_tier_option = match prefix_type {
            PrefixTypes::Mp => Some(4),
            PrefixTypes::ArmorClass => Some(5),
            PrefixTypes::Accuracy => None,
            PrefixTypes::PercentDamage => None,
            PrefixTypes::LifeSteal => None,
            PrefixTypes::Resilience => Some(5),
            PrefixTypes::Evasion => Some(5),
            PrefixTypes::Obscurity => Some(5),
            PrefixTypes::ArmorPenetration => None,
        };
        if let Some(max_tier) = max_tier_option {
            possible_prefixes_and_tiers.push((prefix_type, max_tier))
        }
        i += 1;
    }

    possible_prefixes_and_tiers
});

pub static BODY_ARMOR_POSSIBLE_SUFFIXES_AND_TIERS: Lazy<Vec<(SuffixTypes, u8)>> = Lazy::new(|| {
    let all_suffix_types: Vec<SuffixTypes> = SuffixTypes::iter().collect();
    let mut possible_suffixes_and_tiers: Vec<(SuffixTypes, u8)> = vec![];
    let mut i = 0;
    while i < all_suffix_types.len() {
        let suffix_type = all_suffix_types[i].clone();
        let max_tier_option = match suffix_type {
            SuffixTypes::Strength => Some(4),
            SuffixTypes::Intelligence => Some(4),
            SuffixTypes::Dexterity => Some(4),
            SuffixTypes::Vitality => Some(5),
            SuffixTypes::AllBase => Some(3),
            SuffixTypes::Hp => Some(5),
            SuffixTypes::Focus => None,
            SuffixTypes::Damage => None,
            SuffixTypes::Durability => Some(5),
        };
        if let Some(max_tier) = max_tier_option {
            possible_suffixes_and_tiers.push((suffix_type, max_tier))
        }
        i += 1;
    }

    possible_suffixes_and_tiers
});
