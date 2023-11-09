use crate::items::equipment::affixes::PrefixTypes;
use crate::items::equipment::affixes::SuffixTypes;
use once_cell::sync::Lazy;
use strum::IntoEnumIterator;

use super::vec_of_possible_affixes_and_tiers_from_filter::vec_of_possible_affixes_and_tiers_from_filter;

fn tier_if_prefix_allowed(prefix_type: &PrefixTypes) -> Option<u8> {
    match prefix_type {
        PrefixTypes::Mp => Some(5),
        PrefixTypes::ArmorClass => Some(5),
        PrefixTypes::Accuracy => Some(5),
        PrefixTypes::PercentDamage => None,
        PrefixTypes::LifeSteal => None,
        PrefixTypes::Resilience => Some(5),
        PrefixTypes::Evasion => Some(5),
        PrefixTypes::Obscurity => Some(5),
        PrefixTypes::ArmorPenetration => None,
    }
}
fn tier_if_suffix_allowed(suffix_type: &SuffixTypes) -> Option<u8> {
    match suffix_type {
        SuffixTypes::Strength => Some(5),
        SuffixTypes::Intelligence => Some(5),
        SuffixTypes::Dexterity => Some(5),
        SuffixTypes::Vitality => Some(5),
        SuffixTypes::AllBase => Some(5),
        SuffixTypes::Hp => Some(5),
        SuffixTypes::Focus => Some(5),
        SuffixTypes::Damage => None,
        SuffixTypes::Durability => None,
    }
}

pub static JEWELRY_POSSIBLE_PREFIXES_AND_TIERS: Lazy<Vec<(PrefixTypes, u8)>> = Lazy::new(|| {
    let all_prefix_types: Vec<PrefixTypes> = PrefixTypes::iter().collect();
    vec_of_possible_affixes_and_tiers_from_filter(all_prefix_types, tier_if_prefix_allowed)
});

pub static JEWELRY_POSSIBLE_SUFFIXES_AND_TIERS: Lazy<Vec<(SuffixTypes, u8)>> = Lazy::new(|| {
    let all_suffix_types: Vec<SuffixTypes> = SuffixTypes::iter().collect();
    vec_of_possible_affixes_and_tiers_from_filter(all_suffix_types, tier_if_suffix_allowed)
});
