use crate::items::affixes::{PrefixTypes, SuffixTypes};
use once_cell::sync::Lazy;
use strum::IntoEnumIterator;

pub static BODY_ARMOR_POSSIBLE_PREFIXES_AND_TIERS: Lazy<Vec<(PrefixTypes, u8)>> = Lazy::new(|| {
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

pub static BODY_ARMOR_POSSIBLE_SUFFIXES_AND_TIERS: Lazy<Vec<(SuffixTypes, u8)>> = Lazy::new(|| {
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
            SuffixTypes::Focus => (),
            SuffixTypes::Damage => (),
            SuffixTypes::Durability => possible_suffixes_and_tiers.push((suffix_type, 5)),
        }
        i += 1;
    }

    possible_suffixes_and_tiers
});
